#!/bin/bash

echo "Initializing test"

cargo run --features dhat-profiling -- $1 &
cargo_run_pid=$!

while ! nc -z 0.0.0.0 7878; do
  # wait for 1/10 of the second before checking again
  sleep 0.1
done

cargo test && kill -2 $cargo_run_pid && wait $cargo_run_pid

echo "Test ran successfuly"
