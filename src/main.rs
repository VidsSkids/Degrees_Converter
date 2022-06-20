use std::io;
fn main() {
    println!("Welcome to the Rust Degree Converter!");

    loop {

        let mut degrees = String::new();

        let mut degrees_type = String::new();

        println!("Please enter the number of degrees you would like to convert:");

        io::stdin().read_line(&mut degrees)
            .expect("Failed to read line");

        let degrees: f32 = match degrees.trim().parse::<f32>() {
            Ok(num) => num,
            Err(_) => {
                println!("Please enter a valid number!");
                continue;
            }
        };

        println!("Please enter F for farhenheit or C for celcius:");

        io::stdin().read_line(&mut degrees_type)
            .expect("Failed to read line");

        degrees_type.remove(degrees_type.len() - 1);
        if degrees_type.eq_ignore_ascii_case("F") {
            let celcius = (degrees - 32.0) * 5.0 / 9.0;
            println!("{} fahrenheit degrees is {} celcius degrees", degrees, celcius);
            break;
        } else if degrees_type.eq_ignore_ascii_case("C") {
            let fahrenheit = (degrees * 9.0 / 5.0) + 32.0;
            println!("{} celcius degrees is {} fahrenheit degrees", degrees, fahrenheit);
            break;
        } else {
            println!("Please enter F for farhenheit or C for celcius!");
        }
    }
}
