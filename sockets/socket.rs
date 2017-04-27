fn main() {
    let a = 0;
    {
        let b = 10;
        let a = b;
    }
    println!("{}", a);
}
