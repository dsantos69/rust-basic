use std::io;

// function to covert string to int
fn convert_to_int(data_input: &String) -> i32 {
    let x = data_input.trim().parse::<i32>().unwrap();
    x
}
fn factorial_while() {
    let mut input_value = String::new();
    let mut factorial = 1; // initial value for factorial

    io::stdin()
        .read_line(&mut input_value)
        .expect("Error when read input value");

    let mut input_value_int = convert_to_int(&input_value); // convert to int

    while input_value_int > 1 {
        // The factorial value is multiplied by the value entered
        // and stored. it starts multiplying with 1
        factorial = factorial * input_value_int;

        // Is decreased by 1 of the entered value
        // to continue the factorial calculation
        input_value_int = input_value_int - 1;
    }

    println!("value of factorial is {}", factorial);
}
