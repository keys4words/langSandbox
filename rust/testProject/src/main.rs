use std::io;

fn main() {
    let mut inp = String::new();
    println!("Input something serious...");
    match io::stdin().read_line(&mut inp) {
        Ok(_) => {
            println!("You said: {}", inp);
        },
        Err(e) => {
            println!("Shit happens... With {}", e);
        }
    }
}
