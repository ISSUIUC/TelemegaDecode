
fn main() {
    println!("cargo:rerun-if-changed=src/gfsk/ao.cpp");
    println!("cargo:rerun-if-changed=src/gfsk/ao.h");
    println!("cargo:rerun-if-changed=src/gfsk/hackrf_stuff.c");
    // println!("cargo:rerun-if-changed=src/box-of-shame/*.*");
    cc::Build::new()
        .file("src/gfsk/hackrf_stuff.c")
        .compile("hackrf_stuff");

    cc::Build::new()
        .file("src/gfsk/ao.cpp")
        .compile("ao_fec");
    // cc::Build::new()
    //     .cpp(true)
    //     .std("c++17")
    //     .file("src/box-of-shame/gfsk.cpp")
    //     .file("src/box-of-shame/ao.cpp")
    //     .file("src/box-of-shame/Shifter.cpp")
    //     .file("src/box-of-shame/StreamingBitSync.cpp")
    //     .file("src/box-of-shame/StreamingFec.cpp")
    //     .file("src/box-of-shame/StreamingGFSK.cpp")
    //     .compile("cpp_gfsk");
}
