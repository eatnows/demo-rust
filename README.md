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

# 함수
새로운 함수를 선언하려면 `fn` 키워드를 사용해야한다. 러스트는 함수나 변수 이름의 관례로 `snake case` 방식을 이용을 한다.

```rust
fn main() {
    println!("Hello, world");
    
    another_function();
}

fn another_function() {
    println!("Another function.");
}
```
러스트에서는 `fn` 뒤에 함수 이름과 괄호를 붙여 함수를 정의한다. 중괄호는 함수 본문의 시작과 끝을 컴파일러에게 알려준다.

러스트는 함수 위치를 고려하지 않으며, 호출하는 쪽에서 볼 수 있는 스코프 어딘가에 정의만 되어있으면 된다.

## 매개변수 (parameter)
함수는 매개변수를 갖도록 정의될 수 있으며, 함수 시그니처 (function signature) 의 일부분인 특별한 변수이다. 
함수가 매개변수를 갖고 있으면 이 매개변수에 대한 구체적인 값을 전다할 수 있다. 엄밀하게는 이러한 구체적인 값을 인수 (argument) 라고 부른다.

```rust
fn main() {
    another_function(5, 'h');
}

fn another_function(x: i32, unit_label: char) {
    println!("The measurement is: {}{}", x, unit_label);
}
```
함수 시그니처에서는 각 매개변수의 타입을 반드시 선언해야 한다. 함수의 정의에 타입 명시를 강제하면 이 함수를 다른 코드에서 사용할 때 개발자 의도한 타입을 컴파일러가 추측하지 않아도 된다.

## 구문과 표현식
함수 본문은 필요에 따라 표현식 (expression) 으로 끝나는 구문 (statement) 의 나열로 구성된다.
러스트는 표현식 기반의 언어이므로, 구문과 표현식의 구분은 러스트를 이해하는데 중요하다.
- 구문(statement): 어떤 동작을 수행하고 값을 반하지 않는 명령이다.
- 표현식(expression): 결과값을 평가한다.

`let` 키워드로 변수를 만들고 값을 할당하는 것은 구문이다.

```rust
fn main() {
    let x = 7;
    // let y = (let x = 7); // 오류 발생 // y에 바인딩 시킬 값이 존재하지 않는다.
}
```
함수 정의도 구문이다. 구문은 값을 반환하지 않는다. `let` 구문은 다른 변수에 할당하려고 하면 에러가 발생한다.
작성하는 러스트 코드의 대부분은 표현식이며, 이는 어떤 값을 평가한다. 표현식은 구문의 일부일 수 있다.
`let y = 7;` 이라는 구문에서 7은 7이라는 값을 평가하는 표현식이다. 함수를 호출하는 것도, 매크로를 호출하는 것도 표현식이다.
```rust
fn main() {
    let y = {           // 중괄호로 만들어진 새로운 스코프 블록도 표현식이다.
        let x = 3;
        x + 1       // 표현식은 종결을 나타내는 세미콜론을 쓰지 않음.
    };
    
    println!("The value of y is: {}", y)
}
```
표현식은 종결을 나타내는 세미콜론을 쓰지 않는다. 만약 표현식 끝에 세미콜론을 추가하면, 표현식은 구문으로 변경되고 값을 반환하지 않게 된다.

## 반환 값을 갖는 함수
함수는 호출한 코드에 값을 반환할 수 있다. 그 값의 타입은 `->` 뒤에 선언되어야 한다. 러스트에서 함수의 반환 값은 함수 본문의 마지막 표현식의 값과 동일하다.
`return` 키워드와 값을 지정하여 함수로부터 일찍 값을 반환할 수 있지만, 대부분 함수들은 암묵적으로 마지막 표현식 값을 반환한다.

```rust
fn five() -> i32 {
    5
}

fn main() {
    let x = five();

    println!("The value of x is: {}", x);
}
```

5는 five 함수의 반환 값이며, 이 떄문에 반환 타입을 `i32` 로 설정한 것이다. `;` 로 함수의 마지막 표현식을 구문으로 만든다면 값이 반환되지 않고,
구문은 값을 평가하지 않기 때문에 `()` 로 표현되는 유닛 타입으로 표현된다.


# 제어 흐름문
러스트 코드의 실행 흐름을 제어하도록 해주는 가장 일반적인 재료는 `if` 표현식과 반복문이다.

## if 표현식
`if` 표현식은 여러분의 코드가 조건에 따라 분기할 수 있도록 해준다.

```rust
fn main() {
    let number = 6;
    
    if number % 4 == 0 {
        println!("4로 나누어지는 수");
    } else if number % 3 == 0 {
        println!("3으로 나누어지는 수");
    } else if number % 2 == 0 {
        println!("2로 나누어지는 수");
    } else {
        println!("4, 3, 2로 나누어지지 않는 수");
    }
}
```

모든 `if` 표현식은 `if` 라는 키워드로 시작하고 그 뒤에 조건이 온다. 
조건식은 반드시 `bool` 타입이어야 한다. 조건식이 `bool` 이 아니라면 에러가 발생한다.
Ruby나 JavaScript와 같은 언어와 달리 러스트는 boolean 타입이 아닌 값을 boolean 타입으로 자동 변환하지 않는다.
`else if` 표현식을 너무 많이 사용하면 코드가 복잡해질 수 있으므로, 표현식이 두 개 이상이면 코드를 리팩터링하는 것이 좋다.

`if` 는 표현식이기 떄문에 `let` 구문의 우변에 사용할 수 있다.
```rust
fn main() {
    let condition = true;
    let number = if condition { 5 } else { 6 };     // if 표현식을 계산한 결과값이 바인딩 될 것이다.
    println!("The value of number is : {}", number);
}
```

코드 블록은 블록 안의 마지막 표현식을 계산하고, 숫자는 그 자체로 표현식이다. `if` 표현식의 각 갈래의 결과값은 같은 타입이어야 한다.

## 반복문을 이용한 반복
러스트는 몇 가지 반복문(loop)을 제공하는데 이는 루프 본문의 시작부터 끝까지 수행한 뒤 다시 처음부터 수행한다.
러스트에는 `loop`, `while` 그리고 `for` 라는 세 종류의 반복문이 있다.

### loop로 코드 반복하기
`loop` 키워드는 그만두라고 명시하기 전까지 영원히 코드 블록을 반복 수행한다.
```rust
fn main() {
    let mut counter = 0;
    
    let result = loop {
        counter += 1;
        
        if counter == 10 {
            break counter * 2;
        }
    };
    
    loop {
        println!("The result is {}!", result);
    }
}
```
루프 안에 `break` 키워드를 집어 넣으면 루프를 멈춰야 하는 시점을 프로그램에게 알려줄 수 있다.
`break` 표현식 뒤에 반환하고자하는 값을 넣으면 연산의 결과를 이후에 코드에 전달할 수 있다.

만일 루프 안에 루프가 있다면, `break`와 `continue`는 해당 지점의 바로 바깥쪽 루프에 적용된다.
루프에 루프 라벨 (loop label)을 추가적으로 명시하면 `break` 나 `continue` 와 함께 이 키워드들이 
바로 바깥쪽 루프 대신 라벨이 적힌 특정한 루프에 적용되도록 할 수 있다. **루프 라벨은 반드시 작은 따옴표로 시작해야 한다.**

```rust
fn main() {
    let mut count = 0;
    
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;
        
        loop {
            println!("remaining = {}", remaining);
            if remaining == 9 { 
                break;
            }
            if count == 2 { 
                break 'counting_up;
            }
            remaining -= 1;
        }
        
        count += 1;
    }
    println!("End count = {}", count);
}
```

### while을 이용한 조건 반복문
```rust
fn main() {
    let mut number = 3;

    while number != 0 {
        println!("{number}")
        
        number -= 1;
    };
    println!("LIFTOFF")
}
```
이 구조는 `loop`, `if`, `else`, `break` 를 사용할 때 필요하게 될 많은 중첩 구조를 제거하고 코드를 더 깔끔하게 만든다.
조건식이 `true`로 계산되는 동안 코드가 실행되고, 그렇지 않으면 반복문을 벗어난다.

