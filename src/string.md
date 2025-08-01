# 문자열

- [All Rust string types explained](https://www.youtube.com/watch?v=CpvzeyzgQdw)
- `str`
  - dynamically sized type (DST, 크기가 컴파일 타임에 결정되지 않는 타입)
  - 직접 변수로 쓸 수 없고, 항상 포인터/참조(&, Box, Rc 등) 뒤에 붙어야 한다.

문자열 조작할때 String
immutable 문자열은 Box
공유를 원하면 Rc


| 타입      | 소유권          | 인코딩/내용   | 변경 가능성 | 용도 / 설명                                                                 |
| --------- | --------------- | ------------- | ----------- | --------------------------------------------------------------------------- |
| `&str`    | 불소유 (borrow) | UTF-8         | 불변        | 고정된 문자열 슬라이스. 컴파일타임 리터럴이나 `String`의 일부를 빌려 쓸 때. |
| `String`  | 소유            | UTF-8         | 가변        | 동적으로 생성/조작 가능한 문자열 버퍼.                                      |
| `Vec<u8>` | 소유            | 바이트 시퀀스 | 가변        | 비 UTF-8 또는 바이너리 문자열 처리, 인코딩 검증 전/후 버퍼.                 |

| 타입       | 소유권                       | 인코딩/내용 | 변경 가능성 | 용도 / 설명                                                                                                 |
| ---------- | ---------------------------- | ----------- | ----------- | ----------------------------------------------------------------------------------------------------------- |
| `Box<str>` | 소유 (Owned)                 | UTF-8       | 불변        | 힙에 고정된 문자열 슬라이스. `String`에서 크기를 줄이고 싶을 때, 변경 불가한 소유 문자열을 효율적으로 저장. |
| `Rc<str>`  | 공유 소유 (공유 참조 카운팅) | UTF-8       | 불변        | 여러 곳에서 읽기 전용 문자열을 복사 없이 공유할 때. 참조 카운트로 복제 비용을 줄임.                         |
| `Cow<str>` | 상황에 따라                  | UTF-8       | 상황에 따라 | 필요 시 복사(`Owned`)하거나 빌려쓰기(`Borrowed`)로 유연한 API에 사용.                                       |


| 타입      | 소유권 | 인코딩/내용        | 변경 가능성           | 용도 / 설명                                            |
| --------- | ------ | ------------------ | --------------------- | ------------------------------------------------------ |
| `CStr`    | 불소유 | C 문자열 (널 종료) | 불변                  | FFI로부터 받은 C 문자열 참조.                          |
| `CString` | 소유   | C 문자열 (널 종료) | 가변 (생성 후엔 불변) | Rust → C로 문자열 넘길 때, 내부에 널 바이트 없어야 함. |


| 타입       | 소유권 | 인코딩/내용                                                 | 변경 가능성 | 용도 / 설명                             |
| ---------- | ------ | ----------------------------------------------------------- | ----------- | --------------------------------------- |
| `OsStr`    | 불소유 | 플랫폼네이티브 (Windows: WTF-8/UTF-16, Unix: 바이트 시퀀스) | 불변        | 운영체제 경로/명령줄 등, 플랫폼 문자열. |
| `OsString` | 소유   | 플랫폼네이티브                                              | 가변        | `OsStr`의 소유 버전, 경로 등 구성할 때. |

| 기타      |                                                       |
| --------- | ----------------------------------------------------- |
| &[u8; N]  | 고정 크기 바이트 배열의 참조. 바이너리/저수준 데이터. |
| r#"..."#  | 이스케이프 시퀀스를 처리하지 않는 원시 문자열.        |
| b"..."    | 바이트 문자열 리터럴 (&[u8]).                         |
| br#"..."# | 이스케이프 없이 바이트 원시 문자열.                   |
| Path      | 파일 시스템 경로를 다루는 타입 (&Path 또는 PathBuf).  |

``` rust
let s: &str = "Hello"; // 문자열 리터럴은 &'static str

let mut s: String = String::from("Hello");
s.push_str(", world!");

use std::borrow::Cow;
let borrowed: Cow<str> = Cow::Borrowed("hello");
let owned: Cow<str> = Cow::Owned("world".to_string());

let bytes: Vec<u8> = b"hello".to_vec();
let s: String = String::from_utf8(bytes).unwrap();

use std::ffi::{CStr, CString};
let c_string: CString = CString::new("hello").unwrap();
let ptr: *const std::os::raw::c_char = c_string.as_ptr(); // *const c_char
unsafe {
    let c_str: &CStr = CStr::from_ptr(ptr);
    let rust_str: &str = c_str.to_str().unwrap();
}
```

``` rust
let owned: Box<str> = "Hello, Box<str>!".to_owned().into_boxed_str();
let shared: Rc<str> = Rc::from("Hello, Rc<str>!");

let p: &Path = Path::new("some/folder/file.txt");

let bs: &[u8] = b"hello";
let data: &[u8; 4] = b"\x01\x02\x03\x04";
let slice: &[u8] = data;

let s: &str = r#"He said: "Hello, \n world!""#;
```



``` rust
use std::borrow::Cow;

let mut cow: Cow<str> = Cow::Borrowed("hello");
let ptr_before: *const u8 = cow.as_ptr();

let s: &mut String = cow.to_mut();
s.push_str(" world!");
let ptr_after: *const u8 = cow.as_ptr();

// to_mut()하면서 내부 데이터를 복사해서 힙에 String을 만듬.
// 결과적으로 ptr_before와 ptr_after 주소가 다라짐.

// Cow (Clone on Write)는 처음에는 Borrowed(빌림) 상태로 존재하다가,
// Owned 상태로 변할 때 내부 데이터를 복사하여 소유권을 가지는 새로운 값으로 전환하는 타입.
// Owned로 전환하는 함수는 to_mut() 이나 into_owned() .
```