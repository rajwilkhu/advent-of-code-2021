use std::fs;

fn main() {
    let filename = "input.txt";

    println!("In file {}", filename);

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

    println!("Number of increase: {}", num_increases);
}