TEXT	"".main(SB), ABIInternal, $16-0
SUBQ	$16, SP
MOVQ	BP, 8(SP)
LEAQ	8(SP), BP
NOP
CALL	runtime.printlock(SB)
MOVQ	$3, (SP)
CALL	runtime.printint(SB)
CALL	runtime.printnl(SB)
CALL	runtime.printunlock(SB)
MOVQ	8(SP), BP
ADDQ	$16, SP
RET
