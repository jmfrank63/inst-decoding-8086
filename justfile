# Run tests with code coverage enabled
test:
    #!/usr/bin/env bash
    cargo fmt
    export RUSTFLAGS="-Cinstrument-coverage"
    export LLVM_PROFILE_FILE="inst-decoding-8086-%p-%m.profraw"
    cargo test

# Clean up any existing coverage files
clean-profdata:
    #!/usr/bin/env bash
    rm -f *.profraw
    rm -f *.profdata

clean-reports:
    #!/usr/bin/env bash
    rm -f ./target/debug/lcov.info
    rm -rf ./target/debug/lcov
    rm -rf ./target/debug/coverage

clean-html:
    #!/usr/bin/env bash
    rm -rf ./target/debug/coverage

clean-lcov:
    #!/usr/bin/env bash
    rm -f ./target/debug/lcov.info
    rm -rf ./target/debug/lcov

# Generate a coverage report
cover-html:
    #!/usr/bin/env bash
    export CARGO_INCREMENTAL=0
    export RUSTFLAGS="-Zprofile -Ccodegen-units=1 -Copt-level=0 -Clink-dead-code -Coverflow-checks=off -Zpanic_abort_tests -Cpanic=abort"
    export RUSTDOCFLAGS="-Cpanic=abort"
    grcov . --binary-path ./target/debug/ -s . -t html --branch --llvm --ignore-not-existing --ignore "/*" -o ./target/debug/coverage

cover-lcov:
    #!/usr/bin/env bash
    export CARGO_INCREMENTAL=0
    export RUSTFLAGS="-Zprofile -Ccodegen-units=1 -Copt-level=0 -Clink-dead-code -Coverflow-checks=off -Zpanic_abort_tests -Cpanic=abort"
    export RUSTDOCFLAGS="-Cpanic=abort"
    grcov . --binary-path ./target/debug -s . -t lcov --branch --llvm --ignore-not-existing --ignore "/*" -o ./target/debug/lcov.info
    genhtml -o ./target/debug/lcov --show-details --highlight --ignore-errors source  --ignore-errors unmapped,unmapped --legend ./target/debug/lcov.info
