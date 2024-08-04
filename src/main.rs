use std::io;
use std::fs;

fn search(_city: &str) {
    let file = fs::read_to_string("data.csv").unwrap();
    let data = file.lines().collect::<Vec<&str>>();
    match data.iter().find(|&line| line.contains(_city)) {
        Some(line) => println!("{}", line),
        None => println!("No city found"),
    }
}

fn main() {
    println!("Input a city name");
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let city = input.trim();
    println!("Searching for {}", city);
    search(city);
}