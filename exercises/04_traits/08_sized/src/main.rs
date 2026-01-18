fn main() {
    let x = std::mem::size_of::<&str>();
    println!("Size of &str: {}", x);
}