### for를 이용한 컬렉션에 대한 반복문
```rust
fn main() {
    let a = [10, 20, 30, 40, 50];
    
    for element in a {
        println!("the value is: {element}");
    }
}
```
`for` 루프를 사용하면 여러분이 배열 내 값의 개수를 변경시키더라도 수정해야 할 다른 코드를 기억해둘 필요가 없어진다.
표준 라이브러리가 제공하는 `Range` 타입을 이용하면 특정 횟수만큼의 반복문을 구현할 수 있는데, 
`Range`는 어떤 숫자에서 시작하여 다른 숫자 종료 전까지의 모든 숫자를 차례로 생성해준다.

```rust
fn main() {
    for number in (1..4).rev() { // rev 메서드는 범위값을 역순으로 만들어준다.
        println!("{}", number);
    }
    println!("LIFTOFF!!!");
    // 3! 2! 1! LIFTOFF!!!
}
```

# 소유권 (Ownership)
소유권은 러스트 프로그램의 메모리 관리법을 지배하는 규칙 모음이다. 모든 프로그램은 작동하는 동안 컴퓨터의 메모리 사용 방법을 관리해야 한다.
몇몇 언어는 가비지 컬렉션 방식과 프로그래머가 직접 명시적으로 메모리를 할당하고 해제하는 방식을 택했다.
러스트는 제 3의 방식을 택했다. 소유권이라는 시스템을 만들고, 컴파일러가 컴파일 중에 검사할 여러 규칙을 정해 메모리를 관리하는 방식이다.
이 규칙을 하나라도 위반하면 프로그램은 컴파일되지 않는다.

## 스택 영역과 힙 영역
러스트 같은 시스템 프로그래밍 언어에서는 값을 스택에 저장하느냐 힙에 저장하느냐의 차이가 프로그램의 동작 및 프로그래머의 의사결정에 큰 영향을 미친다.
스택, 힙 둘 다 프로그램이 런타임에 이용하게 될 메모리 영역이라는 공통점이 있지만 구조는 각기 다르다.

스택은 값이 들어온 순서대로 저장하고 역순으로 제거한다. `Last in first out`.
스택에 데이터를 추가하는 행위를 푸시 (push) 라고 하며, 반대로 스택에서 데이터를 제거하는 행위는 팝 (pop) 이라 한다.
**스택에 저장되는 데이터는 모두 명확하고 크기가 정해져 있어야 한다.** 
컴파일 타임에 크기를 알 수 없거나, 크기가 변경될 수 있는 데이터는 스택 대신 힙에 저장되어야 한다.

데이터를 힙에 넣을 때 먼저 저장할 공간이 있는지 운영체제에 물어본다. 그럼 메모리 할당자는 커다란 힙 영역에서 어떤 빈 지점을 찾고,
이 지점은 사용중이라고 표시한 뒤 해당 지점을 가리키는 포인터 (pointer)를 우리한테 반환한다. 
이 과정을 힙 공간 할당 (`allocating on the heap`), 줄여서 할당 (allocation) 이라 한다.
(스택에 값을 푸시하는 것은 할당이라 부르지 않는다.)
포인터는 크기가 정해져 있어 스택에 저장할 수 있으나, 포인터가 가리키는 실제 데이터를 사용하고자 할 때는 포인터를 참조해 해당 포인터가 가리키는 위치로 이동하는 과정을 거쳐야 한다.

스택 영역은 데이터에 접근하는 방식상 힙 영역보다 속도가 빠르다. 메모리 할당자가 새로운 데이터를 저장할 공간을 찾을 필요가 없이 항상 스택의 가장 위에 데이터를 저장하면 되기 때문이다.

힙 영역은 포인터가 가리키는 곳을 찾아가는 과정으로 인해 느려진다. 현대 프로세서는 메모리 내부를 이리저리 왔다 갔다 하는 작업이 적을수록 속도가 빨라지는데,
힙에 있는 데이터들은 서로 멀리 떨어져 있어 프로세서가 계속해서 돌아다녀야 하기 때문이다. 
힙 영역처럼 데이터가 서로 떨어져 있으면 작업이 느려지고, 반대로 스택 영역처럼 데이터가 서로 붙어 있으면 작업이 빨라진다.

함수를 호출하면 호출한 함수에 넘겨준 값(값 중엔 힙 영역의 데이터를 가리키는 포인터도 있을 수 잇다.)과 해당 함수의 지역 변수들이 스택에 푸시가 된다.
그리고 이 데이터들은 함수가 종료될 때 팝된다.

### 소유권 규칙
소유권의 주요 목표가 힙 데이터의 관리라는 점을 알고 있으면 소유권의 동작 방식을 이해하는 데 도움이 된다.
- 러스트에서 각각의 값은 소유자(owner)가 정해져 있다.
- 한 값의 소유자는 동시에 여럿 존재할 수 없다.
- 소유자가 스코프 밖으로 벗어날 때, 값은 버려진다. (dropped)

### 변수의 스코프
```rust
fn main() {
    {                       // s는 아직 선언되지 않아서 여기서는 유효하지 않는다. 
        let s = "hello";    // 이 지점부터 s가 유효하다.
        // s로 무슨 작업을 함...
    }                       // 이 스코프가 종료되었고, s는 더 이상 유효하지 않는다.
}
```
1. `s`가 스코프 내에 나타나면 유효하다
2. 유효기간은 스코프 밖으로 벗어나기 전까지이다.

### String 타입
여태 보아온 문자열은 코드 내에 하드코딩하는 방식의 문자열 리터럴이었다. 
문자열 리터럴이 불변성 (immutable) 을 지니기에 변경할 수 없다는 점과, 프로그램에 필요한 모든 문자열을 프로그래밍하는 시점에 알 수 없다는 점 때문에 문자열 리터럴은 편리하지만 만능은 아니다.
러스트는 또 다른 문자열 타입인 `String`을 제공한다. 이 타입은 힙에 할당된 데이터를 다루기 때문에, 컴파일 타임에 크기를 알 수 없는 텍스트도 저장할 수 있다.
```rust
fn main() {
    let s = String::from("Hello");   
    
    // String 문자열은 변경이 가능하다.
    s.push_str(", world!"); // push_str()이 문자열에 리터럴을 추가한다.

    println!("{}", s);
}
```
이중 콜론 `::`은 우리가 함수를 사용할 때 `string_from` 같은 함수명을 사용하지 않고 
`String` 타입에 있는 특정된 `from` 함수라는 것을 지정할 수 있게 해주는 네임스페이스 연산자이다. 

### 메모리 할당
문자열 리터럴은 컴파일 타임에 내용을 알 수 있으므로, 텍스트가 최종 실행파일에 하드코딩된다. 문자열이 변하지 않을 경우에만 사용할 수 있다.
반면 `String` 타입은 힙에 메모리를 할당하는 방식을 사용하기 때문에 텍스트 내용 및 크기를 변경할 수 있다.
- 실행 중 메모리 할당자로부터 메모리를 요청해야 한다.
- `String` 사용을 마쳤을 때 메모리를 해제할 (즉, 할당자에게 메모리를 반납할) 방법이 필요하다.

러스트에서는 변수가 자신이 소속된 스코프를 벗어나는 순간 자동으로 메모리를 해제하는 방식으로 해결했다.

```rust
{
    let s = String::from("hello");  // s는 이 지점부터 유효하다.

}                                   // 이 스코프가 종료되엇고, s는 더이상 유효하지 않는다. 
```
러스트는 변수가 스코프 밖으로 벗어나면 `drop` 이라는 특별한 함수를 호출한다. 이 함수는 해당 카입을 개발한 개발자가 직접
메모리 해제 코드를 작성해 넣을 수 있게 되어있으며, 위의 경우 `String` 개발자가 작성한 메모리 해제 코드가 실행된다. **`drop`은 닫힌 중괄호 `}`가 나타나는 지점에서 자동으로 호출된다.**

