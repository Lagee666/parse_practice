use std::io;

fn main() {
    let mut input = String::new();

    println!("Enter a list of integers separated by commas:");
    io::stdin().read_line(&mut input).expect("Failed to read input");

    let numbers: Vec<i32> = input           //String
        .split(',')                //Split<'_, char>
        .map(|s| s.trim())   //impl Iterator<Item = &str>
        .filter_map(|s| s.parse().ok()) //impl Iterator<Item = i32>
        .collect();

    if !numbers.is_empty() {
        let sum: i32 = numbers.iter().sum();
        let min: i32 = *numbers.iter().min().unwrap();
        let max: i32 = *numbers.iter().max().unwrap();

        println!("Sum: {}", sum);
        println!("Minimum: {}", min);
        println!("Maximum: {}", max);
    } else {
        println!("No valid integers entered.");
    }
}
