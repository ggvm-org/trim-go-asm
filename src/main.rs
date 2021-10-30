use std::{
    io::{self, stdin, Read},
    process,
};

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

fn replace_abiinternal(inst_str: String) -> String {
    // 4 means NOSPLIT
    inst_str
        .replace("NOSPLIT|ABIInternal", "4")
        .replace("ABIInternal", "4")
}

fn remove_pcdata_funcdata(inst: Vec<String>) -> Vec<String> {
    inst.into_iter()
        .filter(|ins| !ins.starts_with("FUNCDATA") && !ins.starts_with("PCDATA"))
        .collect()
}

fn rename_for_mac(inst_str: String) -> String {
    inst_str
        .replacen("\"\".", "main·", 1)
        .replace("\"\".", "")
        .replace("~", "")
}

fn identity<T>(v: T) -> T {
    v
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

    if matches.is_present("REMOVE_PCDATA_FUNCDATA") && !matches.is_present("TRIM_GOROUTINE") {
        eprintln!("if --rpf is enabled, you must enable --tg");
        process::exit(1)
    }

    let trim_goroutine_fn = if matches.is_present("TRIM_GOROUTINE") {
        trim_goroutine_instructions
    } else {
        identity
    };

    let replace_abi_fn = if matches.is_present("REPLACE_ABIINTERNAL") {
        replace_abiinternal
    } else {
        identity
    };

    let remove_pcdata_func_data_fn = if matches.is_present("REMOVE_PCDATA_FUNCDATA") {
        remove_pcdata_funcdata
    } else {
        identity
    };

    let rename_for_mac_fn = if matches.is_present("FOR_MAC") {
        rename_for_mac
    } else {
        identity
    };

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
                        // Skip "	0x0009 00009 (x.go:3)	"
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
    let x = trim_goroutine_fn(x);
    let x = remove_pcdata_func_data_fn(x);
    let x = replace_abi_fn(x.join("\n"));
    let x = rename_for_mac_fn(x);
    println!("{}", x);

    Ok(())
}
