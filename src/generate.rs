use std::{
    fs::{create_dir, File},
    io::{self, Write},
    path::PathBuf,
};

fn main() -> io::Result<()> {
    const BOILER_PLATE: &str = r#"#![deny(clippy::all, clippy::pedantic)]

use std::{
    fs::File,
    io::{self, Read},
};

fn main() -> io::Result<()> {
    let mut f = File::open("src/day1/input.txt")?;
    let mut buf = String::new();
    f.read_to_string(&mut buf)?;

}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(1, 1)
    }
}"#;

    for day in 1..=25 {
        let path = format!("src/day{}", day);
        create_dir(&path)?;

        let main: PathBuf = [&path, "main.rs"].iter().collect();
        let mut f = File::create(main)?;
        write!(&mut f, "{}", BOILER_PLATE)?;

        let input: PathBuf = [&path, "input.txt"].iter().collect();
        File::create(input)?;
    }

    Ok(())
}
