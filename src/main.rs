fn main() {
    let tup: (i32, f64, String) = (500, 6.4, "string".to_string());

    let (x, y, z) = tup;
    println!("{}", x);
    println!("{}", y);
    println!("{}", z);
}