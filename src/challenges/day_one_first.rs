use std::fs;

#[allow(dead_code)]
pub fn challenge(filename: &str)
{
    println!("Parsing file {}", filename);

    let contents = fs::read_to_string(filename)
        .expect("Something went wrong reading the file");

    let depths: Vec<i32> = contents.lines().map(|line| line.parse::<i32>().unwrap()).collect();

    let mut num_increases = 0;
    let mut prev_depth = depths[0];

    depths.iter().skip(1).for_each(|depth| {
        if *depth > prev_depth {
            num_increases += 1;
        }

        prev_depth = *depth;
    });

    println!("Challenge 1/1 answer: {}", num_increases);
}