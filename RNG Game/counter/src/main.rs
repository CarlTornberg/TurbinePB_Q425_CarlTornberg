use std::io::stdin;

fn main() {
    println!("Select a number we shall count to?");

    loop {
        let mut user_input = String::new();
        if let Err(e) = stdin().read_line(&mut user_input) {
            println!("Failed to read line. Please try again! {}", e);
            continue;
        }
        if let Ok(val) = user_input.trim().parse::<i64>() {
            println!("Counting to {}", val);
            let mut i = 1;
            while i <= val {
                println!("{}", i);
                i += 1;
            }
            break;
        }
        else {
            println!("Please enter a valid number between {} and {}!", i64::MIN, i64::MAX);
            continue;
        }
    }
}
