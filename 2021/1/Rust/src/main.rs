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

    println!("Part 1 Count: {count}");
    part_two()
}

fn part_two() {
    // let test_input = [199, 200, 208, 210, 200, 207, 240, 269, 260, 263];
    let input =
        fs::read_to_string("./input.txt").expect("Something went wrong reading the input file");
    let numbers: Vec<u16> = input.split('\n').map(|x| x.parse().unwrap()).collect();

    let mut count = 0;

    for (i, _num) in numbers.iter().enumerate() {
        if i < numbers.len() - 3 {
            let window = [numbers[i], numbers[i + 1], numbers[i + 2]];
            let next_window = [numbers[i + 1], numbers[i + 2], numbers[i + 3]];

            let window_sum = window.iter().fold(0, |acc, cur| acc + cur);
            let next_window_sum = next_window.iter().fold(0, |acc, cur| acc + cur);

            if next_window_sum > window_sum {
                count += 1;
            }
        }
    }

    println!("Part 2 Count: {count}")
}
