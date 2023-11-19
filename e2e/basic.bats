#!/usr/bin/env bats

load 'helpers/load'

@test "does not panics on default run" {
    run bash -c '$BIN' &

    sleep 1

    pkill -9 http-server

    sleep 1

    assert_success
}

@test "teardowns on graceful shutdown" {
    run bash -c '$BIN --graceful-shutdown' &

    sleep 1

    pkill -SIGINT http-server

    sleep 1

    assert_success
}
