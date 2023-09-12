source ../emsdk/emsdk_env.sh

export EMCC_CFLAGS="-O3"

cargo build --package tree-sitter-json --target wasm32-unknown-emscripten --release

export EMCC_CFLAGS="-O3
                    -o ./public/increment.js
                    -s EXPORTED_FUNCTIONS=['_increment','_get_json_cst','_free_string']
                    -s EXPORTED_RUNTIME_METHODS=ccall"

cargo build --target wasm32-unknown-emscripten --release --lib