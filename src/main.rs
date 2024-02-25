// & : referencing
// * : dereferencing

fn main() {
    let mut my_number = 0;
    let num_ref = &mut my_number; // num_ref가 my_number 데이터를 바꿀 수 있다.

    *num_ref = 10;
}