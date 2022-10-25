use std::io;

// function to covert string to int
fn convert_to_int(data_input: &String) -> i32 {
    let x = data_input.trim().parse::<i32>().unwrap();
    x
}
fn sum_while() {
    let mut sum = 0; // value of sum
    let mut input_value = String::new(); // value of input

    io::stdin()
        .read_line(&mut input_value)
        .expect("Error when read input value");

    let mut value_int = convert_to_int(&input_value); // convert input value to int

    // loop to find out the sum of digits
    while value_int != 0 {
        let r = value_int % 10;
        sum = sum + r;
        value_int = value_int / 10;
    }

    println!("the value of the sum of digits is {}", sum);
}
