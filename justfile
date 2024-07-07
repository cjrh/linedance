test:
    @echo "Running tests"
    cargo build --example simple_usage
    @echo
    @echo "Running simple_usage example: stdin"
    cat examples/data.txt | ./target/debug/examples/simple_usage
    @echo
    @echo "Running simple_usage example: file"
    ./target/debug/examples/simple_usage --files examples/*.txt
