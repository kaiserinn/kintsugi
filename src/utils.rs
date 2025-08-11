use std::io::{self, Write};

pub fn confirm(prompt_message: &str, default: bool) -> bool {
    let hint = if default { "[Y/n]" } else { "[y/N]" };

    loop {
        print!("{prompt_message} {hint} ");
        io::stdout().flush().expect("Failed to flush stdout");

        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        let response = input.trim().to_lowercase();

        match response.as_str() {
            "y" | "yes" => return true,
            "n" | "no" => return false,
            "" => return default,
            _ => {
                println!("Invalid input. Please enter 'y', 'yes', 'n', or 'no'.");
                continue;
            }
        }
    }
}