### 변수와 데이터 간 상호작용 방식: 이동
러스트에서는 동일한 데이터에 여러 변수가 서로 다른 방식으로 상호작용할 수 있다.
```rust
fn main() {
    let x = 5;
    let y = x;    
}
```
정수형 값은 크기가 정해진 단순한 값이기 때문에 두 `5` 값은 스택에 푸시가 된다.

```rust
fn main() {
    let s1 = String::from("hello");
    let s2 = s1;
}
```
이전 코드와 매우 비슷하여 동작 방식도 같은 것 같지만 이번에는 전혀 다른 방식으로 동작한다.
`String`은 
1. 문자열 내용이 들어있는 메모리를 가리키는 포인터
2. 문자열 길이
3. 메모리 용량
이렇게 세 부분으로 이루어져 있다. 이 데이터는 스택에 저장이 된다.
문자열 길이는 `String`의 내용이 현재 사용하고 있는 메모리를 바이트 단위로 나타낸 것이고,
메모리 용량은 메모리 할당자가 `String`에 할당된 메모리의 양을 뜻한다.

`s2`에 `s1`을 대입하면 `String` 데이터가 복사된다. 이때 데이터는 스택에 있는 데이터로 즉 포인터, 길이, 용량 값을 말하며 포인터가 가리키는 힙 영역의 데이터는 복사되지 않는다.

앞선 내용 중 변수가 스코프를 벗어날 때 러스트에서 자동으로 `drop` 함수를 호출하여 해당 변수가 사용하는 힙 메모리를 제거한다는 내용이 있었다. 만약 두 포인터가 같은 곳을 가리키는 경우에는 어떻게 될까?
`s2`, `s1`이 스코프 밖으로 벗어날 때 각각 메모리를 해제하게 되면 중복 해제 (double free) 에러가 발생한다. 이는 메모리 안정성 버그 중 하나이며, 보안을 취약하게 만드는 메모리 손상의 원인이다.

메모리 안정성을 보장하기 위해서 러스트는 `let s2 = s1;`라는 뒤로는 `s1`이 더 이상 유효하지 않다고 판단한다. `s1`이 스코프를 벗어난다 하더라도 아무것도 해제할 필요가 없어진다.

다른 프로그래밍 언어에서 얕은 복사 (shallow copy), 깊은 복사 (deep copy) 라는 용어를 들어보셨다면, 힙 데이터를 복사하지 않고 포인터, 길이, 용량 값만 복사하는 것을 얕은 복사라고 
생각할 수 있지만 러스트에서는 기존의 변수를 무효화하기 때문에 이를 얕은 복사가 아닌 이동 (move) 라고 하고, 앞선 코드는 `s1`이 `s2`로 이동되었다 라고 표현한다.
러스트는 절대 자동으로 깊은 복사로 데이터를 복사하는 일이 없다. 따라서, 러스트가 자동으로 수행하는 모든 복사하는 런타임 성능 측면에서 효율적이라 할 수 있다.

### 변수와 데이터 간 상호작용 방식: 클론
`String`의 힙 데이터까지 깊이 복사하고 싶을 땐 `clone` 이라는 공용 메서드를 사용할 수 있다.
```rust
fn main() {
    let s1 = String::from("hello");
    let s2 = s1.clone();

    prinlnt!("s1 = {}, s2 = {}", s1, s2);
}
```
이 코드에서 `clone` 호출을 보고, 이 지점에서 성능에 영향이 갈 수도 있는 코드가 실행될 것을 알 수 있다., 즉 `clone`은 해당 위치에서 무언가 다른 일이 수행될 것을 알려주는 시각적인 표시이기도 하다.

### 스택에만 저장되는 데이터: 복사
```rust
fn main() {
    let x = 5;
    let y = x;
    
    println!("x = {}, y = {}", x, y) // x = 5, y = 5
}
```
`clone`을 호출하지도 않았는데, `x`는 계속해서 유효하며 `y`로 이동되지도 않았다. 이유는 정수형 등 컴파일 타임에 크기가 고정되는 타입은 모두 스택에 저장되기 떄문이다.
스택에 저장되니, 복사본을 빠르게 만들 수 있고, 굳이 `y`를 생성하고 나면 `x`를 무효화할 필요가 없다. 다시말해 이런 경우에는 깊은 복사와 얕은 복사간에 차이가 없다.

러스트에는 정수형처럼 스택에 저장되는 타입에 달아 놓을 수 있는 `Copy` 트레이트가 있다. 만약 어떤 타입이 이 `Copy` 트레이트가 구현되어 있다면, 이 타입의 변수는 사용되어도 이동되지 않고 자명하게 복사되고 대입 연산 후에도 사용할 수 있다.
하지만 `Drop` 트레이트가 구현된 경우엔 `Copy` 트레이트를 어노테이션 (annotation) 할 수 없다. 즉 스코프 밖으로 벗어났을 때 특정 동작이 요구되는 타입에 `Copy` 어노테이션을 추가하면 컴파일 에러가 발생한다.

`Copy`가 가능한 타입은 뭘까? 일반적으로 단순한 스칼라 값의 묶음은 `Copy` 가능하고, 할당이 필요하거나 리소스의 일종인 경우엔 불가능하다.
- 모든 정수형 타입 (예: `u32`)
- `true`, `false` 값을 갖는 논리형 `bool`
- 모든 부동소수점 타입 (예: `f64`)
- 문자 타입 `char`
- `Copy` 가능한 타입만으로 구성된 튜플

### 소유권과 함수
함수로 값을 전달하는 매커니즘은 변수에 값을 대입할 때와 유사하다. 함수에 변수를 전달하면 대입 연산과 마찬가지로 이동이나 복사가 일어나기 때문이다.
```rust
fn main() {
    let s = String::from("hello");  // s가 스코프 안으로 들어옵니다

    takes_ownership(s);             // s의 값이 함수로 이동됩니다...
    // ... 따라서 여기서는 더 이상 유효하지 않습니다

    let x = 5;                      // x가 스코프 안으로 들어옵니다

    makes_copy(x);                  // x가 함수로 이동될 것입니다만,
    // i32는 Copy이므로 앞으로 계속 x를
    // 사용해도 좋습니다

} // 여기서 x가 스코프 밖으로 벗어나고 s도 그렇게 됩니다. 그러나 s의 값이 이동되었으므로
// 별다른 일이 발생하지 않습니다.

fn takes_ownership(some_string: String) { // some_string이 스코프 안으로 들어옵니다
    println!("{}", some_string);
} // 여기서 some_string이 스코프 밖으로 벗어나고 `drop`이 호출됩니다.
// 메모리가 해제됩니다.

fn makes_copy(some_integer: i32) { // some_integer가 스코프 안으로 들어옵니다
    println!("{}", some_integer);
} // 여기서 some_integer가 스코프 밖으로 벗어납니다. 별다른 일이 발생하지 않습니다.
}
```

### 반환 값과 스코프
소유권은 값을 반환하는 과정에서도 이동한다.
```rust
fn main() {
    let s1 = gives_ownership();         // gives_ownership이 자신의 반환 값을 s1로
                                        // 이동시킵니다

    let s2 = String::from("hello");     // s2가 스코프 안으로 들어옵니다

    let s3 = takes_and_gives_back(s2);  // s2는 takes_and_gives_back로 이동되는데,
                                        // 이 함수 또한 자신의 반환 값을 s3로
                                        // 이동시킵니다
} // 여기서 s3가 스코프 밖으로 벗어나면서 버려집니다. s2는 이동되어서 아무 일도
  // 일어나지 않습니다. s1은 스코프 밖으로 벗어나고 버려집니다.

fn gives_ownership() -> String {             // gives_ownership은 자신의 반환 값을
                                             // 자신의 호출자 함수로 이동시킬
                                             // 것입니다

    let some_string = String::from("yours"); // some_string이 스코프 안으로 들어옵니다

    some_string                              // some_string이 반환되고
                                             // 호출자 함수 쪽으로
                                             // 이동합니다
}

// 이 함수는 String을 취하고 같은 것을 반환합니다
fn takes_and_gives_back(a_string: String) -> String { // a_string이 스코프 안으로
                                                      // 들어옵니다

    a_string  // a_string이 반환되고 호출자 함수 쪽으로 이동합니다
}
```
상황이 다양할지라도, 변수의 소유권 규칙은 언제나 동일하다. 어떤 값을 다른 변수에 대입하면 값이 이동하고,
힙에 데이터를 갖는 변수가 스코프를 벗어나면, 사전에 해당 데이터가 이동하여 소유권이 다른 변수에 이동되지 않은 이상 `drop`에 의해 데이터가 제거된다.

