use std::io;

fn main() {
    println!("Enter a rational number.");

    let mut p_q = String::new();

    io::stdin()
        .read_line(&mut p_q)
        .expect("Failed to read the line");
    
    
}
