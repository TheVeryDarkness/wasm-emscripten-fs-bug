# Some issues with wasm32-unknown-emscripten and file system API

Thank you for turning to this repro. I'm running this on my Mac.

I've stored content from stdout in `./logs/*.log`.

- Metadata is wrong, such as file type is the same with the premission code and file size in bytes is a very big number (changing in time, and it seems to be the Unix timestamp).

  - [metadata](./src/bin/metadata.rs).

- Can't compile if I have such configuration:

  ```toml
  [target.wasm32-unknown-emscripten]
  rustflags = [
      "-C",
      "link-arg=-lnodefs.js",
      "-C",
      "link-arg=-lnoderawfs.js",
      "-C",
      "link-arg=-sALLOW_MEMORY_GROWTH",
      "-C",
      "link-arg=-sEXIT_RUNTIME=1",
      "-C",
      "link-arg=-sSTRICT=1",
  ]
  runner = "node"

  [env]
  EMCC_STRICT="1"
  ```

  I got:

  ```log
    = note: emcc: warning: setting `DISABLE_EXCEPTION_CATCHING` is not meaningful unless linking as C++ [-Wlinkflags]
          wasm-ld: error: symbol exported via --export not found: __cxa_is_pointer_type
          wasm-ld: error: symbol exported via --export not found: __cxa_can_catch
          wasm-ld: error: symbol exported via --export not found: __cxa_increment_exception_refcount
          wasm-ld: error: symbol exported via --export not found: __cxa_decrement_exception_refcount
          wasm-ld: error: symbol exported via --export not found: __cxa_free_exception
  ......
  ```

  And I can see a `"-s" "DISABLE_EXCEPTION_CATCHING=0"` in the arguments when running with `-v`.

  And things are going well if I execute `EMCC_STRICT=1 em++ test-with-cpp/test.cpp -lnodefs.js -lnoderawfs.js -sALLOW_MEMORY_GROWTH -sEXIT_RUNTIME=1 -sSTRICT=1 -o test-with-cpp/build/test.js`. So I think the error comes from those `--export`.
