fn main() {
    // [build.rs docs](https://doc.rust-lang.org/cargo/reference/build-scripts.html)
    println!("cargo:rerun-if-changed=./build");
    println!("cargo:rerun-if-changed=./c");
    let mut child = std::process::Command::new("./c/build.sh")
        .spawn()
        .expect("failed to spawn 'build c lib' child process");
    let ecode = child
        .wait()
        .expect("failed to wait on 'build c lib' child process");
    assert!(ecode.success());
    println!("cargo:rustc-link-lib=static=my_lib");
    println!("cargo:rustc-link-search=native=./build");
}
