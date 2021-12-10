use std::{
    fs::{create_dir, File},
    io::{self, Write},
    path::PathBuf,
};

fn main() -> io::Result<()> {
    const BOILER_PLATE: &'static str = r#"fn main() {
    println!("Hello, world!");
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
