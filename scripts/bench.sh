#!/bin/bash

echo "Initializing benchmarks"

cargo run &
cargo_run_pid=$!

while ! nc -z 0.0.0.0 7878; do
  # wait for 1/10 of the second before checking again
  sleep 0.1
done

cargo bench && kill $cargo_run_pid

echo "Benchmarks ran successfuly"
