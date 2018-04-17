fn main() {
    println!("Hello, world!");
    #[cfg(inside_matrix)]
    println!("BTW, there is no spoon.");
}