```rust
fn main() {
    let s1 = String::from("hello");

    let (s2, len) = calculate_length(s1);

    println!("The length of '{}' is {}.", s2, len);
}

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len(); // len()은 String의 길이를 반환합니다

    (s, length)
}
```
러스트에서는 튜플을 사용해 여러 값을 반환하는 것이 가능하다. 하지만 이런 식은 컨셉이 되기엔 너무 거추장스럽고 많은 작업량이 수반된다. 다행히 러스트에는 소유권 이동없이 값을 사용할 수 있는 참조자 (reference)라는 기능을 가지고 있다.

# 참조와 대여
참조자(reference) 는 해당 주소에 저장된 데이터에 접근할 수 있도록 해주는 주솟값에 해당하는 포인터와 같은 것이다.
포인터와는 달리 참조자는 살아있는 동안 특정 타입에 대한 유효한 값을 가리킴을 보장해 준다.

```rust
fn main() {
    let s1 = String::from("hello");
    let len = calculate_length(&s1);

    println!("The length of '{}' is {}.", s1, len);
}

fn calculate_length(s: &String) -> usize {      // s는 String의 참조자이다.
    s.len()
}   // 여기서 s가 스코프 밖으로 벗어난다. 하지만 참조하는 것을 소유하고 있진 않으므로 버려지지는 않는다.
```
이 앰퍼센드 `&` 기호가 참조자를 나타내고, 어떤 값의 소유권을 가져오지 않고 해당 값을 참조할 수 있도록 해준다.
`s1`에 `&`를 붙인 `&s1` 구문은 `s1` 값을 참조하지만 해당 값을 소유하지 않는 참조자를 생성한다.
값을 소유하지 않으므로 이 참조자가 가리킨 값은 참조자가 사용되지 않을 때까지 버려지지 않는다.

변수 `s`가 유효한 스코프는 여타 함수의 매개변수에 적용되는 스코프와 동일하다. 
하지만 `s`에는 소유권이 없으므로 `s`가 더 이상 사용되지 않을 때도 이 참조자가 가리킨 값이 버려지지 않는다.
함수가 실제 값 대신 참조자를 매개변수로 쓴다면 애초에 소유권이 없으니까 이 소유권을 돌려주기 위한 값 반환도 필요 없어진다.

이처럼 참조자를 만드는 행위를 대여 (borrow) 라고 한다. 변수가 기본적으로 불변성을 지니듯 참조자도 마찬가지로 참조하는 것을 수정할 수 없다.

## 가변 참조자
```rust
fn main() {
    let mut s = String::from("hello");
    change(&mut s);
}

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}
```
가변 참조자는 한 가지 큰 제약사항이 있다. 어떤 값에 대한 가변 참조자가 있다면, 그 값에 대한 참조자는 더 이상 만들 수 없다.
```rust
fn main() {
    let mut s = String::from("hello");
    
    let r1 = &mut s;
    let r2 = &mut s;        // 에러 발생

    prinlnt!("{}, {}", r1, r2);
}
```
이 에러는 `s`를 가변으로 두 번 이상 빌려올 수 없기 때문에 코드가 유효하지 않다고 말해준다.
같은 데이터에 대하여 동시에 여러 가변 참조자의 사용을 막는 이러한 제약은 값의 변경에 대한 제어가 원할하도록 해 준다.
이 제약 덕분에 러스트에서는 컴파일 타임에 데이터 경합 (`data race`)를 방지할 수 있다.

데이터 경합이란 다음 세 가지 상황이 겹칠 때 일어나는 특정한 경합 조건 (`race condition`)을 말한다.
- 둘 이상의 포인터가 동시에 같은 데이터에 접근
- 포인터 중 하나 이상이 데이터에 쓰기 작업을 시행
- 데이터 접근 동기화 매커니즘이 없음

데이터 경합은 정의되지 않은 동작을 일으키며, 런타임에 추적하려고 할 때 문제 진단 및 수정이 어렵다.
하지만 러스트에서는 데이터 경합이 발생할 가능성이 있는 코드의 컴파일을 거부하는 것으로 이 문제를 막아준다.
```rust
// 중괄호로 새 스코프를 만들어, 가변 참조자를 여러개 만들면서 동시에 존재하는 상황을 회피하는 방법도 있다.
fn main() {
    let mut s = String::from("hello");
    {
        let r1 = &mut s;
    }   // 여기서 r1이 스코프 밖으로 벗어나며 따라서 아무 문제없이 새 참조자를 만들 수 있다. 
    let r2 = &mut s;
}
```
가변 참조자와 불변 참조자를 혼용할 때도 유사한 규칙이 적용된다.
```rust
fn main() {
    let mut s = String::from("hello");
    
    let r1 = &s;    // 문제없음
    let r2 = &s;    // 문제없음
    let r3 = &mut s; // 큰 문제가 있다.

    println!("{}, {}, and {}", r1, r2, r3);
}
```
어떤 값에 대한 불변 참조자가 있는 동안 같은 값의 가변 참조자를 만드는 것 또한 불가능하다.
불변 참조자를 사용하는 쪽에서는 사용 중 값이 중간에 변경되리라 예상하지 않는다. 반면 데이터를 읽기만 하는 기능으로는
다른 쪽에서 값을 읽는 기능에 영향을 주지 않으므로 여러개의 불변 참조자를 만드는 것은 가능하다.

**참조자는 정의된 지점부터 시작하여 해당 참조자가 마지막으로 사용된 부분까지 유효하다.**
```rust
fn main() {
    let mut s = String::from("hello");

    let r1 = &s;    
    let r2 = &s;    
    let r3 = &mut s; 

    println!("{}", r3); // hello
}
```

## 댕글링 참조
댕글링 포인터 (dangling pointer) 란, 어떤 메모리를 가리키는 포인터가 남아있는 상황에서 일부 메모리를 해제해 버림으로써,
다른 개체가 할당받았을지도 모르는 메모리를 참조하게 된 포인터를 말한다. 포인터가 있는 언어에서는 자칫 잘못하면 이 댕글링 포인터를 만들기 쉽다.
하지만 러스트에서는 어떤 데이터의 참조자를 만들면, 해당 참조자가 스코프를 벗어나기 전에 
데이터가 먼저 스코프를 벗어나는지 컴파일러에서 확인하여 댕글링 참조가 생성되지 않도록 보장한다.
```rust
fn main() {
    let reference_to_nothing = dangle();
}

fn dangle() -> &String {            // String의 참조자를 반환
    let s = String::from("hello");  // s는 새로운 String이다.
    
    &s  // String s의 참조자를 반환
}   // 여기서 s는 스코프 밖으로 벗어나고 버려진다. 해당 메모리는 해제된다.
// 이런 경우에는 String인 s를 직접 반환해야한다. 그럼 소유권이 이동되며, 할당 해제도 되지 않는다.
```

# 슬라이스 (slice)
슬라이스는 컬렉션 (collection) 을 통째로 참조하는 것이 아닌, 컬렉션의 연속된 일련의 요소를 참조하도록 해준다.
슬라이스는 참조자의 일종으로서 소유권을 갖지 않는다.

