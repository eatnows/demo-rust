# Comment

1. `//` 는 주석을 작성할 때 사용함
2. `///` 는 문서화를 할 때 사용함
3. `/* */` 도 사용할 수 있음. 아래와 같은 코드도 사용 가능.
```rust
let x/*: i16*/ = 10;
```

러스트의 경우 사용하지 않는 변수가 있으면 컴파일러에서 warning을 보내주는데, 
변수 이름앞에 `_`가 있으면 컴파일러에게 이 변수는 무시하라고 전할 수 있음.
```rust
let _apple: i16 = 10;
```

# Integer

u는 Unsigned, i는 signed integer를 의미함.

```text
i8, i16, i32, i64, i128, and isize. 
u8, u16, u32, u64, u128, and usize.
숫자는 bits.  
8 bit = 1 byte
```

iszie, usize는 computer architecture에 따라 다르다. (32비트, 64비트)

만약 32비트 컴퓨터라면 isize가 i32, 64비트 라면 i64 

```rust
fn main() {
    let number = 100;
}
```

위 코드 number 와 같이 어떤 타입인지 명시해주지 않는다면 `i32`를 기본값으로. (어떤 컴퓨터 아키텍처에서도 사용할 수 있음)

```rust
fn main() {
    let number: u8 = 100; // 255
    let two_number = 50; // i32
    let three_number = number + two_number;
    
    // type inference
}
```
u8 + i32 와 같이 다른 타입을 더하기할 수는 없다. two_number와 같이 명시적으로 타입을 지정하지 않았기 때문에 컴파일러가 알아서 타입을 추론해주지만
명시적으로 `u8` + `u16` 과 같은 다른 타입을 더하면 `mismatched types` 에러가 발생한다.

```rust
fn main() {
    let number: u8 = 100; // 255
    let two_number = 156; // i32
    let three_number = number + two_number;

    println!("{}", three_number);
}
```

이 코드와 마찬가지로 u8로 컴파일러가 `three_number` 변수의 타입을 추론하였는데 `u8` 이 담을 수 있는 범위를 넘긴다면 에러가 발생한다.

```text
error: this arithmetic operation will overflow
 --> src/main.rs:4:24
  |
4 |     let three_number = number + two_number;
  |                        ^^^^^^^^^^^^^^^^^^^ attempt to compute `100_u8 + 156_u8`, which would overflow
  |
  = note: `#[deny(arithmetic_overflow)]` on by default
```

# Char

string은 무조건 `""` 를 사용해야함. char는 `''` 를 사용해야한다.
char는 유니코드를 사용할 수 있음. 모든 char 의 크기는 4 bytes 이다.

```rust
fn main() {
    // casting = simple type change using 'as'

    let first_number: u16 = 8;
    let second_number: u8 = 10;
    let third_number = first_number + second_number as u16;

    println!("{}", third_number);
}
```

ASCII는 255개 숫자이기 때문에 u8을 쓰면 안전하게 사용할 수 있음 (캐스팅 할 수 있음)

```rust
fn main() {
    let my_number = 'a' as u8;

    println!("Hello, world! my number is {}", my_number); // 97
}
```
char는 무조건 4 bytes 이고 string은 

```rust
fn main() {
    println!("Size of a char: {}", std::mem::size_of::<char>());

    println!("Size of string containing 'a': {}", "a".len()); // ascii 1byte에 들어갈 수 있으니까 u8 = 1byte = 8 bits
    println!("Size of string containing 'ß': {}", "ß".len()); // 2 bytes
    println!("Size of string containing 'ß': {}", "ßßßßß".len()); // 10 bytes
    println!("Size of string containing '国': {}", "国".len()); // 3 bytes
    println!("Size of string containing '𓅱': {}", "𓅱".len()); // 4 bytes
}
```

rust에서는 len() 라는 것은 byte를 말하는 것이다. 그럼 글자수는 어떻게??

```rust
fn main() {
    let slice = "Hello";
    println!("Slice is {} bytes and also {} characters", slice.len(), slice.chars().count());
    let slice2 = "안녕!";
    println!("Slice2 is {} bytes but only {} characters.", slice2.len(), slice2.chars().count());
}
```

`chars()` 를 사용하면 문자열을 쪼개는 식으로 사용하게 됨. 그 상태에서 `count()` 로 개수를 셀 수 있음.
```text
"Hello there" -> ('H', 'e', 'l' ..).count()
```


# Float

데이터의 변수를 선언할 때 변수명 옆에 데이터의 타입을 적어줘야하는데, 숫자의 경우 값뒤에 데이터 타입을 적어줘도 된다.

```rust
fn main() {
    let number1: u8 = 10;
    let number2 = 10u8;
    let number3 = 10_u8;
    let number4 = 10_______u8; // _ 는 컴파일러가 따로 인식하지 않는다.
    let number5: u8 = 1_0;

    let number6 = 9.;
}
```

float은 소수점 이하 자리를 따로 추가하지 않고 `.` 찍어도 float 으로 인식한다. 

float의 데이터 타입은 f32와 f64 두가지이다. `.` 만 찍어서 선언할 경우 f64로 타입을 추론한다.

```rust
fn main() {
    let number = 9.;   // float f64로 인식 
    
    let number2 = 9.6; 
    let number3 = 9; 
    
    println!("{}", number2 as i32 + number3);
}
```

number2를 i32로 캐스팅할 경우 소수점 이하 자리는 날라간다.


# println!

macro 는 슈퍼펑션 같은 느낌. 코드를 만드는 펑션. 컴파일러만 보는 코드를 만드는 느낌.

macro = function that writes code

```rust
fn main() {
    let my_name = "New";
    println!("My name is {} and my age is {}", my_name, give_age());
    // {} 만 있고 , 뒤에 인자를 넣어주지 않으면 에러가 발생한다.
}

