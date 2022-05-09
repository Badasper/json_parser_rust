dev:
	cargo build

prod:
	cargo build --release

run:
	cargo run

start:
	./target/debug/json_parser

start_prod:
	./target/release/json_parser