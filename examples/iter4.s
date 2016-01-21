	.text
	.file	"iter4.0.rs"
	.section	.text._ZN4main20h91a212910c7d66e6eaaE,"ax",@progbits
	.align	16, 0x90
	.type	_ZN4main20h91a212910c7d66e6eaaE,@function
_ZN4main20h91a212910c7d66e6eaaE:
	.cfi_startproc
	pushq	%rax
.Ltmp0:
	.cfi_def_cfa_offset 16
	movl	$45, %edi
	callq	_ZN7process4exit20h6c4f863c37f7195dKAmE@PLT
.Lfunc_end0:
	.size	_ZN4main20h91a212910c7d66e6eaaE, .Lfunc_end0-_ZN4main20h91a212910c7d66e6eaaE
	.cfi_endproc

	.section	.text.main,"ax",@progbits
	.globl	main
	.align	16, 0x90
	.type	main,@function
main:
	.cfi_startproc
	movq	%rsi, %rax
	movq	%rdi, %rcx
	leaq	_ZN4main20h91a212910c7d66e6eaaE(%rip), %rdi
	movq	%rcx, %rsi
	movq	%rax, %rdx
	jmp	_ZN2rt10lang_start20h58cfae38546804729kxE@PLT
.Lfunc_end1:
	.size	main, .Lfunc_end1-main
	.cfi_endproc


	.section	".note.GNU-stack","",@progbits
