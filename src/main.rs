fn main() {
    let x: u8 = 1;
    println!("{}", x);
    let x: String = "Text".to_string();
    println!("{}", x);
    const THIS_IS_A_CONSTANT: f32 = 1.24;
    println!("{}", THIS_IS_A_CONSTANT);
    // Scalar data types tests
    let foo = 2; //Untyped number. Integer.
    let fooo: u8 = 2; //Typer number. Integer. Unsigned (Only positives)
    let bar = 3.3;
    let baar: f32 = 4.56;
    println!("{}{}{}{}{}{}{}", foo, " ", fooo," ", bar," ", baar);
    let xd: bool = true;
    let dx: char = 'a';
    // Compounds
    let  mut tup = (198, 'A', "Hola a todos".to_string());
    println!("{}", tup.1);
    tup.0 = "replaced".to_string();
    println!("{}", tup.0);
}