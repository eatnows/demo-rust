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

문자열에 입력한 그대로를 출력할 수 있게 `raw text` 를 사용할 수 있다. `r#`
```rust
fn main() {
    print!(r#"c:\thisdrivce\new_drive"#); // raw text
    println!("Let me tell you
    어떤 이야기를
    봅시다");
}
```

'어떤 이야기를' 문장 앞에 공백이 있기 때문에 해당 공백도 같이 출력됨.

```rust
fn main() {
    let my_variable = &9;
    println!("{:p}", my_variable);
}
```

디버그 프린트. p는 포인터의 위치를 보여준다. 그런데 `my_variable = 9` 일 경우는 포인터의 위치를 알 수 없어 오류가 발생하고 
`&` 을 붙여줘야 포인터의 위치를 알 수있다. 

복잡하게 프린트를 할 수도 있다.

Module std::fmt

```rust
fn main() { // {}
    let title = "TODAY'S NEWS";
    println!("{:-^30}", title);
    let bar = "|";
    println!("{: <15}{: >15}", bar, bar); // 패딩을 빈공간으로 왼쪽에 프린트하고 길이15, 오른쪽에도 똑같이
    let a = "SEOUL";
    let b = "TOKYO";
    println!("{city1:-<15}{city2:->15}", city1 = a, city2 = b);
}
```
`<` 왼쪽으로 프린팅, `>` 오른쪽으로 프린팅, `^` 중간으로 프린팅

`-^30` 중간에 `-`을 패딩으로 30자


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


# Function

```rust
fn give_number(one: i32, two: i32) -> i32 {
    one * two
}

fn main() {
    let my_number = give_number(9, 8);
    println!("{my_number}");
}
```

```rust
fn give_number(one: i16, two: i16) -> i16 {
    let multiplied_by_ten = {
        let first_number = 10;
        first_number * one * two
    };

    multiplied_by_ten
}

fn main() {
    let my_number = give_number(9, 1);
    println!("{}", my_number);   // 90
}
```

# mutability and shadowing

mutability 바꿀 수 있는지 없는지. 
Rust 는 immutable by default 이기 떄문에 변수를 만들면 기본적으로 값을 바꿀수가 없다.
만약 변수의 값을 바꾸고 싶다면 `mut` 키워드를 사용해야한다.

```rust
fn main() {
    let mut my_number = 10;
    my_number = 9;
}
```

shadowing 이라는 것은 같은 이름을 다시 쓰는 것을 말함.

```rust
fn main() {
    let my_variable = 10;
    let my_variable = "My variable"; // 새로 만듦
    println!("{}", my_variable);
    // 맨위 my_variable 에는 접촉할 수 없음.
}
```

shadowing 은 원래 있던 변수를 없애버리는 것이 아니라 사용하지 못하게 
잠시 막아버리는 것 shadowing 한 것이 사라지면 다시 원래 변수를 사용할 수 있음

```rust
fn main() {
    let my_variable = 10;
    println!("{}", my_variable); // 10
    {
        // shadowing
        // block 안에서 선언된 변수이기 때문에 block scope 
        let my_variable = "My variable";
        println!("{}", my_variable); // My variable
    }
    println!("{}", my_variable); // 10
}
```

# memory + references

stack memory는 빠르고 그룹처럼 

Rust는 reference 라는 특별하는 pointer를 쓴다. reference는 잠깐 빌리는 것

```rust
fn main() {
    let my_number = 15; // This is an i32
    let single_references = &my_number; // This is an &i32  / reference to my_number
    // reference 가 나오는 함수를 썼는데 그 함수를 다른 reference 나오는 함수를 쓰면 reference to reference (&&) 
    let double_references = &single_references; // This is an &&i32
    let five_references = &&&&&my_number; // This is an &&&&&i32
}
```

my_number가 데이터를 가지고있고 single_references 가 my_number 데이터를 볼수 있다. (그 데이터가 어디있는지 안다.)


