use std::{
    fs::File,
    io::{self, Read},
};

fn main() -> io::Result<()> {
    let mut f = File::open("src/day2/input.txt")?;
    let mut buf = String::new();
    f.read_to_string(&mut buf)?;

    let mut submarine = Submarine::<Part1>::default();
    submarine.run_instructions(buf.as_str());
    println!("part1: {}", submarine.depth * submarine.distance);

    let mut submarine = Submarine::<Part2>::default();
    submarine.run_instructions(buf.as_str());
    println!("part2: {}", submarine.depth * submarine.distance);

    Ok(())
}

type Unit = i64;

trait Operate {
    fn operate(&mut self, c: Command);
}

#[derive(Default)]
struct Part1;
#[derive(Default)]
struct Part2;

#[derive(Default)]
struct Submarine<Part> {
    distance: Unit,
    depth: Unit,
    aim: Unit,
    phantom: std::marker::PhantomData<Part>,
}

impl<Part> Submarine<Part>
where
    Self: Operate,
{
    fn run_instructions(&mut self, commands: &str) {
        commands
            .lines()
            .into_iter()
            .for_each(|c| self.operate(Command::try_from(c).unwrap()));
    }
}

impl Operate for Submarine<Part1> {
    fn operate(&mut self, c: Command) {
        match c {
            Command::Forward(n) => self.distance += n,
            Command::Down(n) => self.depth += n,
            Command::Up(n) => self.depth -= n,
        }
    }
}

impl Operate for Submarine<Part2> {
    fn operate(&mut self, c: Command) {
        match c {
            Command::Forward(n) => {
                self.distance += n;
                self.depth += self.aim * n
            }
            Command::Down(n) => self.aim += n,
            Command::Up(n) => self.aim -= n,
        }
    }
}

enum Command {
    Forward(Unit),
    Down(Unit),
    Up(Unit),
}

impl TryFrom<&str> for Command {
    type Error = Box<dyn std::error::Error>;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let mut parts = value.split(' ');
        let command = parts.next().unwrap();
        let amount = parts.next().unwrap().parse().unwrap();
        match command {
            "forward" => Ok(Command::Forward(amount)),
            "down" => Ok(Command::Down(amount)),
            "up" => Ok(Command::Up(amount)),
            c => Err(Box::new(io::Error::new(io::ErrorKind::InvalidInput, c))),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{Part1, Part2, Submarine};

    const COMMANDS: &'static str = "forward 5
down 5
forward 8
up 3
down 8
forward 2";

    #[test]
    fn test_submarine_commands_part1() {
        let mut submarine = Submarine::<Part1>::default();
        submarine.run_instructions(COMMANDS);

        assert_eq!(submarine.depth * submarine.distance, 150);
    }

    #[test]
    fn test_submarine_commands_part2() {
        let mut submarine = Submarine::<Part2>::default();
        submarine.run_instructions(COMMANDS);

        assert_eq!(submarine.depth * submarine.distance, 900);
    }
}
