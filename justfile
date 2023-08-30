# Run tests with code coverage enabled
test:
    #!/usr/bin/env bash
    export RUSTFLAGS="-Cinstrument-coverage"
    export LLVM_PROFILE_FILE="inst-decoding-8086-%p-%m.profraw"
    cargo test

# Clean up any existing coverage files
clean:
    #!/usr/bin/env bash
    rm -f *.profraw
    rm -f *.profdata
    rm -rf ./target/debug/lcov.info
    rm -rf ./target/debug/coverage/

# Generate a coverage report
cover:
    #!/usr/bin/env bash
    export CARGO_INCREMENTAL=0
    export RUSTFLAGS="-Zprofile -Ccodegen-units=1 -Copt-level=0 -Clink-dead-code -Coverflow-checks=off -Zpanic_abort_tests -Cpanic=abort"
    export RUSTDOCFLAGS="-Cpanic=abort"
    grcov . --binary-path ./target/debug/ -s . -t html --branch --llvm --ignore-not-existing --excl-line "GRCOV_EXCL_LINE" --excl-start "GRCOV_EXCL_START" --excl-stop "GRCOV_EXCL_STOP" -o ./target/debug/coverage/
    grcov . --binary-path ./target/debug/ -s . -t lcov --branch --ignore-not-existing  --excl-line "GRCOV_EXCL_LINE" --excl-start "GRCOV_EXCL_START" --excl-stop "GRCOV_EXCL_STOP" -o ./target/debug/lcov.info
    genhtml -o ./target/debug/ --show-details --highlight --ignore-errors source --legend ./target/debug/lcov.info
    rm -rf *.profraw