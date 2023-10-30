use std::env;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    dotenvy::dotenv()?;

    // print all env values
    for (key, value) in env::vars() {
        println!("{key}: {value}");
    }

    // initialize empty int value
    let mut myenv_int: i32 = 0;

    // initialize empty string value
    let mut myenv_string: String = String::new();

    // only print env values starting with "MYENV_"
    print!("\n\nOnly print env values starting with \"MYENV_\":\n");
    for (key, value) in env::vars().filter(|(key, _)| key.starts_with("MYENV_")) {
        println!("  {key}: {value}");

        // check if env value is an int and assign it to myenv_int
        if let Ok(value) = value.parse::<i32>() {
            myenv_int = value;
            print!("    {} is an int\n", myenv_int);
        }

        // check if env value is a string and assign it to an co
        if let Ok(value) = value.parse::<String>() {
            myenv_string = value;
            print!("    {} is a string\n", myenv_string);
        }
    }

    Ok(())
}