슬라이스로 문제를 해결할 수 있음을 이해하기 위해서 먼저 슬라이스를 사용하지 않는 코드를 보겠다.
```rust
fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes();       // 바이트 배열로 변환
    
    // 배열의 반복자 (iterator)를 iter 매서드로 생성
    // iter() 컬렉션의 각 요소를 반환
    // enumerate() iter 의 각 결과값을 튜플로 감싸 변환
    // 반환하는 튜플은 첫번째 요소가 인덱스, 두 번째 요소가 해당 요소의 참조자로 이루어져 있다.
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
        
        s.len()
    }
}

fn main() {
    let mut s = String::from("hello world");
    let word = first_word(&s);  // word는 값 5를 받는다. (공백이 5번째 인덱스)
    
    s.clear();  // 이 코드는 String 을 비워서 ""로 만든다.
    
    // 여기서 word에는 여전히 5가 들어있지만, 이 5를 의미있게 쓸 수 있는 문자열은 더 이상 없다.
}
```

## 문자열 슬라이스
문자열 슬라이스 (string slice)는 `String`의 일부를 가리키는 참조자를 말한다.
```rust
fn main() {
    let s = String::from("hello world");

    let hello = &s[0..5];
    let world = &s[6..11];    
}
```
만드는 방식은 `String` 참조자와 유사하지만, `hello`는 추가적인 `[0..5]`로 명시된 `String`의 일부분을 가리키는 참조자이다.
`[starting_index..ending_index]` 는 `starting_index` 부터 시작해 `ending_index` 직전, 즉 1을 뺀 위치까지 슬라이스를 생성한다는 의미이다.
슬라이스는 내부적으로 시작 위치, 길이를 데이터 구조에 저장하며, 길이 값은 `ending_index` 값에서 `starting_index` 값을 빼서 계산한다.

`..` 범위 표현법은 인덱스 0부터 시작하는 경우, 앞의 값을 생략할 수 있다.
```rust
fn main() {
    let s = String::from("hello");
    
    let slice = &s[0..2];
    let slice = &s[..2];    // 의미가 같다.
}
```
마찬가지로, `String` 맨 마지막 바이트까지 포함하는 슬라이스는 뒤의 값을 생략할 수 있다.
```rust
fn main() {
    let s = String::from("hello");
    
    let len = s.len();
    
    let slice = &s[3..len];
    let slice = &s[3..];    // 의미가 같다.
}
```
앞뒤 모두 생략할 경우, 전체 문자열이 슬라이스로 생성된다.
```rust
fn main() {
    let s = String::from("hello");
    
    let len = s.len();
    
    let slice = &s[0..len];
    let slice = &s[..];     // 의미가 같음
}
```
문자열 슬라이스를 나타내는 타입은 `&str` 로 작성한다.

```rust
fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();
    
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    
    &s[..];
}
```
이제 first_word 가 반환하는 값은 원래 데이터와 분리된 값이 아니다. 
이 값은 원래 데이터에서 슬라이스 시작 위치를 가리키니느 참조자와 슬라이스 요소 개수로 구성되어 있다.

### 슬라이스로써의 문자열 리터럴
문자열 리터럴은 바이너리 내에 저장된다.
```rust
let s = "Hello, world!";
```
여기서 `s`는 바이너리의 특정 지점을 가리키는 슬라이스이다. `&str` 타입이다. `&str`은 불변참조자이므로, 문자열 리터럴은 변경할 수 없다.

### 문자열 슬라이스를 매개변수로 사용하기
리터럴과 `String`의 슬라이스를 만들 수 있다는 걸 알고 나면 `first_word` 함수 정의를 다음과 같이 작성할 수 있다.
```rust
fn first_word(s: &String) -> &str {}
```
좀 더 경험이 많은 러스타시안은 `&String` 값과 `&str` 값 모두 사용 가능한 함수를 작성할 것이다.
```rust
fn first_word(s: &str) -> &str {}
```
문자열 슬라이스라면 이를 바로 인수로써 전달할 수 있다. `String` 이라면 `String` 의 슬라이스 혹은 `String`에 대한 참조자를 전달할 수 있다.
이러한 유연성은 `역참조 강제 (deref coercions)` 기능을 이용한다.
```rust
fn main() {
    let my_string = String::from("hello world");

    // `first_word`는 `String`의 일부 혹은 전체 슬라이스에 대해 작동한다.
    let word = first_word(&my_string[0..6]);
    let word = first_word(&my_string[..]);
    // 또한 `first_word`는 `String`의 전체 슬라이스와 동일한 `String`의 참조자에 대해서도 작동한다.
    let word = first_word(&my_string);

    let my_string_literal = "hello world";
    
    // `first_word`는 문자열 리터럴의 일부 혹은 전체 슬라이스에 대해 작동한다.
    let word = first_word(&my_string_literal[0..6]);
    let word = first_word(&my_string_literal[..]);
    
    // 문자열 리터럴은 곧 문자열 슬라이스이므로 슬라이스 문법없이 작동한다.
    let word = first_word(&my_string_literal);
}
```

### 그 외 슬라이스
문자열 슬라이스는 문자열에만 특정되어 있다. 하지만 더 범용적인 슬라이스 타입도 존재한다.
```rust
let a = [1, 2, 3, 4, 5];
```
문자열 일부를 참조할 때 처럼 배열 일부를 참조하고 싶다면 다음과 같이 할 수 있다.
```rust
let a = [1, 2, 3, 4, 5];
let slice = &a[1..3];

assert_eq!(slice, &[2, 3]);
```
이 슬라이스는 `&[i32]` 타입이다. 동작 방식은 문자열 슬라이스와 동일하다.
슬라이스의 첫 번째 요소를 참조하는 참조자와 슬라이스의 길이를 저장하며 동작한다. 이런 슬라이스는 모든 컬렉션에 사용이 가능하다.

# 구조체 (struct)
구조체는 여러 값을 묶고 이름을 지어서 의미있는 묶음을 정의하는 데에 사용한다. 객체 지향 언어에 익숙한 분들이라면,
구조체란 객체의 데이터 속성 (attribute)와 비슷한 것이다.

구조체는 여러 개의 연관된 값을 가질 수 있다는 점에서 튜플과 비슷하다. 튜플처럼 구조체의 구성 요소들은 각각 다른 타입이 될 수 있다. (구성 요소에 이름을 붙일 수 있다.)
구조체를 정의하려면 `struct` 키워드와 해당 구조체에 지어줄 이름을 입력한다. 이 후 중괄호 안에는 `필드 (field)` 라고 부르는 각 구성 요소의 이름 및 타입을 정의한다.

```rust
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64
}
```

정의한 구조체를 사용하려면 해당 구조체의 각 필드에 대하여 구체적인 값을 정하여 구조체의 `인스턴스 (instance)`를 생성해야한다. 
인스턴스를 생성하려면 먼저 구조체의 이름을 적고, 중괄호를 열고, 그 안에 필드의 이름 (key)과 해당 필드에 저장할 값을 키: 값 쌍의 형태로 추가해야한다.
이 때 필드의 순서는 구조체를 정의했을 때와 동일하지 않아도 된다.

```rust
fn main() {
    let user1 = User {
        active: true,
        username: String::from("someusername123"),
        email: String::from("someone@example.com"),
        sign_in_count: 1,
    };
}
```
구조체 내 특정 값은 `.` 표기법으로 가져올 수 있다. 가변 인스턴스라면 같은 방식으로 특정 필드의 값을 변경할 수도 있다.

```rust
fn main() {
    let user1 = User {
        active: true,
        username: String::from("someusername123"),
        email: String::from("someone@example.com"),
        sign_in_count: 1,
    };

    user1.email = String::from("anotheremail@example.com");
}
```

가변성은 해당 인스턴스 전체가 지니게 된다. 일부 필드만 가변으로 만들 수 없다. 함수의 마지막 표현식에 구조체의 새 인스턴스를 생성하는 표현식을 써서 해당 인스턴스를 암묵적으로 변환할 수 있다.

```rust
fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        username: username,
        email: email,
        sign_in_count: 1,
    }
}
```
변수명과 구조체 필드명이 같을 때는 `필드 초기화 축약법 (field init shorthand)` 를 사용해서 더 적은 타이핑으로 같은 기능을 구현할 수 있다.
```rust
fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        username,
        email,
        sign_in_count: 1,
    }
}
```

