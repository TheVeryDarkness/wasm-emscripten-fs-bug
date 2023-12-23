use std::env::current_dir;

fn main() {
    println!("{:?}", current_dir());
    // See https://emscripten.org/docs/api_reference/Filesystem-API.html#FS.mount
    // emscripten_functions::emscripten::run_script("FS.mount(NODEFS, { root: '.' }, '/workspace');");
    wasm_emscripten_fs_bug::tree1("workspace", 0);
    wasm_emscripten_fs_bug::tree2("workspace", 0);
}
