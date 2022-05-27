use std::{io, num::ParseFloatError};

fn main() {
    println!("Select which side to solve!\n 1. Solve for Hypotenuse\n 2. Solve for Leg A\n 3. Solve for Leg B");

    let mut select_which_side_to_solve = String::new();
    io::stdin()
        .read_line(&mut select_which_side_to_solve)
        .expect("Failed to read line");

    let selected_side: i8 = select_which_side_to_solve
        .trim()
        .parse()
        .expect("Please type a number!");

    match selected_side {
        1 => {
            println!("Solve for the Hypotenuse!");
            let mut side_a = String::new();
            let mut side_b = String::new();

            println!("Enter the length of side A:");
            io::stdin()
                .read_line(&mut side_a)
                .expect("Failed to read line");

            println!("Enter the length of side B:");
            io::stdin()
                .read_line(&mut side_b)
                .expect("Failed to read line");

            // https://jakedawkins.com/2020-04-16-unwrap-expect-rust/
            let hypotenuse = solve_for_hypotenuse(&side_a, &side_b);
            match hypotenuse {
                Ok(value) => println!("The hypotenuse is {}", value),
                Err(error) => println!("{}", error),
            }
        }
        2 => {
            println!("You selected the Leg A!");
            let mut side_b = String::new();
            let mut hypotenuse = String::new();

            println!("Enter the length of side B:");
            io::stdin()
                .read_line(&mut side_b)
                .expect("Failed to read line");

            println!("Enter the length of the hypotenuse:");
            io::stdin()
                .read_line(&mut hypotenuse)
                .expect("Failed to read line");

            let side_a = solve_for_leg_a(&hypotenuse, &side_b);
            match side_a {
                Ok(value) => println!("The length of Leg A is {}", value),
                Err(error) => println!("{}", error),
            }
        }
        3 => {
            println!("You selected the Leg B!");
            let mut side_a = String::new();
            let mut hypotenuse = String::new();

            println!("Enter the length of side A:");
            io::stdin()
                .read_line(&mut side_a)
                .expect("Failed to read line");

            println!("Enter the length of the hypotenuse:");
            io::stdin()
                .read_line(&mut hypotenuse)
                .expect("Failed to read line");

            let side_b = solve_for_leg_b(&hypotenuse, &side_a);
            match side_b {
                Ok(value) => println!("The length of Leg A is {}", value),
                Err(error) => println!("{}", error),
            }
        }
        _ => println!("You selected an invalid side!"),
    }
}

fn solve_for_hypotenuse(leg_a: &String, leg_b: &String) -> Result<f64, ParseFloatError> {
    // must handle the error case when the user enters a non-numeric value
    let leg_a = match leg_a.trim().parse::<f64>() {
        Ok(leg_a) => leg_a,
        Err(e) => return Err(e),
    };
    let leg_b = match leg_b.trim().parse::<f64>() {
        Ok(leg_b) => leg_b,
        Err(e) => return Err(e),
    };
    let result = (leg_a * leg_a + leg_b * leg_b).sqrt();
    Ok(result)
}

fn solve_for_leg_a(hypotenuse: &String, leg_b: &String) -> Result<f64, ParseFloatError> {
    let hypotenuse = match hypotenuse.trim().parse::<f64>() {
        Ok(hypotenuse) => hypotenuse,
        Err(e) => return Err(e),
    };
    let leg_b = match leg_b.trim().parse::<f64>() {
        Ok(leg_b) => leg_b,
        Err(e) => return Err(e),
    };
    let result = (hypotenuse * hypotenuse - leg_b * leg_b).sqrt();
    Ok(result)
}

fn solve_for_leg_b(hypotenuse: &String, leg_a: &String) -> Result<f64, ParseFloatError> {
    let hypotenuse = match hypotenuse.trim().parse::<f64>() {
        Ok(hypotenuse) => hypotenuse,
        Err(e) => return Err(e),
    };
    let leg_a = match leg_a.trim().parse::<f64>() {
        Ok(leg_a) => leg_a,
        Err(e) => return Err(e),
    };
    let result = (hypotenuse * hypotenuse - leg_a * leg_a).sqrt();
    Ok(result)
}
