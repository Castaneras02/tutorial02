fn main() {
    let mut x = 4;
    println!("{}{}", "x is equal to ", x);
    {
        let x = 13;
        println!("{}", x)
    }
    x = x + 1;
    println!("{}{}", "x is equal to ", x);
}
