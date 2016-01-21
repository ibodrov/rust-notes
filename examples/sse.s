	.text
	.file	"sse.0.rs"
	.section	.text._ZN4main20h6a5e5ca902ed7f7aRaaE,"ax",@progbits
	.align	16, 0x90
	.type	_ZN4main20h6a5e5ca902ed7f7aRaaE,@function
_ZN4main20h6a5e5ca902ed7f7aRaaE:
.Lfunc_begin0:
	.cfi_startproc
	.cfi_personality 155, DW.ref.rust_eh_personality
	.cfi_lsda 27, .Lexception0
	pushq	%r15
.Ltmp6:
	.cfi_def_cfa_offset 16
	pushq	%r14
.Ltmp7:
	.cfi_def_cfa_offset 24
	pushq	%rbx
.Ltmp8:
	.cfi_def_cfa_offset 32
.Ltmp9:
	.cfi_offset %rbx, -32
.Ltmp10:
	.cfi_offset %r14, -24
.Ltmp11:
	.cfi_offset %r15, -16
	movl	$4096, %edi
	movl	$4, %esi
	callq	__rust_allocate@PLT
	movq	%rax, %r14
	testq	%r14, %r14
	je	.LBB0_11
	xorl	%esi, %esi
	movl	$4096, %edx
	movq	%r14, %rdi
	callq	memset@PLT
	movl	$4096, %edi
	movl	$4, %esi
	callq	__rust_allocate@PLT
	movq	%rax, %rbx
	testq	%rbx, %rbx
	je	.LBB0_2
	xorl	%esi, %esi
	movl	$4096, %edx
	movq	%rbx, %rdi
	callq	memset@PLT
	pxor	%xmm0, %xmm0
	movl	$12, %eax
	pxor	%xmm1, %xmm1
	.align	16, 0x90
.LBB0_6:
	movdqu	-48(%r14,%rax,4), %xmm3
	movdqu	-32(%r14,%rax,4), %xmm4
	movdqu	-48(%rbx,%rax,4), %xmm2
	movdqu	-32(%rbx,%rax,4), %xmm5
	pshufd	$245, %xmm2, %xmm6
	pmuludq	%xmm3, %xmm2
	pshufd	$232, %xmm2, %xmm2
	pshufd	$245, %xmm3, %xmm3
	pmuludq	%xmm6, %xmm3
	pshufd	$232, %xmm3, %xmm3
	punpckldq	%xmm3, %xmm2
	pshufd	$245, %xmm5, %xmm6
	pmuludq	%xmm4, %xmm5
	pshufd	$232, %xmm5, %xmm3
	pshufd	$245, %xmm4, %xmm4
	pmuludq	%xmm6, %xmm4
	pshufd	$232, %xmm4, %xmm4
	punpckldq	%xmm4, %xmm3
	paddd	%xmm0, %xmm2
	paddd	%xmm1, %xmm3
	movdqu	-16(%r14,%rax,4), %xmm1
	movdqu	(%r14,%rax,4), %xmm4
	movdqu	-16(%rbx,%rax,4), %xmm0
	movdqu	(%rbx,%rax,4), %xmm5
	pshufd	$245, %xmm0, %xmm6
	pmuludq	%xmm1, %xmm0
	pshufd	$232, %xmm0, %xmm0
	pshufd	$245, %xmm1, %xmm1
	pmuludq	%xmm6, %xmm1
	pshufd	$232, %xmm1, %xmm1
	punpckldq	%xmm1, %xmm0
	pshufd	$245, %xmm5, %xmm6
	pmuludq	%xmm4, %xmm5
	pshufd	$232, %xmm5, %xmm1
	pshufd	$245, %xmm4, %xmm4
	pmuludq	%xmm6, %xmm4
	pshufd	$232, %xmm4, %xmm4
	punpckldq	%xmm4, %xmm1
	paddd	%xmm2, %xmm0
	paddd	%xmm3, %xmm1
	addq	$16, %rax
	cmpq	$1036, %rax
	jne	.LBB0_6
	paddd	%xmm0, %xmm1
	pshufd	$78, %xmm1, %xmm0
	paddd	%xmm1, %xmm0
	pshufd	$229, %xmm0, %xmm1
	paddd	%xmm0, %xmm1
	movd	%xmm1, %edi
.Ltmp0:
	callq	_ZN7process4exit20h6c4f863c37f7195dKAmE@PLT
.Ltmp1:
.LBB0_9:
.Ltmp2:
	movq	%rax, %r15
	movl	$4096, %esi
	movl	$4, %edx
	movq	%rbx, %rdi
	callq	__rust_deallocate@PLT
.LBB0_10:
	movl	$4096, %esi
	movl	$4, %edx
	movq	%r14, %rdi
	callq	__rust_deallocate@PLT
	movq	%r15, %rdi
	callq	_Unwind_Resume@PLT
.LBB0_11:
	callq	_ZN3oom20h60a2387de2f8f1f2fubE@PLT
.LBB0_2:
.Ltmp3:
	callq	_ZN3oom20h60a2387de2f8f1f2fubE@PLT
.Ltmp4:
.LBB0_4:
.Ltmp5:
	movq	%rax, %r15
	jmp	.LBB0_10
.Lfunc_end0:
	.size	_ZN4main20h6a5e5ca902ed7f7aRaaE, .Lfunc_end0-_ZN4main20h6a5e5ca902ed7f7aRaaE
	.cfi_endproc
	.section	.gcc_except_table,"a",@progbits
	.align	4
GCC_except_table0:
.Lexception0:
	.byte	255
	.byte	155
	.asciz	"\266\200\200"
	.byte	3
	.byte	52
	.long	.Lfunc_begin0-.Lfunc_begin0
	.long	.Ltmp0-.Lfunc_begin0
	.long	0
	.byte	0
	.long	.Ltmp0-.Lfunc_begin0
	.long	.Ltmp1-.Ltmp0
	.long	.Ltmp2-.Lfunc_begin0
	.byte	0
	.long	.Ltmp1-.Lfunc_begin0
	.long	.Ltmp3-.Ltmp1
	.long	0
	.byte	0
	.long	.Ltmp3-.Lfunc_begin0
	.long	.Ltmp4-.Ltmp3
	.long	.Ltmp5-.Lfunc_begin0
	.byte	0
	.align	4

	.section	.text.main,"ax",@progbits
	.globl	main
	.align	16, 0x90
	.type	main,@function
main:
	.cfi_startproc
	movq	%rsi, %rax
	movq	%rdi, %rcx
	leaq	_ZN4main20h6a5e5ca902ed7f7aRaaE(%rip), %rdi
	movq	%rcx, %rsi
	movq	%rax, %rdx
	jmp	_ZN2rt10lang_start20h58cfae38546804729kxE@PLT
.Lfunc_end1:
	.size	main, .Lfunc_end1-main
	.cfi_endproc

	.hidden	DW.ref.rust_eh_personality
	.weak	DW.ref.rust_eh_personality
	.section	.data.DW.ref.rust_eh_personality,"aGw",@progbits,DW.ref.rust_eh_personality,comdat
	.align	8
	.type	DW.ref.rust_eh_personality,@object
	.size	DW.ref.rust_eh_personality, 8
DW.ref.rust_eh_personality:
	.quad	rust_eh_personality

	.section	".note.GNU-stack","",@progbits
