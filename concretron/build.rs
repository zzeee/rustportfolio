fn main() {
    // Tell Cargo that if the given file changes, to rerun this build script.
  //  println!("cargo:rustc-link-search=/home/pi/rust2"); //UNCOMMENT ON RUSP
    println!("cargo:rustc-link-search=/opt/homebrew/Cellar/jack/1.9.17/lib");
    // Use the `cc` crate to build a C file and statically link it.

}
