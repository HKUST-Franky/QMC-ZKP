# Run the Mock Test
RUST_LOG=info LOOKUP_BITS=15 cargo run --example quasi_monte_carlo -- --name quasi_monte_carlo -k 16 mock

# Generate the Key
LOOKUP_BITS=15 cargo run --example quasi_monte_carlo -- --name quasi_monte_carlo -k 16 --input quasi_monte_carlo.0.in keygen

# Prove the Circuit
LOOKUP_BITS=15 RUST_BACKTRACE=full cargo run --example quasi_monte_carlo -- --name quasi_monte_carlo -k 16 prove

# Verify the Proof
LOOKUP_BITS=15 RUST_BACKTRACE=full cargo run --example quasi_monte_carlo -- --name quasi_monte_carlo -k 16 verify