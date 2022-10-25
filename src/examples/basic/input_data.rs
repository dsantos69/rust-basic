use std::io; // library standard for use the input

// function to convert strung to int
fn convert_to_int(data_input: &String) -> i32 {
    let x = data_input.trim().parse::<i32>().unwrap();
    x
}

fn input_data() {
    //variable to first number
    let mut number1 = String::new();

    // input to get first number
    io::stdin()
        .read_line(&mut number1)
        .expect("error when reading number 1");

    //variable to second number
    let mut number2 = String::new();

    // input to get second number
    io::stdin()
        .read_line(&mut number2)
        .expect("error when reading number 2");

    if convert_to_int(&number1) > convert_to_int(&number2) {
        println!(
            "the number:{} is greater than the number:{}",
            number1, number2
        );
    } else {
        println!(
            "the number:{} is lesser or equal than the number:{}",
            number1, number2
        );
    }
}
