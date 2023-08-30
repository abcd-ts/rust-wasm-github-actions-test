source ../emsdk/emsdk_env.sh

export EMCC_CFLAGS="-O3
                    -o ./increment.js
                    -s EXPORTED_FUNCTIONS=['_increment']
                    -s EXPORTED_RUNTIME_METHODS=ccall"

cargo build --target wasm32-unknown-emscripten --release --lib