# String and &str

```rust
fn main() { // {}
    // String = Sized type : 어떤 사이즈인지 알 수 있음. 데이터가 주로 힙에 있고, 얼마나 큰지 알 수 있음.
    // str = dynamic type : &없이 그냥 str. 컴파일러가 스택에 얼마나 큰 지 알아야 하는데 알수가 없으니까 & 를 통해서 스택에서는 데이터의 크기를 알 수 있다.
    let my_name = "David"; // &str
    let my_name2 = "David".to_string(); // String
    let other_name = String::from("David");

    // growable + shrinkable
    let mut my_other_name = "David3".to_string(); // 커지고 작아질 수 있는 타입
    my_other_name.push('!'); // String 타입일 때만 추가가능. &str은 안 됨
    println!("{}", my_other_name);

    // String
    // &str ref str "string slice"
}
```

```rust
fn main() { // {}
    // reallocation
    // 16비트 공간에 계속 뭘 추가하여 16비트를 넘어가면 2배씩 큰 공간에다가 복사하여 붙여넣기(재할당) 한다. -> reallocation

    // String
    // .capacity
    // .push
    // .push_str
    // .pop
    // with_capacity
    // allocation

    let mut my_name = "".to_string();
    println!("Length is {} and Capacity is: {}", my_name.len(), my_name.capacity());
    // my_name.push('!'); // 한글자만 가능
    my_name.push_str("David!");
    println!("Length is {} and Capacity is: {}", my_name.len(), my_name.capacity());
    my_name.push_str(" and I live in seoul");
    println!("Length is {} and Capacity is: {}", my_name.len(), my_name.capacity());
    // print
    // 0 0
    // 6 8
    // 26 26


    let mut my_name = String::with_capacity(26); // 처음부터 26크기를  가지고 시작함
    println!("Length is {} and Capacity is: {}", my_name.len(), my_name.capacity());
    // my_name.push('!'); // 한글자만 가능
    my_name.push_str("David!");
    println!("Length is {} and Capacity is: {}", my_name.len(), my_name.capacity());
    my_name.push_str(" and I live in seoul");
    println!("Length is {} and Capacity is: {}", my_name.len(), my_name.capacity());
    // print
    // 0 26
    // 6 26
    // 26 26
    // 초과하면 2배가 되어 52로 allocation 됨
}
```


# const and static

기본적으로 변수는 {} 안에 선언되면 {}안에서의 라이프사이클을 가진다. 

global 변수는 2가지 방법으로 만들 수 있다.

```rust
// const
// const 를 만들 때는 어떤 타입인지 명시를 해주어야 함.
const NUMBER: i32 = 20;
const HIGH_SCORE: i32 = 20;  // global scope
// const 키워드는 아무곳에서나 사용이 가능. main() 에서도 사용이 가능하나 소용이 없다.

// static
// 같은 메모리 공간을 쓰는 보장이 있다.
static LOW_SCORE: i32 = 0; 
// static mut LOW_SCORE: i32 = 0; // unsafe

// lifetime
// 'static lifetime : 프로그램의 처음부터 끝까지 살 수 있음
 

// 대문자로 변수명을 하지 않으면 컴파일러가 warning을 보내는데 그걸 없애줌
// attribute 라고한다.
#[allow(non_upper_case_globals)]
const high_score: i32 = 10;

fn main() {
    let x = 8; // 'let' binding: i32
    let my_name = "David"; // $'static str  : '가 하나있으면 static식으로 오래살 수 있다는 뜻.
    
}
```

```rust
static mut LOW_SCORE: i32 = 0;

fn main() {
    unsafe { LOW_SCORE = 1; }
}
```

`static` 는 `mut` 키워드를 통해서 선언된 변수의 값을 `unsafe` 키워드로 변경시킬 수 있다. 
하지만 안 쓰는 것이 좋다.


