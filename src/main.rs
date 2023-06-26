use std::io;
use std::io::{BufRead, Read};
use console::Term;

const NUMBER_OF_OPTIONS:usize = 8;

fn main() {

    let options: [char;NUMBER_OF_OPTIONS] = ['1','2','4','5','7','8','q','Q'];

    print_welcome_message();
    'main: loop {
        print_convert_options(options);
        let option:char = get_convert_option();

        if is_valid_option(option, options) {
            if option==options[6] || option==options[7] {
                break 'main
            }
            let temperature_value: f64 = get_temperature_value();
            select_converter(option, options, temperature_value);
            println!("Press Enter key to continue...");
            let mut input = String::new();
            let stdin = io::stdin();
            let mut handle = stdin.lock().take(1);
            handle.read_line(&mut input);
        } else {
            println!("Invalid option! Please choose one of the following:");
        }
    }
    goodbye_message();
}



fn convert_kelvin_to_celsius(temperature_value: f64) {
    println!("Kelvin -> Celsius");
    println!("{}ºK = {}ºC\n", temperature_value, (temperature_value-273.15));
}

fn convert_celsius_to_kelvin(temperature_value: f64) {
    println!("Celsius -> Kevin");
    println!("{}ºC = {}ºK\n", temperature_value, (temperature_value+273.15));
}

fn convert_kelvin_to_fahrenheit(temperature_value: f64) {
    println!("Kelvin -> Fahrenheit");
    println!("{}ºK = {}ºF \n", temperature_value, ((temperature_value-273.15)*(9.0/5.0))+32.0)
}

fn convert_fahrenheit_to_kelvin(temperature_value: f64) {
    println!("Fahrenheit -> Kelvin");
    println!("{}ºF = {}ºK \n", temperature_value, ((temperature_value-32.0)*(5.0/9.0))+273.15)
}

fn convert_celsius_to_fahrenheit(temperature_value: f64) {
    println!("Celsius -> Fahrenheit");
    println!("{}ºC = {}ºF\n", temperature_value, (temperature_value*(9.0/5.0))+32.0);
}

fn convert_fahrenheit_to_celsius(temperature_value: f64) {
    println!("Fahrenheit -> Celsius");
    println!("{}ºF = {}ºC \n", temperature_value, (temperature_value-32.0)*(5.0/9.0))
}

fn select_converter(option:char, options:[char;NUMBER_OF_OPTIONS], temperature_value:f64) {
    if option==options[0] {
        convert_fahrenheit_to_celsius(temperature_value);
    } else if option==options[1] {
        convert_celsius_to_fahrenheit(temperature_value);
    } else if option==options[2] {
        convert_fahrenheit_to_kelvin(temperature_value);
    }else if option==options[3] {
        convert_kelvin_to_fahrenheit(temperature_value);
    }else if option==options[4] {
        convert_celsius_to_kelvin(temperature_value);
    }
    else if option==options[5] {
        convert_kelvin_to_celsius(temperature_value);
    }
}

fn get_temperature_value() -> f64 {

    let mut temperature_value:f64;
    loop {
        println!("Enter temperature to convert:");

        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Couldn't read temperature value");
        temperature_value = match input.trim().parse() {
            Ok(temp) => temp,
            Err(_) => continue,
        };
        break;
    }
    temperature_value
}


fn is_valid_option(option:char, options:[char;NUMBER_OF_OPTIONS]) -> bool {
    for c in options {
        if c==option {
            return true;
        }
    }
    false
}

fn get_convert_option() -> char {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Couldn't read input!");

    match input.trim().parse() {
        Ok(option) => option,
        Err(_) => 'z'
    }




}

fn print_convert_options(options:[char;NUMBER_OF_OPTIONS]) {
    println!("Fahrenheit <-> Celsius");
    println!("Enter [{}] to converter Fahrenheit to Celsius;",options[0]);
    println!("Enter [{}] to converter Celsius to Fahrenheit;\n", options[1]);
    println!("Fahrenheit <-> Kelvin");
    println!("Enter [{}] to converter Fahrenheit to Kelvin;", options[2]);
    println!("Enter [{}] to converter Kelvin to Fahrenheit;\n", options[3]);
    println!("Celsius <-> Kelvin");
    println!("Enter [{}] to converter Celsius to Kelvin;", options[4]);
    println!("Enter [{}] to converter Kelvin to Celsius;\n", options[5]);
    println!("Enter [{}] or [{}] to quit!", options[6], options[7]);
}

fn goodbye_message() {
    println!("****************************");
    println!("* Thank you, see you soon! *");
    println!("**************************** \n");
}

fn print_welcome_message() {
    println!("*************************************");
    println!("* Welcome to temperature converter! *");
    println!("************************************* \n");
}