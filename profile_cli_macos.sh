rm -rf ./instruments/cli_CPU-Counters.trace
xctrace record --template "CPU Counters" --output ./instruments/cli_CPU-Counters.trace --target-stdin=/dev/ttys001 --target-stdout=/dev/ttys001 --launch -- ./target/release/cli ./test_data/gsoc-2018.json
