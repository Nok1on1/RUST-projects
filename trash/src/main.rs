use std::io::stdin;
fn main() {
    let hello = println!("hello what's your name?");
    let mut name = String::new();
    stdin().read_line(&mut name).expect("failed");
        
}
