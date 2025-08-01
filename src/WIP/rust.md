
##

https://doc.rust-lang.org/std/macro.panic.html
``` rust
panic!("This is a panic!");
```

# IDE

## VS Code


## Visual Studio

- [kitamstudios.RustAnalyzer](https://marketplace.visualstudio.com/items?itemName=kitamstudios.RustAnalyzer)
  - 프로젝트 템플릿 없음 폴더체로

## Rust Rover ( JetBrains )

- <https://www.jetbrains.com/ko-kr/rust/>

## lapce

- <https://lap.dev/lapce/>
  - Modern open source code editor in Rust


https://www.slideshare.net/xtozero/game-physics-engine-development

https://danielkeep.github.io/itercheat_baked.html

https://github.com/github/gitignore/blob/master/Rust.gitignore


`C:\ProgramData\Microsoft\Windows\Start Menu\Programs\Visual C++ Build Tools\Windows Desktop Command Prompts\Visual C++ 2015 x64 Native Build Tools Command Prompt`


- rust개발자  Graydon Hoare
  - https://github.com/graydon
  - https://graydon2.dreamwidth.org/
	
	https://news.hada.io/topic?id=14872



rust
https://github.com/microsoft/windows-rs


## unwrap ( 이건 trait아님 )
Result - unwrap
Option - unwrap

unwrap()은 실패하면 바로 프로그램이 패닉(panic) 
unwrap_or
unwrap_or_else
?는 실패하면 현재 함수를 즉시 종료하고, 그 에러를 호출자에게 반환


let cstring = CString::new("hello").unwrap();
let raw: *mut c_char = cstring.into_raw();
unsafe {
    let _ = CString::from_raw(raw); // drop 되면서 메모리 해제
}


# rust

# The Rust Programming Language

https://github.com/killercup/trpl-ebook

https://github.com/reidarsollid/RustyCage

https://github.com/kud1ing/awesome-rust#ides

http://areweideyet.com/

https://github.com/phildawes/racer

https://github.com/RustDT/RustDT




# TOML (Tom's Obvious, Minimal Language)
INI와 비슷하지만,
TOML aims to be a minimal configuration file format that's easy to read due to obvious semantics. TOML is designed to map unambiguously to a hash table. TOML should be easy to parse into data structures in a wide variety of languages.

https://github.com/toml-lang/toml



# JAVA interop
jna
https://github.com/jnr/jnr-ffi
http://openjdk.java.net/projects/panama/

https://github.com/Monnoroch/RustJni

https://www.youtube.com/watch?v=O5vzLKg7y-k

http://rustbyexample.com/

https://doc.rust-lang.org/nomicon/


https://www.penflip.com/sarojaba/rust-doc-korean

https://github.com/kud1ing/awesome-rust

Rust Guidelines
http://aturon.github.io/

rust-learning
https://github.com/ctjhoa/rust-learning


소유권
http://rustbyexample.com/scope.html
https://www.penflip.com/sarojaba/rust-doc-korean/blob/master/ownership.md



rustup
http://www.rustup.rs/

rusty-tags
https://github.com/dan-t/rusty-tags

rust-clippy
https://github.com/Manishearth/rust-clippy

Felix Klock - Rust: A type system you didn't know you wanted - Curry On

https://www.youtube.com/watch?v=Q7lQCgnNWU0
http://pnkfelix.github.io/curry-on2015.html#/thanks

https://www.youtube.com/watch?v=d1uraoHM8Gg


http://hackr.io/
http://aml3.github.io/RustTutorial/html/toc.html
https://jaredonline.svbtle.com/roguelike-tutorial-in-rust-part-1
https://stepic.org/lesson/Rust-The-Basics-9268/step/1?unit=undefined



# TODO
* android ndk
  - https://developer.android.com/studio/projects/add-native-code.html
* rust cdylib
  - https://blog.rust-lang.org/2016/07/07/Rust-1.10.html

* http://stackshare.io/

* 팀을 구하는 툴 개발: http://ndcreplay.nexon.com/NDC2014/sessions/NDC2014_0076.html
