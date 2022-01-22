extern crate cc;

fn main() {
    cc::Build::new()
        .file("tests/dummy_plugin.c")
        .include("target/debug/build/polychat-plugin-66da350ab16af3ae/out")
        .out_dir("target")
        .shared_flag(true)
        .compile("garbage");
}