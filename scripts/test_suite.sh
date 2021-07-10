#!/bin/bash

echo "Initializing test suite \"$SUITE\""

cargo run --features dhat-profiling -- $1 &
cargo_run_pid=$!

while ! nc -z 0.0.0.0 7878; do
  # wait for 1/10 of the second before checking again
  sleep 0.1
done

cargo test $SUITE
EXIT_STATUS=$?
kill -2 $cargo_run_pid && wait $cargo_run_pid


if [ "$EXIT_STATUS" -eq 0 ]; then
    echo "Tests for suite '$SUITE' succed"
    exit 0
else
    echo "Tests for suite '$SUITE' failed"
    exit 1
fi
