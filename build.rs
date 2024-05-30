fn main() {
  cc::Build::new()
    .file("blowfish/blowfish.cpp")
    .file("blowfish/wrapper.cpp")
    .compile("FFXIVBlowfish")
}
