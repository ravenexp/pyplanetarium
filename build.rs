fn main() {
    if std::env::var("TARGET").unwrap() == "x86_64-pc-windows-gnu" {
        let libdir = std::env::var("PYO3_CROSS_LIB_DIR")
            .expect("PYO3_CROSS_LIB_DIR is not set when cross-compiling");
        python3_dll_a::generate_implib(&libdir)
            .expect("python3.dll import library generator failed");
    }
}
