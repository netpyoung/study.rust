https://marketplace.visualstudio.com/items?itemName=kitamstudios.RustAnalyzer&ssr=false#overview
  프로젝트 템플릿 없음 폴더체로
https://www.jetbrains.com/ko-kr/rust/
https://lap.dev/lapce/
  Modern open source code editor in Rust
https://www.slideshare.net/xtozero/game-physics-engine-development

cargo init

rust
 All Rust string types explained https://www.youtube.com/watch?v=CpvzeyzgQdw


문자열
라이프타임.
이터레이터


https://danielkeep.github.io/itercheat_baked.html

https://github.com/github/gitignore/blob/master/Rust.gitignore

read eval print loop


`C:\ProgramData\Microsoft\Windows\Start Menu\Programs\Visual C++ Build Tools\Windows Desktop Command Prompts\Visual C++ 2015 x64 Native Build Tools Command Prompt`



- 메모리세이프
   // borrow checker
  - rust
    Graydon Hoare
	https://github.com/graydon
	https://graydon2.dreamwidth.org/
	
	https://news.hada.io/topic?id=14872
	https://github.com/mainmatter/100-exercises-to-learn-rust
	https://rust-exercises.com/100-exercises/01_intro/01_syntax


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


## build.rs
https://doc.rust-lang.org/cargo/reference/build-scripts.html