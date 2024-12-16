# Run the Mock Test
RUST_LOG=info cargo run --example monte_carlo -- --name monte_carlo -k 8 mock

# Generate the Key
cargo run --example monte_carlo -- --name monte_carlo -k 8 --input monte_carlo.0.in keygen

# Proof the Circuit
RUST_BACKTRACE=full cargo run --example monte_carlo -- --name monte_carlo -k 8 prove

# Verify the Proof
RUST_BACKTRACE=full cargo run --example monte_carlo -- --name monte_carlo -k 8 verify