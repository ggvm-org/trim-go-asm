# trim-go-asm
Trim Go Assembly from $ go tool compile -S

```
USAGE:
    trim-go-asm [OPTIONS]

OPTIONS:
        --fm         todo!
    -h, --help       Print help information
        --ra         Replace `ABIInternal` to 4(NOSPLIT)
        --rpf        Remove PCDATA and FUNCDATA insts, if you want to enable this option, you must
                     enable --tg too.
        --tg         Trim Goroutine prologue / epilogue
                     // Trim these instructions.
                     MOVQ       (TLS), CX
                     CMPQ       SP, 16(CX)
                     PCDATA     $0, $-2
                     JLS        70
                     PCDATA     $0, $-1
                     // ...
                     NOP
                     PCDATA     $1, $-1
                     PCDATA     $0, $-2
                     CALL       runtime.morestack_noctxt(SB)
                     PCDATA     $0, $-1
                     JMP        0
    -V, --version    Print version information
```
