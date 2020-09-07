fn main() {
    // some functions in crate libc not work; so link it again
    println!("cargo:rustc-link-lib=c");
    println!("cargo:rustc-link-search=/usr/lib/");
}
