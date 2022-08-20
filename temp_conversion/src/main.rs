use std::io;

fn main() {
    // print!("\x1B[2J\x1B[1;1H");
    println!("Welcome to the temperature conversion program");
    println!("Written by Ian Daniel\n\n");

    let mut temp_type = String::new();

    loop {
        println!("What is the temperature that you need to convert?");
        println!("\tA: Celsius");
        println!("\tB: Fahrenheit");
        println!("\tQ: Quit");

        io::stdin()
            .read_line(&mut temp_type)
            .expect("Failed to read line");

        match temp_type.trim() {
            "a" | "A" | "b" | "B " => break,
            "q" | "Q" => return,
            _ => {
                print!("\x1B[2J\x1B[1;1H");
                temp_type = "".to_string();
                println!("Please enter either A, B or Q\n\n\n");
                continue;
            }
        };
    }

    let mut temperature = String::new();
    let f_temp: f32;
    loop {
        println!("\nWhat is the temperature that you want to convert?");
        io::stdin()
            .read_line(&mut temperature)
            .expect("Failed to read line");
        match temperature.trim().parse() {
            Ok(num) => f_temp = num,
            Err(_) => {
                println!("Please only enter numbers");
                temperature = "".to_string();
                continue;
            }
        };

        break;
    }    

    let new_temp: f32 = match temp_type.trim() {
        "a" | "A" => to_fahrenheit(f_temp),
        "b" | "B" => to_celsius(f_temp),
        _ => return
    };

    println!("The new temperature is {}", new_temp);
}

fn to_fahrenheit(celsius: f32) -> f32 { (celsius * (9.0/5.0)) + 32.0 }
fn to_celsius(fahrenheit: f32) -> f32 { (fahrenheit - 32.0) * (5.0/9.0) }