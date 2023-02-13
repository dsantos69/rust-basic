fn main() {
    let tupla = (12, "values", 3.14, (1, 2, 3));
    let (a, b, c, d) = tupla;

    println!("The value of A is {}", a);
    println!("The value of B is {}", b);
    println!("The value of C is {}", c);
    println!("The value of D is {:?}", d);
}
