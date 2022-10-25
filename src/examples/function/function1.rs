fn double(num: i32) -> i32 {
    return 2 * num;
}

fn greater(a: i32, b: i32) -> i32 {
    if a >= b {
        return a;
    } else {
        return b;
    }
}

fn main() {
    println!("The double of the number 5 is {}", double(5));

    println!("The largest number between 5 and 4 is {}", greater(5, 4));
}
