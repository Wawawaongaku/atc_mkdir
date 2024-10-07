use std::fs;
use std::io::{self, Write};

fn main() -> io::Result<()> {
    let mut word = String::new();
    let mut ls = String::new();

    print!("Please input contest name (Ex: ABC123) > ");
    io::stdout().flush()?;
    io::stdin().read_line(&mut word)?;
    let word = word.trim();

    print!("A.. ");
    io::stdout().flush()?; 
    io::stdin().read_line(&mut ls)?;

    match fs::create_dir(word) {
        Ok(_) => println!("Directory '{}' created successfully.", word),
        Err(e) => eprintln!("Failed to create directory '{}': {}", word, e),
    }

    for i in 0..7 {
        if let Some(character) = char::from_u32(i + 65) {
            let file_name = format!("{}/{}.cpp", word, character);
            let file_content = r#"#include <iostream>
#include <bits/c++io.h>
#include <string>
using namespace std;

int main() {
    
}"#;

            match fs::write(&file_name, file_content) {
                Ok(_) => println!("Created {}.cpp", character),
                Err(e) => eprintln!("Failed to create {}.cpp: {}", character, e),
            }
        }
    }

    println!("All Completed!");
    Ok(())
}
