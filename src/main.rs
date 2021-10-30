use std::io::{self, stdin, Read};

fn main() -> io::Result<()> {
    let mut buffer = String::new();
    stdin().read_to_string(&mut buffer)?;

    let x: Vec<_> = buffer
        .lines()
        .filter(|line| line.split('\t').count() > 3)
        .map(|line| line.split('\t').skip(2).collect::<Vec<_>>().join("\t"))
        .collect();
    println!("{}", x.join("\n"));
    Ok(())
}
