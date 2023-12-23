set -eux
EMCC_STRICT=1 em++ test-with-cpp/test.cpp -lnodefs.js -lnoderawfs.js -sALLOW_MEMORY_GROWTH -sEXIT_RUNTIME=1 -sSTRICT=1 -o test-with-cpp/build/test.js
node test-with-cpp/build/test.js >logs/test-with-cpp.log
RUST_BACKTRACE=1 cargo +nightly run -Zbuild-std --target wasm32-unknown-emscripten --release -vv --bin tree1    >logs/tree1.log
RUST_BACKTRACE=1 cargo +nightly run -Zbuild-std --target wasm32-unknown-emscripten --release -vv --bin tree2    >logs/tree2.log
RUST_BACKTRACE=1 cargo +nightly run -Zbuild-std --target wasm32-unknown-emscripten --release -vv --bin metadata >logs/metadata.log
RUST_BACKTRACE=1 cargo +nightly run -Zbuild-std --target wasm32-unknown-emscripten --release -vv --bin mount    >logs/mount.log
