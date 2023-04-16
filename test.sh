#!/usr/bin/env sh

set -x

DEFAULT_WASM_BINDGEN_VERSION="0.2.84"
WASM_BINDGEN_VERSION=$DEFAULT_WASM_BINDGEN_VERSION
TESTS_FAILED=0
BUILD_FAILED=0
REMOVE_TARGET_DIR=0
REMAINING_ATTEMPTS_TO_REPRODUCE=20

for arg in "$@"; do
  case $arg in
    -b|--wasm-bindgen-version)
    WASM_BINDGEN_VERSION="$2"
    shift 2
    ;;
    --remove-target-dir)
    REMOVE_TARGET_DIR=1
    shift
    ;;
    --max-attempts-to-reproduce)
    REMAINING_ATTEMPTS_TO_REPRODUCE="$2"
    shift 2
    ;;
  esac
done


set_config() {
    sed -i "s/wasm-bindgen = \".*\"/wasm-bindgen = \"$WASM_BINDGEN_VERSION\"/" Cargo.toml
}

revert_config() {
    sed -i "s/wasm-bindgen = \".*\"/wasm-bindgen = \"$DEFAULT_WASM_BINDGEN_VERSION\"/" Cargo.toml
}

build() {
    RUSTFLAGS='-C target-feature=+atomics,+bulk-memory,+mutable-globals' \
        cargo build --target wasm32-unknown-unknown --release \
        -Z build-std=std,panic_abort || BUILD_FAILED=1
    wasm-bindgen \
        target/wasm32-unknown-unknown/release/my_app.wasm \
        --out-dir app/pkg \
        --target web
}

run_tests() {
    cd tests
    npx playwright test \
        --trace=on \
        --grep="has title" \
        --reporter=list \
    || TESTS_FAILED=1
    cd ..
}

run() {
    if [ $REMOVE_TARGET_DIR -eq 1 ]; then
        rm -rf target
    fi
    set_config

    build
    if [ "$BUILD_FAILED" -eq 1 ]; then
        echo "Build failed" >&2
        exit 1
    fi

    run_tests

    revert_config

    if [ $TESTS_FAILED -eq 0 ]; then
        echo "Tests passed!"
    else
        if [ -z "$CI" ]; then 
            npx playwright show-trace \
                tests/test-results/webkit-fails-has-title-webkit/trace.zip &
        fi
        echo "Tests failed" >&2
    fi
}

main() {
    while [ $REMAINING_ATTEMPTS_TO_REPRODUCE -gt 0 ]; do
        run
        if [ $TESTS_FAILED -eq 1 ]; then
            break
        fi
        REMAINING_ATTEMPTS_TO_REPRODUCE=$((REMAINING_ATTEMPTS_TO_REPRODUCE - 1))
    done
    exit $TESTS_FAILED
}

main
