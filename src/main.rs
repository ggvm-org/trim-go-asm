use std::io::{self, stdin, Read};

fn main() -> io::Result<()> {
    let mut buffer = String::new();
    stdin().read_to_string(&mut buffer)?;

    let x: Vec<_> = buffer
        .lines()
        .filter_map(|line| {
            if line.split('\t').count() > 2 {
                Some(line.split('\t').skip(2).collect::<Vec<_>>().join("\t"))
            } else {
                None
            }
        })
        .collect();
    dbg!(&x);
    println!("{}", x.join("\n"));
    Ok(())
}
