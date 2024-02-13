fn main() {
    let slice = "Hello";
    println!("Slice is {} bytes and also {} characters", slice.len(), slice.chars().count());
    let slice2 = "안녕!";
    println!("Slice2 is {} bytes but only {} characters.", slice2.len(), slice2.chars().count());
}
