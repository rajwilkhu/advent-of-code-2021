use std::fs;

#[allow(dead_code)]
pub fn challenge(filename: &str)
{
    println!("Parsing file {}", filename);

    let contents = fs::read_to_string(filename)
        .expect("Something went wrong reading the file");

    let sonar_readings: Vec<i32> = contents.lines().map(|line| line.parse::<i32>().unwrap()).collect();

    let mut num_increases = 0;
    let mut prev_depth = 0;

    for (pos, depth) in sonar_readings[0..(sonar_readings.len() - 2)].iter().enumerate() {
        let slice = &sonar_readings[pos..(pos + 3)];
        let current_depth:i32 = slice.iter().sum();

        println!("Prev depth is {}", prev_depth);

        if pos > 0 && current_depth > prev_depth {
            num_increases = num_increases + 1;
        }
        prev_depth = current_depth;
    }

    println!("Challenge 1 answer: {}", num_increases);
}