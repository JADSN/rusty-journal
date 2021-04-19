all: run

run:
	cargo run

list:
	cargo run -- -j test-journal.json list

add:
	 cargo run -- -j test-journal.json add "water the plants"

done:
	 cargo run -- -j test-journal.json done 1