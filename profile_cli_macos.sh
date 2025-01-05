rm -rf ./instruments/json_parser_cli_CPU-Counters.trace
xctrace record --template "CPU Counters" --output ./instruments/json_parser_cli_CPU-Counters.trace --target-stdin=/dev/ttys001 --target-stdout=/dev/ttys001 --launch -- ./target/release/json_parser_cli
