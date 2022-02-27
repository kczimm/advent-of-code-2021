//! # Day 2: Dive!
//! Now, you need to figure out how to pilot this thing.
//!
//! It seems like the [Submarine] can take a series of [Command]s like forward 1, down 2, or up 3:
//!
//! - forward X increases the horizontal position by X [Unit]s.
//! - down X increases the depth by X [Unit]s.
//! - up X decreases the depth by X [Unit]s.
//!
//! Note that since you're on a [Submarine], down and up affect your depth, and so they have the opposite result of what you might expect.
//!
//! The [Submarine] seems to already have a planned course (your puzzle input). You should probably figure out where it's going. For example:
//! ```not_rust
//! forward 5
//! down 5
//! forward 8
//! up 3
//! down 8
//! forward 2
//! ```
//! Your horizontal position and depth both start at 0. The steps above would then modify them as follows:
//!
//! - forward 5 adds 5 to your horizontal position, a total of 5.
//! - down 5 adds 5 to your depth, resulting in a value of 5.
//! - forward 8 adds 8 to your horizontal position, a total of 13.
//! - up 3 decreases your depth by 3, resulting in a value of 2.
//! - down 8 adds 8 to your depth, resulting in a value of 10.
//! - forward 2 adds 2 to your horizontal position, a total of 15.
//!
//! After following these instructions, you would have a horizontal position of 15 and a depth of 10. (Multiplying these together produces 150.)
//!
//! Calculate the horizontal position and depth you would have after following the planned course. What do you get if you multiply your final horizontal position by your final depth?
//!
//! # Part Two
//! Based on your calculations, the planned course doesn't seem to make any sense. You find the [Submarine] manual and discover that the process is actually slightly more complicated.
//!
//! In addition to horizontal position and depth, you'll also need to track a third value, aim, which also starts at 0. The [Command]s also mean something entirely different than you first thought:
//!
//! - down X increases your aim by X [Unit]s.
//! - up X decreases your aim by X [Unit]s.
//! - forward X does two things:
//! - It increases your horizontal position by X [Unit]s.
//! - It increases your depth by your aim multiplied by X.
//! Again note that since you're on a [Submarine], down and up do the opposite of what you might expect: "down" means aiming in the positive direction.
//!
//! Now, the above example does something different:
//!
//! - forward 5 adds 5 to your horizontal position, a total of 5. Because your aim is 0, your depth does not change.
//! - down 5 adds 5 to your aim, resulting in a value of 5.
//! - forward 8 adds 8 to your horizontal position, a total of 13. Because your aim is 5, your depth increases by 8*5=40.
//! - up 3 decreases your aim by 3, resulting in a value of 2.
//! - down 8 adds 8 to your aim, resulting in a value of 10.
//! - forward 2 adds 2 to your horizontal position, a total of 15. Because your aim is 10, your depth increases by 2*10=20 to a total of 60.
//! After following these new instructions, you would have a horizontal position of 15 and a depth of 60. (Multiplying these produces 900.)
//!
//! Using this new interpretation of the [Command]s, calculate the horizontal position and depth you would have after following the planned course. What do you get if you multiply your final horizontal position by your final depth?

use std::{
    fs::File,
    io::{self, Read},
};

fn main() -> io::Result<()> {
    let mut f = File::open("src/day2/input.txt")?;
    let mut buf = String::new();
    f.read_to_string(&mut buf)?;

    let mut submarine = Submarine::<Naive>::default();
    submarine.run_instructions(buf.as_str());
    println!("part 1: {}", submarine.depth * submarine.distance);

    let mut submarine = Submarine::<Complicated>::default();
    submarine.run_instructions(buf.as_str());
    println!("part 2: {}", submarine.depth * submarine.distance);

    Ok(())
}

/// A [Unit] of both position and depth of the [Submarine].
pub type Unit = i64;

pub trait Operate {
    fn operate(&mut self, c: Command);
}

#[derive(Default)]
struct Naive;

#[derive(Default)]
struct Complicated;

#[derive(Default)]
pub struct Submarine<Calculation> {
    distance: Unit,
    depth: Unit,
    aim: Unit,
    phantom: std::marker::PhantomData<Calculation>,
}

impl<Calculation> Submarine<Calculation>
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

impl Operate for Submarine<Naive> {
    fn operate(&mut self, c: Command) {
        match c {
            Command::Forward(n) => self.distance += n,
            Command::Down(n) => self.depth += n,
            Command::Up(n) => self.depth -= n,
        }
    }
}

impl Operate for Submarine<Complicated> {
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

pub enum Command {
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
    use super::{Complicated, Naive, Submarine};

    const COMMANDS: &'static str = "forward 5
down 5
forward 8
up 3
down 8
forward 2";

    #[test]
    fn test_submarine_commands_naive() {
        let mut submarine = Submarine::<Naive>::default();
        submarine.run_instructions(COMMANDS);

        assert_eq!(submarine.depth * submarine.distance, 150);
    }

    #[test]
    fn test_submarine_commands_complicated() {
        let mut submarine = Submarine::<Complicated>::default();
        submarine.run_instructions(COMMANDS);

        assert_eq!(submarine.depth * submarine.distance, 900);
    }
}
