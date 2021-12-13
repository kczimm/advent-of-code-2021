use std::{
    fs::File,
    io::{self, Read},
};

fn main() -> io::Result<()> {
    let mut f = File::open("src/day2/input.txt")?;
    let mut buf = String::new();
    f.read_to_string(&mut buf)?;

    let mut submarine = Submarine::default();

    submarine.run_instructions(buf.as_str());

    println!("part1: {}", submarine.depth * submarine.distance);

    Ok(())
}

type Unit = i64;

#[derive(Default)]
struct Submarine {
    distance: Unit,
    depth: Unit,
}

impl Submarine {
    fn operate(&mut self, c: Command) {
        match c {
            Command::Forward(n) => self.distance += n,
            Command::Down(n) => self.depth += n,
            Command::Up(n) => self.depth -= n,
        }
    }

    fn run_instructions(&mut self, commands: &str) {
        commands
            .lines()
            .into_iter()
            .for_each(|c| self.operate(Command::try_from(c).unwrap()));
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
    use super::Submarine;

    #[test]
    fn test_submarine_commands() {
        const COMMANDS: &'static str = "forward 5
down 5
forward 8
up 3
down 8
forward 2";

        let mut submarine = Submarine::default();
        submarine.run_instructions(COMMANDS);

        assert_eq!(submarine.depth * submarine.distance, 150);
    }
}
