https://github.com/eqrion/cbindgen
https://github.com/mono/CppSharp
https://github.com/Dushistov/flapigen-rs

https://medium.com/@chyyran/calling-c-natively-from-rust-1f92c506289d


https://github.com/swig/swig/issues/455



HandleRef 1.0
SafeHandle 2.0
https://docs.microsoft.com/en-us/dotnet/api/system.runtime.interopservices.handleref?view=net-6.0


https://devblogs.microsoft.com/dotnet/improvements-in-native-code-interop-in-net-5-0/


https://dev.to/robertohuertasm/rust-once-and-share-it-with-android-ios-and-flutter-286o


crate-type = ["staticlib", "cdylib"]

[target.'cfg(target_os = "android")'.dependencies]
jni = { version = "0.13.1", default-features = false }

#[repr(C)]
struct X {
}

#[no_mangle]
pub unsafe extern "C" fn  fn_test() {
    println!("Hello, world!");
}


# Android targets
rustup target add aarch64-linux-android armv7-linux-androideabi i686-linux-android x86_64-linux-android
cargo install cargo-ndk


# iOS targets
rustup target add aarch64-apple-ios armv7-apple-ios armv7s-apple-ios x86_64-apple-ios i386-apple-ios
# install the Xcode build tools.
xcode-select --install
# this cargo subcommand will help you create a universal library for use with iOS.
cargo install cargo-lipo
# this tool will let you automatically create the C/C++11 headers of the library.
cargo install cbindgen


cbindgen src/lib.rs -l c > rustylib.h



cargo lipo --release



https://github.com/shekohex/flutterust


jps
https://github.com/BezPowell/blitz-path/blob/master/src/jps.rs
https://github.com/samueltardieu/pathfinding

https://github.com/sidneywang/rsbind

https://blog.datalust.co/rust-at-datalust-how-we-integrate-rust-with-csharp/

==========

https://doc.rust-lang.org/cargo/reference/profiles.html#default-profiles
https://github.com/johnthagen/min-sized-rust
[profile.release]
opt-level = 3
debug = false
split-debuginfo = '...'  # Platform-specific.
debug-assertions = false
overflow-checks = false
lto = true             # default: false                 # https://llvm.org/docs/LinkTimeOptimization.html
panic = 'unwind'
incremental = false
codegen-units = 1      # default: 16
rpath = false

