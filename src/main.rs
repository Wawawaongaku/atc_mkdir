use std::fs;

fn main() {
    let mut word = String::new();
    let mut ls = String::new();
    println!("Please input contest name (Ex: (ABC123)) >");
    std::io::stdin().read_line(&mut word).ok();
    println!("A..");
    std::io::stdin().read_line(&mut ls).ok();
    let _ = fs::create_dir(word.clone());
    for i in 0..7 {
        if let Some(character) = char::from_u32(i + 65) {
        let _ = fs::write(word.clone() + "/" + &character.to_string() + ".cpp", "#include <iostream>\n#include <bits/c++io.h>\n#include <string>\nusing namespace std;\n\nint main() {\n\t\n}");
        println!("{}" , "Created ".to_owned() + &character.to_string() + ".cpp");
        }
    }
    println!("{}", "All Completed!");
}
