use std::fs;

#[allow(dead_code)]
pub fn challenge(filename: &str)
{
    println!("Parsing file {}", filename);

    let contents = fs::read_to_string(filename)
        .expect("Something went wrong reading the file");

    let instructions: Vec<(&str, i32)> = contents.lines().map(|line| {
        let line_parts: Vec<&str> = line.split(" ").collect();
        (line_parts[0], line_parts[1].parse::<i32>().unwrap())
    }).collect();

    let mut horizontal_position = 0;
    let mut depth = 0;
    let mut aim = 0;

    instructions.iter().for_each(|instruction| {
       match instruction.0 {
           "forward" => {
               horizontal_position += instruction.1;
               depth += aim * instruction.1;
           },
           "down" => aim += instruction.1,
           "up" => aim -= instruction.1,
           _ => println!("WTF?"),
       }
    });

    println!("Challenge 2/1 answer: {:?}", horizontal_position * depth);
}