use std::io::{self, stdin, Read};

use clap::{App, Arg};

fn trim_goroutine_instructions(instructions: Vec<String>) -> Vec<String> {
    // Skip these instructions (1..=5)
    // MOVQ	(TLS), CX
    // CMPQ	SP, 16(CX)
    // PCDATA	$0, $-2
    // JLS	70
    // PCDATA	$0, $-1

    // Skip these instrutions (count - 6 ..= count)
    // NOP
    // PCDATA  $1, $-1
    // PCDATA  $0, $-2
    // CALL    runtime.morestack_noctxt(SB)
    // PCDATA  $0, $-1
    // JMP     0

    let count = instructions.len();

    instructions
        .into_iter()
        .enumerate()
        .filter(|(idx, _)| !(1..=5).contains(idx) && !(count - 6..=count).contains(idx))
        .map(|(_, line)| line)
        .collect()
}

fn main() -> io::Result<()> {
    let matches = App::new("trim-go-asm")
        .version("0.1")
        .author("Krouton <me+git@tokinia.me>")
        .about("Trim Go Assembly from $ go tool compile -S")
        .arg(
            Arg::new("TRIM_GOROUTINE")
                .about(
                    r#"Trim Goroutine prologue / epilogue
// Trim these instructions.
MOVQ	(TLS), CX
CMPQ	SP, 16(CX)
PCDATA	$0, $-2
JLS	70
PCDATA	$0, $-1
// ...
NOP
PCDATA	$1, $-1
PCDATA	$0, $-2
CALL	runtime.morestack_noctxt(SB)
PCDATA	$0, $-1
JMP	0"#,
                )
                .takes_value(false)
                .long("tg"),
        )
        .get_matches();

    let trim_goroutine_routine = matches.is_present("TRIM_GOROUTINE");

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
    let x = if trim_goroutine_routine {
        trim_goroutine_instructions(x)
    } else {
        x
    };
    println!("{}", x.join("\n"));

    Ok(())
}
