use std::io;
use std::io::{BufRead, BufReader, Lines};

pub struct File(std::fs::File);

impl File {
    pub fn lines(self) -> impl Iterator<Item=String> {
        io::BufReader::new(self.0).lines().map(|s| s.unwrap())
    }

    pub fn try_lines(self) -> Lines<BufReader<std::fs::File>> {
        io::BufReader::new(self.0).lines()
    }
}

pub fn exists<P: AsRef<std::path::Path>>(path: P) -> bool {
    std::fs::metadata(path).is_ok()
}

pub fn open<P: AsRef<std::path::Path>>(path: P) -> io::Result<File> {
    std::fs::OpenOptions::new().read(true).open(path.as_ref()).map(File)
}

pub fn open_rw<P: AsRef<std::path::Path>>(path: P) -> io::Result<File> {
    std::fs::OpenOptions::new().read(true).write(true).open(path.as_ref()).map(File)
}

pub fn create<P: AsRef<std::path::Path>>(path: P) -> io::Result<File> {
    std::fs::OpenOptions::new().write(true).create(true).truncate(true).open(path.as_ref()).map(File)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lines() -> io::Result<()> {
        let f = open("Cargo.toml")?;
        for line in f.lines() {
            println!("{}", line);
        }
        assert_eq!(1, 1);
        Ok(())
    }
}
