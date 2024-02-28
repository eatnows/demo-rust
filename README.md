# 변수와 가변성
러스트에서 변수는 기본적으로 불변 (immutable) 이다.
```rust
fn main() {
    let x = 10;
    x = 20; // error
}
```
변수를 가변으로 만들고 싶으면 `mut` 키워드를 사용한다.
```rust
fn main() {
    let mut x = 10;
    x = 20;
}
```

# 상수
상수는 불변 변수와 마찬가지로 값을 바꾸는 것을 허용하지 않는다. 
변수와 가장 큰 차이점은 상수는 `mut` 키워드를 사용할 수 없다. 즉 항상 불변인 값이다.
`const` 키워드로 선언을 한다. 값의 타입은 반드시 명시되어야 한다.

상수는 전역 스코프를 포함한 어떤 스코프에서도 선언이 가능하다.
상수는 런타임에서 결정되는 값으로는 설정할 수 없고 상수 표현식으로만 설정될 수 있다.

상수는 단어 사이에 밑줄을 사용하고 문자를 대문자로 쓰는 것이 관례이다.
상수는 선언된 스코프 내에서 프로그램이 종료될 때까지 모든 시간동안 유효하다.

# Shadow
러스트에서는 새 변수를 이전 변수명으로 선언할 수 있다. 이전 변수가 새 변수에 의해 가려졌다고 표현하여 shadow 라고 한다.
변수를 사용할 때 컴파일러는 새 변수를 보게된다. 똑같은 변수명과 `let` 키워드로 변수를 가릴 수 있다.

```rust
fn main() {
    let x = 10;
    let x = 10 + 10;

    {
        let x = x + 10;
        println!("{}", x); // 30
    } // 스코프 안 쪽 shadow 가 끝난다.
    
    println!("{}", x); // 20
}
```

shadowing 은 `mut` 과는 다르다. `let` 키워드 없이 변수에 값을 재할당 하려고 한다면 에러가 발생한다.
`let` 사용하면 값을 변형하면서 불변을 유지할 수 있다. 

`let` 키워드로 새로운 변수를 만드는 것이기 때문에 같은 변수명으로 다른 데이터 타입의 값을 저장할 수 있다. 

```rust
fn main() {
    let x = "abc";  // 문자열 타입
    let x = x.len(); // 숫자 타입
}
```

# 데이터 타입
러스트의 모든 값은 특정한 타입을 가진다. 러스트는 모든 변수의 타입이 컴파일 시점에 반드시 정해져 있어야한다.(statically typed)
러스트 컴파일러는 타입을 추론할 수 있는데, 타입 추론이 불가능할 경우에는 명시적으로 타입을 적어줘야한다.
```rust
fn main() {
    let guess: u32 = "42".parse().expect("Not a number");    
}
```

## 스칼라 타입 (scalar type)
스칼라 타입은 하나의 값을 표현한다. 
1. 정수
2. 부동 소수점 숫자
3. Boolean
4. 문자

### 정수형 (integer type)
정수형은 소수점이 없는 숫자이다.

| 길이 | 부호 있음 (signed) | 부호 없음 (unsigned) |
|-----|------------------|-------------------|
| 8-bit | i8 | u8 |
| 16-bit | i16 | u16 |
| 32-bit | i32 | u32 |
| 64-bit | i64 | u64 |
| 128-bit | i128 | u128 |
| arch | isize | usize |

`isize` 와 `usize` 타입은 프로그램이 동작하는 컴퓨터의 아키텍처에 따라 데이터의 크기가 결정이 된다.
64-bit 아키텍처이면 64비트를, 32-bit 아키텍처이면 32비트를 갖게된다.

| 숫자 리터럴 | 예 |
|----------|-----|
| Decimal | 98_222 |
| Hex | 0xff |
| Octal | 0o77 |
| Binary | 0b1111_0000 |
| Byte(u8 only) | b'A' |

따로 데이터타입을 정하지 않고 정수를 사용한다면 기본값으로 `i32`로 선언된다.

### 부동 소수점 타입
러스트에도 소수점을 갖는 숫자인 부동 소수점(floating-point) 숫자 기본 타입이 두 가지 있다. 
부동 소수점 타입은 `f32`와 `f64`로 각각 32bit, 64bit 크기를 갖는다. 기본 타입은 `f64` 이다.

