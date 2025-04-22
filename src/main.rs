mod hosts;
mod utils;

fn main() {
    println!("Let's Relax!");
    let arguments: Vec<String> = std::env::args().map(|arg| arg).collect();
    if arguments.len() < 1 {
        panic!("arguments must be defined");
    }
    if arguments
        .get(1)
        .expect("No arguments provided")
        .to_lowercase()
        == "help"
    {
        utils::utils::help();
    }
}
