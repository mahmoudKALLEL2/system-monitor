# Justfile

# Build the project
build:
    cargo build

# Run the project with the CPU stress test
cpu:
    cargo run -- cpu --duration 10 --threshold 80.0

# Run the project with the memory stress test
memory:
    cargo run -- memory --duration 10

# Run the endurance test for storage
storage:
    cargo run -- storage --duration 10 --file-path test_file.bin

# Run All endurence tests
run-all:
    cargo run -- run-all --duration 10

# Run all tests
test:
    cargo test

# Clean the project
clean:
    cargo clean

# Help command to list available tasks
help:
    @echo "Available commands:"
    @echo "  just build            - Build the project"
    @echo "  just cpu              - Run the CPU stress test"
    @echo "  just memory           - Run the memory stress test"
    @echo "  just run-all          - Run All stress tests"
    @echo "  just storage          - Run the storage endurance test"
    @echo "  just test             - Run all tests"
    @echo "  just clean            - Clean the project"
