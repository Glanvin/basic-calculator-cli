use std::io;

fn main() {
    println!("Hello, world!\n");

    let ivy = "Ivy";
    let ivy_age = 24;
    println!("Hello, I'm {} and I'm {} years old!", ivy, ivy_age);

    let nyo = "Nyo";
    let nyo_age = 23;
    println!("\nHello, I'm {} and I'm {} years old!", nyo, nyo_age);

    println!("What's your name?\nEnter your name: ");

    let mut user = String::new();
    io::stdin().read_line(&mut user).unwrap_or_default();
    println!("Hello, {}!", user.trim());
}
