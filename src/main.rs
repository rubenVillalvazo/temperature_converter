use std::io;

fn fahrenheit_to_celsius(value: f64) -> f64 {
    let celsius: f64 = (value - 32.0) / 1.8;
    return celsius;
}

fn celsius_to_fahrenheit(value: f64) -> f64 {
    let fahrenheit: f64 = (value * 1.8) + 32.0;
    return fahrenheit;
}

fn user_input() -> String {
    let mut user_input: String = String::new();
    io::stdin()
        .read_line(&mut user_input)
        .expect("Failed to read line");
    return user_input;
}

fn select_temperature() {
    'chose: loop {
        println!("Chose (F) for fahrenheit to celsius or (C) for celsius to fahrenheit: ");

        let user_choice: String = user_input().trim().to_lowercase();

        if user_choice == "f" {
            println!("Input the Fahrenheit value: ");

            let fahrenheit: f64 = user_input()
                .trim()
                .parse()
                .expect("Must to input a number: found _string_ or _None_");

            println!(
                "The equivalent of {fahrenheit} fahrenheit to celsius is: {}",
                fahrenheit_to_celsius(fahrenheit)
            );
            break 'chose;
        } else if user_choice == "c" {
            println!("Input the Celsius value: ");

            let celsius: f64 = user_input()
                .trim()
                .parse()
                .expect("Must to input a number: found _string_ or _None_");

            println!(
                "The equivalent of {celsius} celsius to fahrenheit is: {}",
                celsius_to_fahrenheit(celsius)
            );
            break 'chose;
        } else {
            println!("Input a valid option");
            continue;
        }
    }
}

fn converter() {
    select_temperature();
    'converter: loop {
        println!("Would you like to do another convertion? (Y/N)");
        let user_answer: String = user_input().trim().to_lowercase();
        if user_answer == "y" || user_answer == "yes" {
            converter();
            break 'converter;
        } else if user_answer == "n" || user_answer == "no" {
            break 'converter;
        } else {
            println!("Please input a valid option.");
            continue;
        }
    }
}

fn main() {
    println!("TEMPERATURE CONVERTER.");
    converter();
}
