use std::io;

mod trim;

use clap::{App, Arg};
use trim::run;

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
        .arg(
            Arg::new("REPLACE_ABIINTERNAL")
                .about("Replace `ABIInternal` to 4(NOSPLIT)")
                .takes_value(false)
                .long("ra"),
        )
        .arg(
            Arg::new("REMOVE_PCDATA_FUNCDATA")
                .about("Remove PCDATA and FUNCDATA insts, if you want to enable this option, you must enable --tg too.")
                .takes_value(false)
                .long("rpf"),
        ).arg(Arg::new("FOR_MAC").about("todo!").takes_value(false).long("fm"))
        .get_matches();
    run(matches)?;
    Ok(())
}
