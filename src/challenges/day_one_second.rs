use std::fs;

#[allow(dead_code)]
pub fn challenge(filename: &str)
{
    println!("Parsing file {}", filename);

    let contents = fs::read_to_string(filename)
        .expect("Something went wrong reading the file");

    let sonar_reading: Vec<i32> = contents.lines().map(|line| line.parse::<i32>().unwrap()).collect();

    let mut num_increases = 0;
    let mut prev_depth = 0;

    for (pos, depth) in sonar_reading[0..(sonar_reading.len() - 3)].iter().enumerate() {
        if pos > sonar_reading.len() - 3 {
            break;
        }

        let slice = &sonar_reading[pos..(pos + 3)];
        let current_depth = slice[0] + slice[1] + slice[2];

        // println!("Sum {}", slice.iter().sum());

        println!("Prev depth is {}", prev_depth);
        if pos > 0 && current_depth > prev_depth {
            num_increases = num_increases + 1;
        }
        prev_depth = current_depth;
    }

    println!("Challenge 1 answer: {}", num_increases);
}