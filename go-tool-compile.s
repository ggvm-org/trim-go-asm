TEXT	"".main(SB), ABIInternal, $16-0
MOVQ	(TLS), CX
CMPQ	SP, 16(CX)
PCDATA	$0, $-2
JLS	70
PCDATA	$0, $-1
SUBQ	$16, SP
MOVQ	BP, 8(SP)
LEAQ	8(SP), BP
FUNCDATA	$0, gclocals·33cdeccccebe80329f1fdbee7f5874cb(SB)
FUNCDATA	$1, gclocals·33cdeccccebe80329f1fdbee7f5874cb(SB)
PCDATA	$1, $0
NOP
CALL	runtime.printlock(SB)
MOVQ	$3, (SP)
CALL	runtime.printint(SB)
CALL	runtime.printnl(SB)
CALL	runtime.printunlock(SB)
MOVQ	8(SP), BP
ADDQ	$16, SP
RET
NOP
PCDATA	$1, $-1
PCDATA	$0, $-2
CALL	runtime.morestack_noctxt(SB)
PCDATA	$0, $-1
JMP	0
