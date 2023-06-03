use std::io;

fn main() {
    loop {
        println!("Enter 'F' to convert a Farenheit value to Celsius.");
        println!("Enter 'C' to convert a Celsius value to Farenheit");
        println!("Enter 'Q' to quit.");

        let mut choice = String::new();

        io::stdin()
            .read_line(&mut choice)
            .expect("Failed to read line.");

        let choice = choice.trim();

        if choice == "Q" {
            break;
        }

        let mut temp = String::new();

        if choice == "F" {
            println!("Converting Farenheit to Celsius.");
            println!("Enter Farenheit value to convert: ");
        } else if choice == "C" {
            println!("Converting Celsius to Farenheit.");
            println!("Enter Celsius value to convert:");
        }

        io::stdin()
            .read_line(&mut temp)
            .expect("Failed to read line.");

        let temp: f32 = match temp.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        if choice == "F" {
            let result = (temp - 32.0) / 1.8;
            println!("{} Farenheit is {} Celsius", temp, result);
        } else if choice == "C" {
            let result: f32 = (temp * 1.8) + 32.0;
            println!("{} Celsius is {} Farenheit", temp, result);
        }
    }
}
