#!/bin/bash

echo "Initializing test suite \"$SUITE\""

# Execute with DHAT Profiling and store PID in `cargo_run_pid`
cargo run --features dhat-profiling -- $1 & cargo_run_pid=$!

# Waits for server to be available to continue
while ! nc -z 0.0.0.0 7878; do
  # wait for 1/10 of the second before checking again
  sleep 0.1
done

# Execute tests and store PID in `cargo_test_pid`
cargo test $SUITE & cargo_test_pid=$!

# Waits for tests to finish before continuing
wait $cargo_test_pid

# Store exit status
EXIT_STATUS=$?

# Kill server running on `cargo_run_pid`
kill -KILL $cargo_run_pid && wait $cargo_run_pid

if [ "$EXIT_STATUS" -eq 0 ]; then
    echo "Tests for suite '$SUITE' succeded"
    exit 0
else
    echo "Tests for suite '$SUITE' failed"
    exit 1
fi
