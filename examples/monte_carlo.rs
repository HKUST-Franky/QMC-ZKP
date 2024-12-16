use clap::Parser;
use halo2_base::{
    gates::circuit::builder::BaseCircuitBuilder,
    utils::ScalarField,
    AssignedValue,
};
use halo2_graph::scaffold::{cmd::Cli, run};
use serde::{Deserialize, Serialize};
use rand::Rng;
use std::str::FromStr;

/// Defines a struct to represent the circuit input
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CircuitInput {
    pub num_points: String, // Define num_points as a string type
}

/// Original Monte Carlo method to estimate the value of π
///
/// # Arguments
///
/// * `builder` - A mutable reference to the circuit builder
/// * `input` - The circuit input
/// * `make_public` - A mutable reference to a vector of assigned values to be made public
fn monte_carlo_pi<F: ScalarField>(
    builder: &mut BaseCircuitBuilder<F>,
    input: CircuitInput,
    make_public: &mut Vec<AssignedValue<F>>,
) {
    // Get the main context of the circuit
    let ctx = builder.main(0);

    // Create a random number generator
    let mut rng = rand::thread_rng();
    let mut in_circle = 0;

    // Convert the string to u32
    let num_points: u32 = u32::from_str(&input.num_points).expect("Invalid number format for num_points");

    // Perform num_points random point throws
    for _ in 0..num_points {
        let x: f64 = rng.gen(); // Generate a random number in [0, 1)
        let y: f64 = rng.gen(); // Generate a random number in [0, 1)
        // Check if the point (x, y) is inside the unit circle
        if x * x + y * y <= 1.0 {
            in_circle += 1; // If inside the circle, increment the count
        }
    }

    // Estimate the value of π
    let pi_estimate = 4.0 * (in_circle as f64) / (num_points as f64);
    // Scale the estimated value of π and convert to integer
    let pi_estimate_int = (pi_estimate * 1_000_000.0) as u64;

    // Convert the integer to a scalar field element
    let pi_estimate_fp = F::from(pi_estimate_int);

    // Load the estimated value of π as a witness value in the circuit
    let pi_assigned = ctx.load_witness(pi_estimate_fp);
    // Set the estimated value of π as public
    make_public.push(pi_assigned);

    // Print the estimated value of π
    println!("Estimated π: {:.6}", pi_estimate);
}

/// Main function, the entry point of the program
fn main() {
    // Initialize the logger
    env_logger::init();

    // Parse command line arguments
    let args = Cli::parse();
    // Run the Monte Carlo π estimation circuit
    run(monte_carlo_pi, args);
}
