# trait

- 다른 언어에서의 interface라고 보면됨

## 자동파생

``` rust
// 내부 필드 x와 y가 전부 i32이고, i32는 이 모든 트레이트를 이미 구현하고 있기 때문에,
// Point는 자동으로 Debug, Clone, PartialEq, Eq, Hash, Default를 구현

#[derive(Debug, Clone, PartialEq, Eq, Hash, Default)]
struct Point {
    x: i32,
    y: i32,
}
```

## 닷넷과 비교

| 닷넷의                | 트레이트                                   |                                      |
| --------------------- | ------------------------------------------ | ------------------------------------ |
| IDisposable.Dispose() | `Drop`                                     | 소멸자 (scope 벗어날 때 커스텀 정리) |
| ICloneable.Clone()    | `Clone`                                    |                                      |
| ToString()            | `Display`                                  |                                      |
| IEquatable<T>         | `Eq`                                       | `PartialEq`도 만족해야함             |
| IComparable<T>        | `Ord`                                      | `Eq` + `PartialOrd` 만족해야 함      |
| GetHashCode()         | `Hash`                                     |                                      |
| IEnumerable           | `Iterator` ,`IntoIterator`, `FromIterator` |                                      |

## #[derive(Debug)]

- 닷넷의 DebuggerDisplay 어트리뷰트와 비슷하지만 닷넷은 디버거 표시에만 나타낼 뿐이고, rust는 출력용.

|       |                           |
| ----- | ------------------------- |
| {:?}  | 한 줄, 간결한 디버그 표현 |
| {:#?} | pretty 출력               |

## ToOwned 

- 빌린 값(&T)의 소유된 owned 복사본을 만든다
  - Copy 타입이 아니면, 보통 새로운 소유 데이터를 만들기 때문에 힙 메모리 할당이 발생

let s1: String = "hello".to_owned();
```
"hello" (바이너리 상수, &str)
|
|
*  to_owned()
[ h e l l o \0 ]   <- 힙에 새로 복사됨
|
s1: String
```

## 기타 Trait

|                              |                                         |                                                    |                                        |
| ---------------------------- | --------------------------------------- | -------------------------------------------------- | -------------------------------------- |
| `Default`                    | 기본값 생성                             | `let v: Vec<i32> = Default::default();`            | 여러 타입에 대해 `::default()` 제공    |
| `AsRef<T>`                   | 참조로 변환 (가벼운 뷰)                 | `fn foo<S: AsRef<str>>(s: S)`                      | `&String` → `&str` 등                  |
| `AsMut<T>`                   | 가변 참조로 변환                        | 비슷하게 `fn bar<S: AsMut<[u8]>>(s: &mut S)`       |                                        |
| `Borrow<T>` / `BorrowMut<T>` | 키/뷰로서 빌려쓰기 (컬렉션 내부와 호환) | `HashMap<String, V>`에 `&str`로 접근 가능하게 해줌 | `AsRef`와 유사하지만 의미가 다름       |
| `Send`                       | 스레드 간 안전한 이동 가능              | `std::thread::spawn`의 인자에 필요                 | 컴파일러가 타입이 `Send`인지 추론      |
| `Sync`                       | 여러 스레드에서 동시에 참조 가능        | `&T`가 스레드 안전하면 `T: Sync`                   |                                        |
| `Sized`                      | 컴파일 타임 크기 고정                   | 대부분 타입에 자동                                 | `?Sized`로 역전 가능                   |
| `'static` (라이프타임 제한)  | 프로그램 전체 생존 가능 참조            | `let s: &'static str = "hello";`                   | 트레이트는 아니지만 제약으로 자주 등장 |


## Into<T> 트레잇

- Into<U>는 From<T> for U가 있으면 자동 구현됨
- 다른 타입으로 바꾸고 소유권도 넘기는데 쓰임.
