enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug)]
enum Gender {
    Male,
    Female,
}

fn main() {
    let player: Direction = Direction::Right;
    let player_sex: Gender = Gender::Male;

    match player {
        Direction::Up => println!("The player is up"),
        Direction::Down => println!("The player is down"),
        Direction::Left => println!("The player is left"),
        Direction::Right => println!("The player is right"),
    }

    println!("The player sex is {:?}", player_sex);
}
