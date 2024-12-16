# Privacy-enhancing Financial Data Analytics Tool Development

This repository is dedicated to implementing the Quasi-Monte Carlo method within a Zero-Knowledge Proof framework. It also includes the original Monte Carlo algorithm for conducting ablation experiments.

## Setup

Install Rust:

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

Clone this repo:

```bash
git clone https://github.com/HKUST-Franky/QMC-ZKP.git

cd QMC-ZKP
```

## Quick Start with `Quasi-Monte-Carlo`

You can simply run the bash script for all four stages, including the mock prover, key generation, proof generation, and proof verification:

```bash
bash QMC.sh
```

Alternatively, you can run each stage separately.

### Mock Prover

Run the mock prover using:

```bash
LOOKUP_BITS=<BITS> cargo run --example quasi_monte_carlo -- --name quasi_monte_carlo -k <DEGREE> mock # for example, DEGREE=8 or 16, BITS = DEGREE - 1
```

where `--name` can be used to specify the circuit. By default, the program will try to read the input as a JSON from [`data/quasi_monte_carlo.in`](data/quasi_monte_carlo.in). A different input path can be specified with the option `--input filename.in`, which is expected to be located at `data/filename.in`.

The `MockProver` does not run the cryptographic prover on your circuit but instead directly checks if constraints are satisfied. This is useful for testing purposes and runs faster than the actual prover.

Here `DEGREE` is a variable you specify to set the circuit to have `2^DEGREE` number of rows. The halo2-lib API will automatically allocate columns for the optimal circuit that fits within the specified number of rows. See [here](https://docs.axiom.xyz/zero-knowledge-proofs/getting-started-with-halo2#cost-modeling) for a discussion of how to think about the row vs. column tradeoff in a Halo2 circuit. _Note:_ The last ~9 rows of a circuit are reserved for the proof system (blinding factors to ensure zero-knowledge).

If you want to see the statistics for what is actually being auto-configured in the circuit, you can run:

```bash
RUST_LOG=info LOOKUP_BITS=<BITS> cargo run --example quasi_monte_carlo -- --name quasi_monte_carlo -k <DEGREE> mock # for example, DEGREE=8 or 16, BITS = DEGREE - 1
```

Here is the successful running output:

```markdown
[2024-12-16T08:38:36Z INFO  halo2_base::gates::flex_gate::threads::multi_phase] Auto-calculated config params:
    FlexGateConfigParams {
       k: 16,
       num_advice_per_phase: [
          5,
       ],
       num_fixed: 1,
    }
Total range check advice cells to lookup per phase: [78072, 0, 0]
[2024-12-16T08:38:36Z INFO  halo2_base::gates::circuit::builder] Auto-calculated config params:
    BaseCircuitParams {
       k: 16,
       num_advice_per_phase: [
          5,
       ],
       num_fixed: 1,
       num_lookup_advice_per_phase: [
          2,
          0,
          0,
       ],
       lookup_bits: Some(
          15,
       ),
       num_instance_columns: 1,
    }
[2024-12-16T08:38:36Z INFO  halo2_base::gates::flex_gate] Poisoned rows after FlexGateConfig::configure 9
[2024-12-16T08:38:36Z INFO  halo2_base::gates::range] Poisoned rows after RangeConfig::configure 9
```

### Key Generation

To generate a random universal trusted setup (for testing only!) and the proving and verifying keys for your circuit, run:

```bash
LOOKUP_BITS=15 cargo run --example quasi_monte_carlo -- --name quasi_monte_carlo -k 16 --input quasi_monte_carlo.0.in keygen
```

For technical reasons (to be removed in the future), keygen still requires an input file of the correct format. However, keygen is only done once per circuit, so it is best practice to use a different input than the input you want to test with.

This will generate a proving key `data/quasi_monte_carlo.pk` and a verifying key `data/quasi_monte_carlo.vk`. It will also generate a file `configs/quasi_monte_carlo.json` which describes (and pins down) the configuration of the circuit. This configuration file is later read by the prover.

Here is the successful running output:

```markdown
threshold: 1.0
In circle count: 787.0
Ratio: 0.787
Estimated π: 3.148
Gate Chip | Phase 0: 293258 advice cells
Total 19 fixed cells
Total range check advice cells to lookup per phase: [78072, 0, 0]
Start:   Generating vkey & pkey
End:     Generating vkey & pkey ....................................................1.419s
Proving key written to: "data/quasi_monte_carlo.pk"
Verifying key written to: "data/quasi_monte_carlo.vk"
```

### Proof Generation

After you have generated the proving and verifying keys, you can generate a proof for your circuit using:

```bash
LOOKUP_BITS=15 cargo run --example quasi_monte_carlo -- --name quasi_monte_carlo -k 16 prove
```

This creates a SNARK proof, stored as a binary file `data/quasi_monte_carlo.snark`, using the inputs read (by default) from `data/quasi_monte_carlo.in`. You can specify a different input file with the option `--input filename.in`, which would look for a file at `data/filename.in`.

Using the same proving key, you can generate proofs for the same ZK circuit on _different_ inputs using this command.

Here is the successful running output:

```markdown
threshold: 1.0
In circle count: 787.0
Ratio: 0.787
Estimated π: 3.148
Start:   Reading pkey from "data/quasi_monte_carlo.pk"
End:     Reading pkey from "data/quasi_monte_carlo.pk" .............................43.488ms
Start:   Create proof
End:     Create proof ..............................................................2.542s
Start:   Write SNARK
End:     Write SNARK ...............................................................7.250ms
Proving time: 2.55354475s
Snark written to: "data/quasi_monte_carlo.snark"
```

### Verifying a Proof

You can verify the proof generated above using:

```bash
LOOKUP_BITS=15 cargo run --example quasi_monte_carlo -- --name quasi_monte_carlo -k 16 verify
```

Here is the successful running output:

```markdown
threshold: 1.0
In circle count: 787.0
Ratio: 0.787
Estimated π: 3.148
Gate Chip | Phase 0: 293258 advice cells
Total 19 fixed cells
Total range check advice cells to lookup per phase: [78072, 0, 0]
Snark verified successfully in 3.700833ms
```

## Quick Start with `Original Monte Carlo`

For our ablation experiment to evaluate the influence without the ZKPs, suggesting that the original Monte Carlo method, due to its inherent randomness, cannot be correctly implemented within the circuit.

You could simply run the bash script for four stages, including the mock prover, key generation, proof generation, and proof verification:

```bash
bash MC.sh
```