### 기존 인스턴스를 이용해 새 인스턴스를 만들 때 구조체 업데이트 문법 사용하기
다른 인스턴스에서 대부분의 값을 유지한 채로 몇 개의 값만 바꿔 새로운 인스턴스를 생성하게되는 경우에 유용한 것은 바로 `구조체 업데이트 문법 (struct update syntax` 이다.
```rust
fn main() {
    let user2 = User {
        email: String::from("another@example.com"),
        ..user1     // 나머지 필드는 구조체 업데이트 문법으로 user1의 필드 값을 사용
    };
}
```
`..` 문법은 따로 명시된 필드를 제외한 나머지 필드를 주어진 인스턴스의 필드 값으로 설정한다.
user1의 값들과 동일한 값들로 나머지를 채우려면 `..user1` 를 제일 끝에 적어야 하지만,
다른 필드들은 구조체의 정의 내에 있는 필드들의 순서와는 상관없이 마음대로 몇 개든 임의 순서로 적을 수 있다.

구조체 업데이트 문법이 대입처럼 `=`를 사용한다는 점을 주목하라 이 구문은 데이터를 이동시킨다. 이 예제에서 user2를 생성한 이후에는
user1을 더 이상 사용할 수 없는데, 이는 user1의 username 필드의 `String`이 user2로 이동되기 때문이다.
user2에 email과 username의 String을 모두 제공하고 user1에서는 active와 sign_in_count 값만 사용한다면
user2를 만든 이후에도 user1은 유효하다. active와 sign_in_count 모두 copy 트레이트를 구현한 타입으로 복사가 일어난다.

### 명명된 필드 없는 구조체를 사용하여 다른 타입 만들기
러스트는 튜플과 유사한 형태의 `튜플 구조체 (tuple structs)` 도 지원한다. 튜플 구조체는 구조체 자체에는 이름을 지어 의미를 주지만,
이를 구성하는 필드에는 이름을 붙이지 않고 타입만 적는 형태이다. 
튜플 구조체는 튜플 전체에 이름을 지어주거나 특정 튜플을 다른 튜플과 구분하고 싶을 때 유용하다.

튜플 구조체의 정의는 일반 구조체처럼 `struct` 키워드와 구조체 명으로 시작되나, 그 뒤에는 타입들로 이루어진 튜플이 따라온다

```rust
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

fn main() {
    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);
}
```
구조체 내의 필드 구성은 같더라도 각각의 구조체는 별도의 타입이다. 여러 부분으로 해체할 수도 있고, `.` 과 인덱스로 개별 값에 접근할 수도 있다.

### 필드가 없는 유사 유닛 구조체
필드가 아예 없는 구조체를 정의할 수도 있습니다. 이는 유닛 타입 `()`와 비슷하게 동작하므로 `유사 유닛 구조체 (unit-like structs)` 라 지칭한다.
유사 유닛 구조체는 어떤 타입에 대해 트레이트를 구현하고 싶지만 타입 내부에는 어떤 데이터를 저장할 필요는 없을 경우 유용하다.
```rust
struct AlwaysEqual;

fn main() {
    let subject = AlwaysEqual;
}
```
AlwaysEqual을 정의하기 위해서 `struct` 키워드 뒤에 이름을 쓰고 바로 세미콜론을 붙였다. 중괄호도 괄호도 필요없다.
나중에 AlwaysEqual 의 모든 인스턴스는 언제나 다른 모든 타입의 인스턴스와 같도록 하는 동작을 구현하며, 이미 알고 있는 결과값의 테스트 용도로 사용한다고 가정해 보자. 이런 동작에는 데이터가 필요 없을 것이다.


# 메서드 문법
`메서드 (method)` 는 함수와 유사하다. `fn` 키워드와 함수명으로 선언하고, 매개변수와 반환 값을 가지며, 다른 어딘가로부터 호출될 때 실행된다.
하지만 메서드는 함수와 달리 구조체 컨텍스트에 정의되고 (열거형이나 트레이트 객체 안에 정의되기도 함), 첫 번째 매개변수가 항상 `self` 라는 차이점이 있다.
`self` 매개변수는 메서드를 호출하고 있는 구조체 인스턴스를 나타낸다.

### 메서드 정의하기
```rust
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
}

fn main() {
    let rec1 = Rectangle {
        width: 30,
        height: 50,
    };

    println!(
        "The area of the rectangle is {} square pixels.",
        rec1.area()
    );
}
```
Rectangle의 컨텍스트에 함수를 정의하기 위해서, Rectangle에 대한 `impl (implementation)` 블록을 만드는 것으로 시작한다.
이 `impl` 블록은 Rectangle 타입과 연관된다. 그런 다음 area 함수를 `impl`의 중괄호 안으로 옮기고 함수 시그니처의 첫 번째 매개변수를 `self`로 변경한다.
`메서드 문법 (method syntax)` 를 사용해 Rectangle 인스턴스의 area 메서드를 호출할 수 있다. 메서드 문법은 인스턴스, 점, 메서드명, 괄호 및 인수 로 구성된다.

area 시그니처에서는 `rectangle: Rectangle` 대신 `&self`를 사용했다. `&self`는 실제로는  `self: &self`를 줄인 것이다.
`impl` 블록 내에서 `Self`는 `impl` 블록의 대상이 되는 타입의 별칭이다. 메서드는 `Self` 타입의 `self` 라는 이름의 매개변수를 첫 번째 매개변수로 가져야 하는데,
그렇게 해야 첫 번째 매개변수 자리에 적은 `self` 형태의 축약형을 사용할 수 있다.
메서드는 다른 매개변수가 그런 것처럼 `self`의 소유권을 가져올 수도, 지금처럼 `self`를 불변으로 빌려올 수도, 가변으로 빌려올 수도 있다.

러스트는 다른 언어들처럼 구조체 필드에 대한 게터를 자동으로 만들지 않는다.

### 더 많은 매개변수를 가진 메서드 
메서드는 `self` 매개변수 뒤에 여러 매개변수를 가질 수 있으며 이 매개변수는 함수에서의 매개변수와 동일하게 기능한다.

### 연관 함수
`impl` 블록 내에 구현된 모든 함수를 `연관 함수 (associated function)` 라고 부르는데, 이는 `impl` 뒤에 나오는 타입과 모두 연관된 함수이기 때문이다.
동작하는데 해당 타입의 인스턴스가 필요하지 않다면 `self` 를 첫 매개변수로 갖지 않는 연관 함수를 정의할 수도 있다. (따라서 메서드는 아니다.)

메서드가 아닌 연관 함수는 구조체의 새 인스턴스를 반환하는 생성자로 자주 활용 된다. 이 함수들은 보통 `new` 라고 명명되는데,
`new`는 이 언어에서 특별한 이름 혹은 키워드가 아니다.
```rust
impl Rectangle {
    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }
}
```
반환 타입 및 함수 본문의 `Self` 키워드는 `impl` 카워드 뒤에 적혀있는 타입의 별칭으로 여기서는 Rectangle이 된다.

연관 함수를 호출할 땐 `let sq = Rectangle::square(3);` 처럼 구조체 명에 `::` 구문을 붙여서 호출한다. 연관 함수는 구조체의 네임스페이스 안에 있기 때문이다.

### 여러개의 impl 블록
각 구조체는 여러 개의 `impl` 블록을 가질 수 있다.
```rust
impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
}

impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}
```
위 코드에서는 `impl` 블록을 여러 개로 나눠야 할 이유는 전혀 없지만 이렇게 `impl` 블록을 여러개 작성할 수도 있다.

# 열거형 정의하기
Rectangle이 width와 height를 가지고 있는 것 처럼 구조체가 서로 연관된 필드 및 데이터를 묶는 방법을 제공한다면,
열거형은 어떤 값이 여러 개의 가능한 값의 집합 중 하나라는 것을 나타내는 방법을 제공한다.
가능한 모든 `배리언트 (variant)`들을 죽 늘어놓을 수 있는데, 이 때문에 열거형이라는 이름이 붙은 것이다.
열거형의 값은 여러 배리언트 중 하나만 될 수 있다.
```rust
enum IpAddrKind {
    V4,     // 열거형의 배리언트
    V6      // 열거형의 배리언트
}
```