# returning a reference 
OWNERSHIP - 소유권

어떤 변수가 자기 데이터를 언제까지 해줄 수 있는지, 누가 데이터를 가지는지에 대한 개념.

```rust
fn main() {
    let country = String::from("대한민국");
    let ref_one = &country;
    let ref_two = &country;

    println!("Country is: {}", ref_one); // 대한민국
}
```

```rust
// 'static : 계속 존재할 거다
fn return_it() -> &'static String {
    let country = String::from("대한민국");
    &country // return &country
}

// & = reference
fn main() {
    let my_country = return_it();
}
```

`return_it()` 에서 `'static` 키워드를 사용해 static lifetime 이라고 명시를 해줘도 해당 코드는 실행이 되지 않는데, 그 이유는

`return_it()` 안에서 선언된 `country` 는 `return_it()`의 `{}` 내에서만 살게되고 함수가 끝날 때 `country` 가 소멸되기 떄문에 
그 `country`의 reference 를 반환 받는 `main()`의 my_country 는 안전하지 않기 떄문에 실행되지 않는다.

# Mutable reference

`&` immutable reference / shared reference

`&mut` mutable reference / unique reference

```rust
// & : referencing
// * : dereferencing : value 자체
// && **
fn main() {
    let mut my_number = 9;
    let num_ref = &mut my_number; // num_ref가 my_number 데이터를 바꿀 수 있다.

    *num_ref = 10;

    println!("Number is now {}", my_number);
}
```

&mut &mut 과 같이 연속해서 사용하면 *도 연속해서 사용을 해주어야 한다.

# references and shadowing

```rust
fn main() {
    let mut number = 10;
    let number_ref = &number;
    let number_change = &mut number;
    *number_change += 10;
    println!("{}", number_ref);
}
```
위 코드는 에러가 난다. 

```rust
fn main() {
    let mut number = 10;
    let number_change = &mut number;
    *number_change += 10;
    let number_ref = &number;
    println!("{}", number_ref);
}
```
이 코드에 경우 예전 러스트 버전이였으면 에러가 발생하지만 컴파일러가 계속 개발되어 현재는 정상동작한다.

```rust
fn main() {
    // shadowing
    let country = "대한민국"; // 사라진게 아니다.
    let country_ref = &country; // 원래 만들었던 대한민국 변수의 데이터를 바라보고있음.
    let country = 8;    // country 만 쓰면 컴파일러가 해당 값만 가져옴 // shadowing
    println!("{}, {}", country_ref, country); // 대한민국, 8
}
```

# references in functions

```rust
// OWNERSHIP
// move semantics
fn print_country(country_name: String) { // ref 가 아닌 String
    println!("My country is {}", country_name);
}

fn main() {
    let country = "대한민국".to_string();
    print_country(country); // 이 function이 country에 소유권을 가지게 된다. 그래서 이제 이 main 함수에서는 country를 사용할 수 없게 됨
    print_country(country); // 에러 발생!
}
```

print_country에 파라미터가 $str이 아닌 String이기 때문에 main 함수에서 country를 인자로 넣어주면 
country 변수의 소유권이 print_country 함수로 넘어간다.
country 변수는 print_country 함수의 스코프 내에서만 사용이 가능해져 
main 함수에서 country 변수를 사용할 수 없기 떄문에 두번째 호출에서 에러가 발생한다.

```rust
// OWNERSHIP
// move semantics
fn print_country(country_name: &String) { // ref 가 아닌 String
    println!("My country is {}", country_name); // country 라는 변수의 value를 잠깐 보는것
}

fn main() {
    let country = "대한민국".to_string();
    print_country(&country); // reference만 주는것. 소유권이 넘어가지 않음
    print_country(&country); // 정상 출력
}
```
# mutable references and mut in functions

