use std::io;
// https://stackoverflow.com/questions/26388861/how-can-i-include-a-module-from-another-file-from-the-same-project
mod pythagorean;
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
            let hypotenuse =
                pythagorean::pythagorean_theorem::solve_for_hypotenuse(&side_a, &side_b);
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

            let side_a = pythagorean::pythagorean_theorem::solve_for_leg_a(&hypotenuse, &side_b);
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

            let side_b = pythagorean::pythagorean_theorem::solve_for_leg_b(&hypotenuse, &side_a);
            match side_b {
                Ok(value) => println!("The length of Leg A is {}", value),
                Err(error) => println!("{}", error),
            }
        }
        _ => println!("You selected an invalid side!"),
    }
}
