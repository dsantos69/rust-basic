use std::io;

// function to covert string to int
fn convert_to_int(data_input: &String) -> i32 {
    let x = data_input.trim().parse::<i32>().unwrap();
    x
}
fn avarange_while() {
    println!("Informe a quantidade de medias");
    let mut notes = String::new(); // value of input
    io::stdin()
        .read_line(&mut notes)
        .expect("Error when read notes");

    let mut sum_recuperation = 0;
    let mut sum_aproved = 0;
    let mut sum_reproved = 0;
    let mut invalid_value = 0;
    let mut i = 0;

    while convert_to_int(&notes) > i {
        println!("Informe a {}º media", i + 1);
        let mut media_student = String::new();
        io::stdin()
            .read_line(&mut media_student)
            .expect("error when read media_student");

        i += 1;

        if convert_to_int(&media_student) <= 3 && convert_to_int(&media_student) > 0 {
            sum_reproved += 1
        }

        if convert_to_int(&media_student) < 6 && convert_to_int(&media_student) > 3 {
            sum_recuperation += 1
        }

        if convert_to_int(&media_student) <= 10 && convert_to_int(&media_student) >= 6 {
            sum_aproved += 1
        }

        if convert_to_int(&media_student) < 0 && convert_to_int(&media_student) < 10 {
            invalid_value += 1
        }
    }

    println!("---------------------");
    println!("the number of students approved is {}", sum_aproved);
    println!("the number of students in recovery is {}", sum_recuperation);
    println!("the number of students disapproved is {}", sum_reproved);
    if invalid_value != 0 {
        println!("the number of invalid values {}", invalid_value);
    }
}