```rust
fn add_is_great(country_name: &mut String) {
    country_name.push_str(" is great!");
    println!("Now it says: {}", country_name);
}

fn main() {
    let mut my_country = "캐나다".to_string();
    // add_is_great(my_country); // by value
    // add_is_great(&my_country); // by reference
    add_is_great(&mut my_country); // by mutable reference
    add_is_great(&mut my_country);
}
```

```rust
fn add_is_great(mut country_name: String) { // take by value, declare as mutable
    country_name.push_str(" is great!");
    println!("Now it says: {}", country_name);
}

fn main() {
    let my_country = "대한민국".to_string(); // 바꿀수가 없음 mut 이 아님
    add_is_great(my_country); // mut 파라미터에 넣어줬는데 잘됨
}
```

my_country 가 먼저 소유권을 가지고 있고 add_is_great가 value의 소유권을 가지고 있으니까
이제 main에서 my_country는 상관없고 add_is_great()가 country_name의 소유권자이기 때문에 mut으로 사용할 수도 있음. reference와 상관없음.

# copy and clone

```rust
// It's trivial to copy the bytes

// Ownership and copy types
fn prints_number(number: i32) {
    println!("{}", number);
}

fn prints_string(input: String) {
    println!("{}", input);
}
// copy - copy types
// clone - non-copy types
fn main() {
    let my_number = 8;
    prints_number(my_number);
    prints_number(my_number);

    let my_country = "Austria".to_string();
    prints_string(my_country.clone());  // 소유권이 넘어가 실행되지 않기 때문에 처음 사용 시 clone을 사용하게 끔 함
    prints_string(my_country);
}
```

copy type이란 `i32` `char` 이런 것들. owner ship 이랑 상관없이 사용할 수 있음.

copy type이 아니면 my_number를 prints_number가 사용하면 소유권이 넘어가서 2번호출을 할 수 없지만 
copy type이면 자동적으로 copy가 된다. (prints_number의 인자로 들어가는 my_number가 copy되어 인자로 사용된다.)

# uninitialized variables and for loops

```rust
// uninitialized variable
// control flow

// possibly uninitialized = maybe doesn't have a value yet 
fn loop_then_return(mut counter: i32) -> i32 { // mut를 앞에 쓰면 소유권을 받아서 mutable하게 쓰겠다.
    loop {
        counter += 1;
        if counter % 50 == 0 {
            // 102 / 50 -> 2 remainder 2
            break;
        }
    }

    counter
}

fn main() {
    let my_number;

    {
        //복잡한 것들
        let x = loop_then_return(43);
        my_number = x
    };

    println!("{}", my_number);
}
```

Rust 에서는 초기화를 하지 않고 변수를 선언만 하는 것을 지양한다고 함. 위 코드같은 스타일에 코드는 러스트스럽지 않다고함.

# arrays

```rust
// Collection types
// Array

// &str
fn main() {
    let array = ["One", "Two"]; // [&str; 2]
    let array2 = ["One", "Two", "Five"]; // array와 array2는 다른타입이다. // [&str; 3]

    println!("Is array the same as array2? {}", array == array2);
}
```

[] 를 이용해서 array를 만든다. rust에서는 배열에 들어가는 타입하고 크기에 따라서 아예 다른 타입이 된다.
array는 그대로 추가할 수가 없고, 삭제할 수도 없는 기본적인 타입

```rust
fn main() {
    let array = [0; 640]; // 0을 넣고 640번 해주세요.

    println!("{:?}", array)
}

fn main() {
    let array = ["1월", "2월"]; // indexing

    println!("{}", array[0]) // zeroth
}
```
```rust

// buffer
fn main() {
    let array = ["1월", "2월"]; // indexing

    println!("{:?}", array.get(1)) // zeroth
    // Some(있으면) None(없으면) Option enum
}
```

`array[0]` 과 같이 사용할 수있고 `array.get(1)` 이런식으로 array의 값을 가지고 올 수도 있다. 


# slices




