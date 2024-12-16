use clap::Parser;
use halo2_base::{
    gates::circuit::builder::BaseCircuitBuilder,
    utils::{ScalarField, BigPrimeField},
    AssignedValue,
};
use halo2_graph::scaffold::{cmd::Cli, run};
use halo2_base::gates::RangeInstructions;
use halo2_graph::gadget::fixed_point::{FixedPointChip, FixedPointInstructions};
use halton::Sequence; // Import Halton sequence generator
use std::str::FromStr;
use serde::{Serialize, Deserialize};

// Define the circuit input structure
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CircuitInput {
    pub num_points: String, // Define num_points as a string type
}

// Use quasi-Monte Carlo method to estimate the value of π
fn quasi_monte_carlo_pi<F: ScalarField>(
    builder: &mut BaseCircuitBuilder<F>,
    input: CircuitInput,
    make_public: &mut Vec<AssignedValue<F>>,
) where F: BigPrimeField {
    const PRECISION: u32 = 63;
    println!("build_lookup_bit: {:?}", builder.lookup_bits());
    let range_chip = builder.range_chip();
    let fixed_point_chip = FixedPointChip::<F, PRECISION>::default(builder);

    // Get the main context of the circuit
    let ctx = builder.main(0);

    // Convert the string to u64
    let num_points: u64 = u64::from_str(&input.num_points).expect("Invalid number format for num_points");
    assert!(num_points > 0, "num_points must be greater than zero"); // Ensure num_points is greater than zero
    println!("Number of points: {}", num_points); // Debug information

    let num_points_const = ctx.load_witness(fixed_point_chip.quantization(num_points as f64));
    make_public.push(num_points_const);

    // Calculate the reciprocal of num_points (computed outside the circuit)
    let num_points_inv_value = fixed_point_chip.quantization(1.0 / (num_points as f64));
    let num_points_inv = ctx.load_witness(num_points_inv_value);

    let zero = ctx.load_witness(fixed_point_chip.quantization(0.0));
    let one = ctx.load_witness(fixed_point_chip.quantization(1.0));
    let four = ctx.load_witness(fixed_point_chip.quantization(4.0));
    let mut in_circle = zero;

    let threshold = ctx.load_witness(fixed_point_chip.quantization(1.0));

    // Create Halton sequence generators
    let halton_x = Sequence::new(2); // Halton sequence with base 2
    let halton_y = Sequence::new(3); // Halton sequence with base 3

    // Generate num_points points using Halton sequences
    for (i, (x_val, y_val)) in halton_x.zip(halton_y).take(num_points as usize).enumerate() {
        let x_field = fixed_point_chip.quantization(x_val);
        let y_field = fixed_point_chip.quantization(y_val);
        
        let x = ctx.load_witness(x_field);
        let y = ctx.load_witness(y_field);

        // Calculate x^2 + y^2
        let x2 = fixed_point_chip.qmul(ctx, x, x);
        let y2 = fixed_point_chip.qmul(ctx, y, y);
        let sum = fixed_point_chip.qadd(ctx, x2, y2);
        
        let is_in_circle: AssignedValue<F> = range_chip.is_less_than(ctx, sum, threshold, 64);
        if is_in_circle.value().is_zero().into() {
            in_circle = fixed_point_chip.qadd(ctx, in_circle, zero);
        } else {
            in_circle = fixed_point_chip.qadd(ctx, in_circle, one);
        }

        // Detailed debug information
        println!("Point {}: x = {:?}, y = {:?}", i, fixed_point_chip.dequantization(*x.value()), fixed_point_chip.dequantization(*y.value()));
        println!("x^2 = {:?}, y^2 = {:?}", fixed_point_chip.dequantization(*x2.value()), fixed_point_chip.dequantization(*y2.value()));
        println!("x^2 + y^2 = {:?}", fixed_point_chip.dequantization(*sum.value()));
        println!("threshold: {:?}", fixed_point_chip.dequantization(*threshold.value()));
    }

    println!("In circle count: {:?}", fixed_point_chip.dequantization(*in_circle.value())); // Debug information

    let ratio = fixed_point_chip.qmul(ctx, in_circle, num_points_inv);

    println!("Ratio: {:?}", fixed_point_chip.dequantization(*ratio.value())); // Debug information

    // Estimate the value of π
    let pi_estimate = fixed_point_chip.qmul(ctx, ratio, four);
    // Set the estimated value of π as public
    make_public.push(pi_estimate);
    make_public.push(in_circle);
    make_public.push(ratio);

    println!("Estimated π: {:?}", fixed_point_chip.dequantization(*pi_estimate.value())); // Debug information
}

// Main function, the entry point of the program
fn main() {
    env_logger::init();

    let args = Cli::parse();
    // Run the quasi-Monte Carlo π estimation circuit
    run(quasi_monte_carlo_pi, args);
}
