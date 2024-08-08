use std::io;
use std::fs;
use std::time::Instant;

fn search(city: &str) {
    let file = fs::read_to_string("data.csv").unwrap();
    let data = file.lines().collect::<Vec<&str>>();
    let now = Instant::now();
    match data.iter().find(|&line| line.contains(city)) {
        Some(line) => println!("{} Took {} microseconds to find.", line, now.elapsed().as_micros()),
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

#[cfg(test)]
mod test {
    use std::fs;
    use std::time::Instant;
    fn search(city: &str) {
        let file = fs::read_to_string("data.csv").unwrap();
        let data: Vec<String> = file.lines().map(|line| line.to_string()).collect();
        let now = Instant::now();
        match data.iter().find(|&line| line.contains(city)) {
            Some(line) => println!("{} Took {} microseconds to find.", line, now.elapsed().as_micros()),
            None => println!("No city found"),
        }
    }

    #[test]
    fn test() {
        let input = "Nordvik";
        let city = input.trim();
        println!("Searching for {}", city);
        search(city);
    }
}