### 열거형 값
```rust
let four = IpAddrKind::V4;
let six = IpAddrKind::V6;
```
열거형을 정의할 때의 식별자로 네임스페이스가 만들어져서, 각 배리언트 앞에 `이중 콜론 (::)`을 붙여야 한다.
이 방식은 `IpAddrKind::V4`, `IpAddrKind::V6` 가 모두 `IpAddrKind` 타입이라는 것을 표현할 수 있기 때문에 유용하다.

열거형을 사용하면 더 많은 이점이 있다. IP 주소 타입에 대해 더 생각해 보면, 지금으로서는 실제 IP 주소 데이터를 저장할 방법이 업고
어떤 종류인지만 알 수 있다.
```rust
enum IpAddr {
    V4(String),
    V6(String),
}

let home = IpAddr::V4(String::from("127.0.0.1"));
let loopback = IpAddr::V6(String::from("::1"));
```
열거형의 각 배리언트에 직접 데이터를 붙임으로써 구조체를 사용할 필요가 없어졌다. 또 여기서 열거형의 동작에 대한 다른 세부 사항을 살펴보기가 좀 더 쉬워졌다.
IpAddr::V4() 는 String 인수를 입력받아서 IpAddr 타입의 인스턴스를 결과로 만드는 함수이다. 
열거형을 정의한 결과로써 생성자 함수가 자동적으로 정의된다.
```rust
enum IpAddr {
    V4(u8, u8, u8, u8),
    V6(String),
}
let home = IpAddr::V4(127, 0, 0, 1);
let loopback = IpAddr::V6(String::from("::1"));
```
열거형 배리언트에는 어떤 종류의 데이터라도 넣을 수 있다. 문자열, 숫자 타입, 구조체 등은 물론 다른 열거형마저도 포함할 수 잇다.

열거형과 구조체는 한 가지 더 유사한 점이 있다. 구조체에 `impl` 을 사용해서 메서드를 정의한 것처럼, 열거형에도 정의할 수 있다.
```rust
impl Message {
    fn call(&self) {
        // 메서드 본문
    }
}

let m = Message::Write(String::from("hello"));
m.call();
```
메서드 본문에서는 self를 사용하여 호출한 열거형의 값을 가져올 것이다.

## Option 열거형이 널 값보다 좋은 점들
러스트는 다른 언어들에서 흔하게 볼 수 있는 `널 (null)` 개념이 없다.
널은 값이 없음을 표현하는 하나의 값이다.
하지만 '현재 어떠한 이유로 인해 유효하지 않거나, 존재하지 않는 하나의 값'이라는 널이 표현하려고 하는 개념은 여전히 유용하다.
널의 문제는 실제 개념에 있기보다, 특정 구현에 있다. 러스트에는 널이 없지만, 값의 존재 혹은 부재의 개념을 표현할 수 있는 열거형이 있다.
그 열거형이 바로 `Option<T>`이다.
```rust
enum Option<T> {
    None,
    Some(T),
}
```
`Option<T>` 열거형은 너무나 유용하기 떄문에, 러스트에서 기본으로 임포트하는 목록인 프렐루드에도 포함되어 있다.
이것의 배리언트 또한 프렐루드에 포함되어 있다. 따라서 `Some`, `None` 배리언트 앞에 `Option::` 도 붙이지 않아도 된다.
하지만 Option<T>는 여전히 그냥 일반 열거형이고, Some(T) 와 None 역시 여전히 Option<T>의 배리언트이다.

`Some` 값을 얻게 되면, 값이 존재한다는 것과 해당 값이 `Some` 내에 있다는 것을 알 수 있다.
`None` 값을 얻게 되면, 얻은 값이 유효하지 않다는, 어떤 면에서는 널과 같은 의미를 갖는다.

`T`에 대한 연산을 수행하기 전에 `Option<T>` 를 `T`로 변환해야 한다. 이런 방식은 널로 인해 발생하는 가장 흔한 문제인, 실제로는 널인데 널이 아니라고 가정하는 상황을 발견하는데 도움이 된다.

# match 제어 흐름 구조
러스트는 `match` 라고 불리는 매우 강력한 제어 흐름 연산자를 가지고 있는데, 이는 일련의 패턴에 대해 어떤 값을 비교한 뒤 어떤 패턴에 매핑되었는지를 바탕으로 코드를 수행하도록 해준다.
패턴은 리터럴 값, 변수명, 와일드카드 등 다양한 것으로 구성될 수 있다. `match`의 힘은 패턴의 표현성으로부터 오며 컴파일러는 모든 가능한 경우가 처리되는지 검사한다.

```rust
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => {
            println!("Lucky penny!");
            1
        },
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}
```

`match` 키워드 뒤에 표현식을 써줬는데, 위의 경우에는 `coin` 값이다. `if`에서 사용하는 조건식과 유사하지만, 큰 차이점이 있다.
`if`를 사용할 경우에는 조건문에서 boolean 값을 반환해야 하지만, 여기서는 어떤 타입이든 가능하다.

하나의 갈래 (arm)는 패턴과 코드 두 부분으로 이루어져 있다. 패턴과 실행되는 코드를 구분해주는 `=>` 연산자가 있다.
각 갈래가 그냥 값을 반환하는 것 처럼 매치 갈래의 코드가 짧다면 중괄호는 보통 사용하지 않는다. 만일 매치 갈래 내에서 여러 줄의 코드를 실행시키고 싶다면 중괄호를 사용하고,
그렇게 되면 갈래 뒤에 붙이는 쉼표는 옵션이 된다.


### 값을 바인딩하는 패턴 
매치 갈래의 또 다른 유용한 기능은 패턴과 매칭된 값들의 일부분을 바인딩할 수 있다는 것이다. 이것이 열거형의 배리언트로부터 어떤 값들을 추출할 수 있는 방법이다.

```rust
#[derive(debug)] // so we can inspect the state in a minute
enum UsState {
    Alabama,
    Alaska,
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => {}
        Coin::Nickel => {}
        Coin::Dime => {}
        Coin::Quarter(state) => {
            println!("State quarter from {:?}!", state);
            25
        }
    }
}
```
만일 `value_in_cents(Coin::Quarter(UsState::Alaska))` 를 호출했다면, `coin`은 `Coin::Quarter(UsState::Alaska)`가 될것이다.  
`state`에 대한 바인딩 값 `UsState::Alaska`가 될 것이다.

### Option<T> 을 이용하는 매칭
`Option<T>` 값을 사용하려면 `Some` 일 때 실행돼서, `Some` 내의 `T` 값을 얻을 수 있는 코드가 필요하다.
동전을 비교하는 대신 `Option<T>`의 배리언트를 비교하겠지만, `match` 표현식이 동작하는 방식은 동일하다.
```rust
fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}

let five = Some(5);
let six = plus_one(five);   // Some(5) -> Some( i + 1 ) -> Some(6)
let none = plus_one(None);
```
`match`와 열거형을 조합하는 것은 다양한 경우에 유용하다.

### 매치는 철저하다
논의할 필요가 있는 `match`의 다른 관점이 있다. 갈래의 패턴들은 모든 가능한 경우를 다루어야 한다.
```rust
fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        Some(i) => Some(i + 1),
    }
}
```
여기서는 `None` 케이스를 다루지 않았고, 따라서 이 코드는 버그를 일으킬 것이다.(컴파일을 시도하면 에러가 발생한다)
러스트의 매치는 철저하다. (exhaustive) 발생할 수 있는 경우 중 놓친 게 있음을 아는 것은 물론, 어떤 패턴을 놓쳤는가도 알고 있다.

