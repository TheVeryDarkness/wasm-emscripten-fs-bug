use std::env::current_dir;

fn main() {
    println!("Current: {:?}", current_dir().unwrap());
    wasm_emscripten_fs_bug::tree2("~", 0);
}
