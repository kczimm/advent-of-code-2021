use std::{
    fs::File,
    io::{self, Read},
};

fn main() -> io::Result<()> {
    let mut f = File::open("src/day3/input.txt")?;
    let mut buf = String::new();
    f.read_to_string(&mut buf)?;

    let (gamma_rate, epsilon_rate) = calculate_rates(buf.as_str());
    let power_consumption = gamma_rate * epsilon_rate;
    println!("part1: {}", power_consumption);

    Ok(())
}

type _OxygenGeneratorRating = usize;
type _CO2ScrubberRating = usize;

fn _calculate_oxygen_generator_rating(report: &str) -> _OxygenGeneratorRating {
    let binary_numbers = report.lines().into_iter().collect::<Vec<&str>>();

    let numbers_len = binary_numbers.len();
    assert!(numbers_len > 0, "report has no lines");

    let bit_depth = binary_numbers[0].len();

    for i in 0..bit_depth {
        let _num_ones = binary_numbers
            .iter()
            .filter(|number| number.chars().nth(i).unwrap() == '1')
            .count();
    }

    0
}

type GammaRate = usize;
type EpsilonRate = usize;

fn calculate_rates(report: &str) -> (GammaRate, EpsilonRate) {
    let mut position_counts = Vec::new();
    let mut numbers = 0;
    for binary_number in report.lines() {
        for (i, c) in binary_number.chars().enumerate() {
            if position_counts.len() < i + 1 {
                position_counts.push(0);
            }
            match c {
                '1' => position_counts[i] += 1,
                '0' => continue,
                c => panic!("bad bit: {}", c),
            }
        }
        numbers += 1;
    }
    let mut gamma_rate: GammaRate = 0;
    let mut epsilon_rate: EpsilonRate = 0;

    for (i, count) in position_counts.iter().rev().enumerate() {
        if *count > numbers / 2 {
            // most common bit is 1
            gamma_rate += 1 << i;
        } else {
            // least common bit is 1
            epsilon_rate += 1 << i;
        }
    }

    (gamma_rate, epsilon_rate)
}

#[cfg(test)]
mod tests {
    use crate::calculate_rates;

    const INPUT: &'static str = "00100
11110
10110
10111
10101
01111
00111
11100
10000
11001
00010
01010";

    #[test]
    fn test_part1_example() {
        let (gamma_rate, epsilon_rate) = calculate_rates(INPUT);
        let power_consumption = gamma_rate * epsilon_rate;
        assert_eq!(power_consumption, 198);
    }
}