// return을 명시적으로 적어줘도 되지만 마지막 라인에 나오는 숫자나 변수를 자동으로 리턴해준다.
// 42 끝에 ; 를 붙이면 에러가 발생한다.
fn give_age() -> i32 {
    42
}
```

1.58 버전부터 아래와 같이 인자를 넣어줘도 사용이 가능하다.
```rust
fn main() {
    // 1.58 버전부터 사용 가능
    let my_name = "New";
    let my_age = 22;
    println!("My name is {my_name} and my age is {}", my_age);

    // 이런식으로 {} 안에 넣고 변수를 연결시켜줄 수도 있다.
    // string interpolation
    let my_name = "New";
    let my_age = 22;
    println!("My name is {name} and my age is {age}", 
    name = my_name,
    age = my_age
    );

    // 이름이 아닌 숫자를 넣어줘도 된다.
    let my_name = "New";
    let my_age = 22;
    println!("My name is {0} and my age is {1}",
         my_name,
         my_age
    );


    // 이런건 아직 안 되는듯
    let my_name = "New";
    let my_age = 22;
    println!("My name is {my_name} and my age is {my_age + 2}"); // error
    fn give_age() -> i32 {
        42
    }
    println!("My name is {my_name} and my age is {give_age()}"); // error
}
```


# semicolon

세미콜론은 특별한 뜻이 있다.

() - empty tuple, Rust에서는 unit type이라고 한다. 다른 언어에서는 (void) 라고 한다.

```rust
// expression-based language
fn empty_tuple() -> () {
    8;  // 세미콜론이 있으면 empty tuple을 반환. 사실 없어도됨
}

fn empty_tuple() {
    
}
```

세미콜론이 있으면 empty tuple이 나온다고 생각하면 된다.

```rust
fn empty_tuple() {
    
}

// 사실 main 함수도 empty tuple 이기 때문에 empty tuple을 반환하는 것은 된다.
fn main() {
    let tuple = empty_tuple();
    tuple // ; 있어도됨 
}
```

main 함수도 empty tuple 이기 떄문에 empty tuple을 return 하는 것은 된다. 
하지만 empty tuple이 아닌 다른 데이터를 리턴하면 에러가 발생한다.

```rust
fn main() {
    8 // error 발생, 하지만 8; 리턴하면 empty tuple 리턴하는 것이기 떄문에 에러가 나지 않는다.  
}
```

```rust
fn empty_tuple() {

}

// Display print = {}

fn main() {
    let tuple = empty_tuple();
    // Debug print
    println!("{:?}", tuple);
}
```

{}로 하면 안되지만 {:?} 하게되면 Debug print로 타입과 프로퍼티를 그대로 프로그래머전용 print 라고 보면된다.









