
fn main() {
    println!("cargo:rerun-if-changed=src/hackrf_stuff.c");

    cc::Build::new()
        .file("src/hackrf_stuff.c")
        .compile("hackrf_stuff");
}
