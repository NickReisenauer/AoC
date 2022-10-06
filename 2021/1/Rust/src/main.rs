use std::fs;

fn main() {
    // let test_input = [199, 200, 208, 210, 200, 207, 240, 269, 260, 263];
    let input =
        fs::read_to_string("./input.txt").expect("Something went wrong reading the input file");
    let numbers: Vec<u16> = input.split('\n').map(|x| x.parse().unwrap()).collect();

    let mut count = 0;

    for (i, _num) in numbers.iter().enumerate() {
        let current = numbers[i];
        let prev_index = if i == 0 { 0 } else { i - 1 };
        let prev = numbers[prev_index];
        if current > prev {
            count += 1
        }
    }

    println!("Count: {count}");
}
