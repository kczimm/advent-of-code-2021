use std::{
    fs::File,
    io::{self, Read},
};

fn main() -> io::Result<()> {
    let mut f = File::open("src/day1/input.txt")?;
    let mut buf = String::new();
    f.read_to_string(&mut buf)?;

    let report = Report::try_from(buf.as_str()).expect("parsing input failed");

    println!("part1: {}", report.num_increases());

    Ok(())
}

type Depth = i64;

struct Report {
    measurements: Vec<Depth>,
}

impl Report {
    fn num_increases(&self) -> usize {
        let mut num = 0;

        if self.measurements.len() > 1 {
            for i in 1..self.measurements.len() {
                if (self.measurements[i] - self.measurements[i - 1]) > 0 {
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

    #[test]
    fn number_of_increases() {
        let input = "199
200
208
210
200
207
240
269
260
263";
        let report = Report::try_from(input).unwrap();

        assert_eq!(report.num_increases(), 7);
    }
}
