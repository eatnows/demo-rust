use std::cmp::Ordering;
use std::io;
use rand::Rng;

fn main() {
    println!("Guess the number!");

    // Rng 는 난수 생성기를 구현한 메서드들을 정의한 트레이트 (trait) 이며
    // 해당 메서드들을 이용하기 위해서는 반드시 스코프 내에 있어야한다.
    let secret_number = rand::thread_rng().gen_range(1..=100);

    println!("The secret number is: {secret_number}");

    loop {
        println!("Please input your guess");

        // 새로운 String 인스턴스를 담음
        // String은 표준 라이브러리에서 제공하는 확장 가능한 (growable) UTF-8 인코딩의 문자열 타입
        let mut guess = String::new();

        // std 표준 라이브러리
        // io 입출력 라이브러리
        // std::io 는 사용자의 입력을 받는 것을 포함하여 io와 관련된 기능을 제공
        // :: 연관함수 (associated function) - 어떤 타입에 구현된 함수
        // & 코드의 여러 부분에서 데이터를 여러 번 메모리에서 복사하지 않고 접근하기 위한 방법을 제공하는 참조자임을 나타낸다 (reference)
        // 참조자가 변수처럼 기본적으로 불변이다. &mut 을 해야 가변으로 작성할 수 있음
        io::stdin()
            // 사용자 이력이 어떤 문자열에 저장될 것인지 알려줌
            // 메서드가 문자열의 내용물을 바꿀 수 있기 때문에 이 문자열 인수는 가변이어야 한다.
            .read_line(&mut guess)
            // Result는 enum 인데, 여러 개의 가능한 상태 값을 배리언트 (variant) 라고 한다.
            // Result 타입의 목적은 에러 처리용 정보를 담아내기 위한 것
            // Result의 배리언트는 Ok, Err 이다.
            // OK 성공했음을 나타내며 내부에 성공적으로 생성된 결과를 가지고 있음
            // Err 실패했음을 나타내며 그 이유에 대한 정보를 가지고 있음
            // Result 타입의 값에도 메서드가 잇다. Result 인스턴스에는 expect 메서드가 있다.
            // 위의 경우 결과값은 사용자가 표준 입력으로 입력했던 바이트의 개수이다.
            .expect("Failed to read line");

        // shadowing
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => { continue; },
        };

        // {} 자리표시자 (placeholder)
        println!("You guessed: {guess}");

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
// 크레이트는 러스트 코드 파일들의 모음이라는 점.
// 우리가 만들고 있는 프로젝트는 실행이 가능한 바이너리 크레이트 (binary crate)이다.
// rand 크레이트는 자체적으로 실행될 수는 없고, 다른 프로그램에서 사용되기 위한 용도인 라이브러리 크레이트 (library crate)이다.


