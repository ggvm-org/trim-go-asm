use std::io::{self, stdin, Read};

fn main() -> io::Result<()> {
    let mut buffer = String::new();
    stdin().read_to_string(&mut buffer)?;

    let x: Vec<_> = buffer
        .lines()
        .filter_map(|line| {
            // Drop the line like
            //     0x0000 65 48 8b 0c 25 00 00 00 00 48 3b 61 10 76 37 48  eH..%....H;a.v7H
            //     ...
            //     rel 5+4 t=17 TLS+0
            //     ...
            // go.cuinfo.packagename. SDWARFCUINFO dupok size=0
            // ""..inittask SNOPTRDATA size=24
            // gclocals·33cdeccccebe80329f1fdbee7f5874cb SRODATA dupok size=8
            if line.split('\t').count() > 2 {
                // skip(2) means
                // Let "	0x0009 00009 (x.go:3)	CMPQ	SP, 16(CX)" be trimed to "CMPQ	SP, 16(CX)"
                Some(
                    line
                        // Optimize for me
                        // .replace("\"\".", "main·")
                        .split('\t')
                        .skip(2)
                        .collect::<Vec<_>>()
                        .join("\t"),
                )
            } else {
                None
            }
        })
        .collect();
    dbg!(&x);
    println!("{}", x.join("\n"));
    Ok(())
}
