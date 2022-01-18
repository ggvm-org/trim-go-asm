use std::{
    convert::identity,
    io::{self, stdin, Read},
    process,
};

use clap::ArgMatches;

use crate::tree::{Instruction, Instructions};

pub fn trim_goroutine_instructions(instructions: Vec<String>) -> Vec<String> {
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

pub fn replace_abiinternal(inst_str: String) -> String {
    // 4 means NOSPLIT
    inst_str
        .replace("NOSPLIT|ABIInternal", "4")
        .replace("ABIInternal", "4")
}

pub fn remove_pcdata_funcdata(inst: Vec<String>) -> Vec<String> {
    inst.into_iter()
        .filter(|ins| !ins.starts_with("FUNCDATA") && !ins.starts_with("PCDATA"))
        .collect()
}

pub fn rename_for_mac(inst_str: String) -> String {
    inst_str
        .replacen("\"\".", "main·", 1)
        .replace("\"\".", "")
        .replace("~", "")
}

pub fn erase_common(inst_str: String) -> Vec<String> {
    inst_str
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
        .collect()
}

pub fn new_erase_common(inst_str: String) -> Instructions {
    Instructions::new(
        inst_str
            .lines()
            .filter_map(|line| {
                if line.split('\t').count() > 2 {
                    let line = line.trim().to_string();
                    dbg!(&line);
                    Some(Instruction::from(line))
                } else {
                    None
                }
            })
            .collect(),
    )
}

pub fn run(matches: ArgMatches) -> io::Result<()> {
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

    let mut inst_str = String::new();
    stdin().read_to_string(&mut inst_str)?;

    let mut insts = new_erase_common(inst_str);
    dbg!(&insts);
    dbg!(insts.trim_goroutine_instructions());
    let insts = insts.optimize_for_me();
    println!("{}", insts);
    // let insts = trim_goroutine_fn(insts);
    // let insts = remove_pcdata_func_data_fn(insts);
    // let inst_str = replace_abi_fn(insts.join("\n"));
    // let inst_str = rename_for_mac_fn(inst_str);
    // println!("{}", inst_str);
    Ok(())
}
