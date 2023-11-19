#!/usr/bin/env bats

load 'helpers/load'

@test "basic_auth_resolves_request_successfuly" {
    $BIN --username john --password appleseed &
    sleep 1

    run curl -s -u john:appleseed http://localhost:7878
    assert_output "{\"status_code\":401,\"message\":\"Unauthorized\"}"
    assert_success
    sleep 1

    pkill -9 http-server
    sleep 1

    assert_success
}
