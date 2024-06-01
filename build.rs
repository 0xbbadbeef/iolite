fn main() {
  cc::Build::new()
    .file("src/blowfish/blowfish.cpp")
    .file("src/blowfish/wrapper.cpp")
    .compile("FFXIVBlowfish")
}