### 포괄 패턴과 _ 자리표시자
열거형을 사용하면서 특정한 몇 개의 값들에 대해 특별한 동작을 하지만, 그 외의 값들에 대해서는 기본 동작을 취하도록 할 수 있다.
```rust
// 3이 나오면 새 멋진 모자를 얻고
// 7을 굴리면 모자를 잃게된다.
let dice_roll = 9;
match dice_roll {
    3 => add_fancy_hat(),
    7 => remove_fancy_hat(),
    other => move_player(other),
}

fn add_fancy_hat() {}
fn remove_fancy_hat() {}
fn move_player(num_spaces: u8) {}
```
나머지 모든 가능한 값을 다루는 마지막 갈래에 대한 패턴은 `other` 라는 이름을 가진 변수이다. `other` 갈래 쪽의 코드는 이 변숫값을 `move_player` 함수에 넘기는 데 사용한다.
`u8` 이 가질 수 있는 모든 값을 나열하지 않았음에도 이 코드는 컴파일 되는데, 그 이유는 특별하게 나열되지 않은 나머지 모든 값에 대해 마지막 패턴이 매칭될 것이기 때문이다.
이러한 포괄 (catch-all) 패턴은 `match`의 철저함을 만족시킨다. 패턴들은 순차적으로 평가되므로 마지막에 포괄적인 갈래를 위치시켜야 한다.
포괄 패턴이 필요한데 그 포괄 패턴의 값을 사용할 필요는 없는 경우에 쓸 수 있는 패턴도 있다. `_` 는 어떠한 값이라도 매칭되지만, 그 값을 바인딩하지는 않는 특별한 패턴이다.
이는 러스트에게 해당 값을 사용하지 않겠다는 것을 알려주므로 러스트는 사용되지 않는 변수에 대한 경고를 띄우지 않을 것이다.

```rust
// 주사위를 굴려 3 혹은 7이외의 숫자가 나왔다면 아무 일도 일어나지 않는다.

let dice_roll = 9;
match dice_roll {
    3 => add_fancy_hat(),
    7 => remove_fancy_hat(),
    _ => (),
}

fn add_fancy_hat() {}
fn remove_fancy_hat() {}
```
여기에서는 러스트에게 명시적으로 앞의 갈래에 매칭되지 않은 어떠한 값도 사용하지 않을 것이며, 어떠한 코드도 실행하지 않기를 원한다고 명시적으로 알려준 것이다.

# if let 을 사용한 간결한 제어 흐름
`if let` 문법은 `if` 와 `let` 을 조합하여 하나의 패턴만 매칭시키고 나머지 경우는 무시하도록 값을 처리하는 간결한 방법을 제공한다.
```rust
// 그 값이 Some 배리언트 일 경우에만 코드를 실행시키고 싶다.
let config_max = Some(3u8);
match config_max {
    Some(max) => println!("The maximum is configured to be {}", max),
    _ => (),
}
```
이 값이 `Some` 이면 패턴 내에 있는 max에 Some 배리언트의 값을 바인딩하고 출력한다. `None` 값에 대해서는 아무처리도 하지 않으려고 한다.
`match` 표현식을 만족시키려면 딱 하나의 배리언트 처리 후 `_ => ()` 를 붙여야 하는데, 이는 다소 성가신 보일러 플레이트 코드이다.
그 대신, `if let` 을 이용하여 이 코드를 더 짧게 쓸 수 있다.
```rust
let config_max = Some(3u8);
if let Some(max) = config_max {
    printnln!("The maximum is configured to be {}", max);
}
```
`if let`은 `=` 로 구분된 패턴과 표현식을 입력받는다. 이는 `match`와 동일한 방식으로 작동하는데, 여기서 표현식은 `match`에 주어지는 것이고 패턴은 이 `match`의 첫 번째 갈래와 같다.
위의 경우 패턴은 `Some(max)` 이고 `max` 는 `Some` 내에 있는 값에 바인딩된다. 그렇게 되면 `match` 의 갈래 안에서 `max`를 사용했던 것과 같은 방식으로 `if let` 본문 블록 내에서 `max`를 사용할 수 있다.

`if let` 을 사용하면 보일러 플레이트 코드를 덜 쓰게 된다. 하지만 `match`가 강제햇던 철저한 검사를 안하게 된다.
즉 `if let`은 한 패턴에 매칭될 때만 코드를 실행하고 다른 경우는 무시하는 `match` 문을 작성할 때 사용하는 문법 설탕 `Syntax sugar` 라고 생각하면 된다.

if let 과 함께 else 를 포함시킬 수 있다. else 뒤에 나오는 코드 블록은 `match` 표현식에서 `_` 케이스 뒤에 나오는 코드 블록과 동일하다.
```rust
let mut count = 0;
match coin {
    Coin::Quarter(state) => println!("State quarter from {:?}!", state),
    _ => count += 1,
}
// 위와 아래는 동작이 같다.
if let Coin::Quarter(state) = coin {
    println!("State quarter from {:?}", state);
} else {
    count += 1
}
```

# 패키지와 크레이트
크레이트(crate) 는 러스트가 컴파일 한 차례에 고려하는 가장 작은 코드 단위이다. `cargo` 대신 `rustc` 를 실행하여 단일 소스 코드 파일을 넘겨주더라도,
컴파일러는 그 파일이 크레이트라고 생각한다. 크레이트는 여러 모듈을 담을 수 있고, 모듈은 이 크레이트와 함께 컴파일되는 다른 파일들에 정의되어 있을 수도 있다.

크레이트는 바이너리일 수도 있고, 라이브러리일 수도 있다. 바이너리 크레이트 (binary crate)는 커맨드 라인 프로그램이나 서버처럼 실행 가능한 실행파일로 컴파일할 수 있는 프로그램이다.
바이너리 크레이트는 실행파일이 실행되면 무슨 일이 일어나는지를 정의한 `main` 함수를 포함하고 있어야 한다.

라이브러리 크레이트는 (library crate) 는 `main` 함수를 가지고 있지 않고 실행파일 형태로 컴파일되지 않는다. 그 대신, 여러 프로젝트에서 공용될 의도로 만들어진 기능들이 정의되어 있다.
러스타시안들이 '크레이트'라고 말하면 대부분은 이 라이브러리 크레이트를 의미하는 것이고, 크레이트라는 단어는 일반적인 프로그래밍 개념에서의 '라이브러리'와 혼용된다.

패키지 (package) 는 일련의 기능을 제공하는 하나 이상의 크레이트로 구성된 번들이다. 패키지에는 이 크레이트들을 빌드하는 법이 설명된 Cargo.toml 파일이 포함되어 있다.
카고는 실제로 코드를 빌드하는 데 사용하는 커맨드 라인 도구의 바이너리 크레이트가 포함된 패키지입니다. 카고 패키지에는 또한 이 바이너리 크레이트가 의존하고 있는 라이브러리 패키지도 포함되어 있다.
다른 프로젝트도 카고의 라이브러리 크레이트에 의존하여 카고의 커맨드 라인 도구가 사용하는 것과 동일한 로직을 사용할 수 있다.

패키지에는 여러 개의 바이너리 크레이트가 원하는 만큼 포함될 수 있지만, 라이브러리 크레이트는 하나만 넣을 수 있다. 
패키지에는 적어도 하나 이상의 크레이트가 포함되어야 하며, 이는 라이브러리든 바이너리든 상관없다.
```text
// 패키지 생성
cargo new my-project
```
Cargo.toml 을 텍스트 편집기로 열어보면 src/main.rs 가 따로 적시되진 않음을 알 수 있다. 
카고는 패키지명과 같은 이름의 바이너리 크레이트는 src/main.rs 가 크레이트 루트라는 관례를 준수한다. 마찬가지로 패키지 디렉터리에 src/lib.rs 파일이 존재할 경우,
카고는 해당 패키지가 패키지명과 같은 이름의 라이브러리 크레이트를 포함하고 있다고 판단한다. 그리고 이 라이브러리 크레이트의 크레이트 루트는 src/lib.rs 이다.
카고는 라이브러리 혹은 바이너리를 빌드할 때 이 크레이트 루트 파일을 `rustc` 에게 전달한다.
만약 어떤 패키지가 src/main.rs 와 src/lib.rs 를 가지고 있다면 해당 패키지는 패키지와 같은 이름의 바이너리, 라이브러리 크레이트를 포함하게 된다.
src/bin 디렉터리 내에 파일을 배치하면 각각의 파일이 바이너리 크레이트가 되어, 여러 바이너리 크레이트를 패키지에 포함할 수 있다.