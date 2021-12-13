use std::{
    fs::File,
    io::{self, Read},
};

fn main() -> io::Result<()> {
    let mut f = File::open("src/day1/input.txt")?;
    let mut buf = String::new();
    f.read_to_string(&mut buf)?;

    let report = Report::try_from(buf.as_str()).expect("parsing input failed");

    println!("part1: {}", report.num_increases(1));
    println!("part2: {}", report.num_increases(3));

    Ok(())
}

type Depth = i64;

struct Report {
    measurements: Vec<Depth>,
}

impl Report {
    fn num_increases(&self, window_size: usize) -> usize {
        assert!(window_size > 0, "window size must be positive");

        let mut num = 0;

        if self.measurements.len() > window_size {
            for i in window_size..self.measurements.len() {
                if (self.measurements[i] - self.measurements[i - window_size]) > 0 {
                    num += 1;
                }
            }
        }

        num
    }
}

impl TryFrom<&str> for Report {
    type Error = std::num::ParseIntError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let mut measurements = Vec::new();

        for number in value.lines() {
            let measurement = number.parse::<Depth>()?;
            measurements.push(measurement);
        }

        Ok(Self { measurements })
    }
}

#[cfg(test)]
mod tests {
    use crate::Report;

    const INPUT: &'static str = "199
200
208
210
200
207
240
269
260
263";

    #[test]
    fn number_of_increases_window_size_1() {
        let report = Report::try_from(INPUT).unwrap();
        let window_size = 1;
        assert_eq!(report.num_increases(window_size), 7);
    }

    #[test]
    fn number_of_increases_window_size_3() {
        let report = Report::try_from(INPUT).unwrap();

        let window_size = 3;
        assert_eq!(report.num_increases(window_size), 5);
    }
}