```rust
// IEEE-754 표준을 따른다.
fn main() {
    let x = 2.0;        // f64
    let y: f32 = 3.0;   // f32
}
```

`f32` 타입은 1배 수 정밀도 (single-precision) 인 부동 소수점이고, 
`f64`는 2배 수 정밀도 (double-precision) 이다.

### boolean 타입
다른 언어들처럼 `true`와 `false` 두 값을 가질 수 있다. boolean 값은 1바이트이다.

```rust
fn main() {
    let t = true;
    let f: bool = false;
}
```

### 문자타입
러스트의 `char` 언어의 가장 기본적인 알파벳 타입이다.
```rust
fn main() {
    let c = 'z';
    let z: char = 'ℤ';
}
```
`char` 타입은 작은따옴표를 쓰는 점을 주의해아한다. `char` 타입의 크기는 4바이트이며 유니코드 스칼라 값을 표현하며 ASCII보다 훨씬 더 많은 값을 표현할 수 있다.


## 복합 타입 (compound type)
복합 타입은 여러 값을 하나의 타입으로 묶을 수 있습니다. 러스트에는 튜플 (tuple)과 배열 (array) 두 가지 기본 복합 타입이 있다.

### 튜플 타입 (tuple type)
튜플은 다양한 타입의 여러 값을 묶어 하나의 복합 타입으로 만드는 일반적인 방법이다. 튜플은 고정된 크기를 갖는다. 한 번 선언되면 크기를 줄이거나 늘릴 수 없다.

괄호 안에 쉼표로 구분하여 값들의 목록을 작성하면 튜플을 만들 수 있다. 튜플 내의 각 위치는 타입을 갖고, 타입들은 서로 달라도 된다.
```rust
fn main() {
    // 타입을 명시하지 않아도 됨.
    let tup: (i32, f64, String) = (500, 6.4, "string");
}
```
튜플은 하나의 복합 요소로 취급되므로 변수 `tup`은 튜플 전체가 바인딩된다. 패턴 매칭을 하여 튜플 값을 해체하여 사용할 수 있다.

```rust
fn main() {
    let tup = (500, 6.4, 1);
    let (x, y, z) = tup;

    println!("The value of y is: {y}");
}
```
tuple을 세 개의 분리된 변수 `x`, `y`, `z`로 바꾼다. 이것을 구조 해체(destructuring) 이라고 부른다.

`.` 뒤에 접근하고자 하는 값의 인덱스를 쓰는 방식으로도 튜플 요소에 접근할 수 있다.
```rust
fn main() {
    let x: (i32, f64, u8) = (500, 6.4, 1);
    let five_hundred = x.0;
    let six_point_four = x.1;
}
```
아무 값도 없는 튜플은 유닛 (unit) 이라고 한다. 이 값과 타입은 모두 `()`로 작성되고 빈 값이나 비어있는 반환 타입을 나타낸다.
표현식이 어떠한 값도 반환하지 않는다면 암묵적으로 유닛 값을 반환하게 된다.

### 배열 타입
튜플과는 달리 배열의 모든 요소는 모두 같은 타입이어야 한다. 러스트의 배열은 고정된 길이를 갖는다.

```rust
fn main() {
    let a = [1, 2, 3, 4, 5];
}
```
힙보다는 스택에 데이터를 할당하고 싶을 때나 항상 고정된 개수의 요소로 이루어진 경우라면 배열이 유용하다.

다음과 같이 대괄호 안에 요소의 타입을 쓰고 세미콜론을 쓴 뒤 요소의 개수를 적는 식으로 배열의 타입을 작성할 수도 있다.

```rust
fn main() {
    // i32: 각 요소의 타입, 5: 배열이 5개 요소를 갖고 있음을 나타낸다.
    let a: [i32; 5] = [1, 2, 3, 4, 5];
    
    // 모두 3으로 채워진 5개 요소를 갖게된다.
    let b = [3; 5]; // [3, 3, 3, 3, 3]
}
```
배열은 스택에 할당될 수 있는 계산 가능한 고정된 크기의 단일 메모리 뭉치이다. 
```rust
fn main() {
    let a = [1, 2, 3, 4, 5];
    
    let first = a[0];
    let second = a[1];
}
```




