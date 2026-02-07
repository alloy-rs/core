.section .text.alloy_primitives::utils::keccak_cache::compute::<alloy_primitives::utils::keccak256_impl>,"ax",@progbits
	.p2align	4
.type	alloy_primitives::utils::keccak_cache::compute::<alloy_primitives::utils::keccak256_impl>,@function
alloy_primitives::utils::keccak_cache::compute::<alloy_primitives::utils::keccak256_impl>:
	.cfi_startproc
	push rbp
	.cfi_def_cfa_offset 16
	push r15
	.cfi_def_cfa_offset 24
	push r14
	.cfi_def_cfa_offset 32
	push r13
	.cfi_def_cfa_offset 40
	push r12
	.cfi_def_cfa_offset 48
	push rbx
	.cfi_def_cfa_offset 56
	sub rsp, 168
	.cfi_def_cfa_offset 224
	.cfi_offset rbx, -56
	.cfi_offset r12, -48
	.cfi_offset r13, -40
	.cfi_offset r14, -32
	.cfi_offset r15, -24
	.cfi_offset rbp, -16
	lea rax, [rdx - 88]
	mov r14, rdx
	mov r15, rsi
	mov rbx, rdi
	cmp rax, -87
	jb .LBB1_28
	movabs r8, 6692946090922938792
	cmp r14, 17
	jb .LBB1_12
	movabs r11, -8223903398948509690
	movabs rax, 4053791697709505004
	cmp r14, 81
	jae .LBB1_10
	mov r10, r15
	mov r9, r14
.LBB1_5:
	mov rcx, qword ptr [r10]
	xor rax, qword ptr [r10 + 8]
	xor rcx, r11
	mov rdx, rax
	mulx rdx, rax, rcx
	xor rdx, rax
	cmp r9, 33
	jb .LBB1_9
	xor r11, qword ptr [r10 + 16]
	xor rdx, qword ptr [r10 + 24]
	mulx rdx, rax, r11
	xor rdx, rax
	cmp r9, 49
	jb .LBB1_9
	mov rax, qword ptr [r10 + 32]
	xor rdx, qword ptr [r10 + 40]
	xor rax, r8
	mulx rdx, rax, rax
	xor rdx, rax
	cmp r9, 65
	jb .LBB1_9
	mov rax, qword ptr [r10 + 48]
	xor rdx, qword ptr [r10 + 56]
	xor rax, r8
	mulx rdx, rax, rax
	xor rdx, rax
.LBB1_9:
	xor r9, qword ptr [r15 + r14 - 16]
.LBB1_17:
	mov rax, qword ptr [r15 + r14 - 8]
.LBB1_18:
	xor rax, rdx
	xor r9, r8
	mov rdx, rax
	mulx rcx, rax, r9
	xor rcx, rax
	mov rax, qword ptr [rip + alloy_primitives::utils::keccak_cache::CACHE@GOTPCREL]
	mov r12, qword ptr [rax + 8]
	mov rdx, qword ptr [rax]
	lea rax, [r12 - 1]
	neg r12
	and rax, rcx
	and r12, rcx
	shl rax, 7
	lea rbp, [rdx + rax]
	mov rax, qword ptr [rdx + rax]
	cmp rax, r12
	jne .LBB1_23
	mov rcx, r12
	or rcx, 1
	mov rax, r12
	lock cmpxchg	qword ptr [rbp], rcx
	jne .LBB1_23
	movzx eax, byte ptr [rbp + 8]
	cmp r14, rax
	jne .LBB1_22
	lea rsi, [rbp + 9]
	mov rdi, r15
	mov rdx, r14
	call qword ptr [rip + bcmp@GOTPCREL]
	test eax, eax
	je .LBB1_31
.LBB1_22:
	mov qword ptr [rbp], r12
.LBB1_23:
	mov r13, rsp
	mov rdi, r13
	mov rsi, r15
	mov rdx, r14
	call qword ptr [rip + alloy_primitives::utils::keccak256_impl@GOTPCREL]
	mov rax, qword ptr [rbp]
	test al, 1
	jne .LBB1_27
	mov rcx, rax
	or rcx, 1
	lock cmpxchg	qword ptr [rbp], rcx
	jne .LBB1_27
	lea rdi, [rsp + 32]
	mov rsi, r15
	mov rdx, r14
	call qword ptr [rip + memcpy@GOTPCREL]
	mov byte ptr [rbp + 8], r14b
	mov r13, rsp
	vmovups zmm0, zmmword ptr [rsp + 32]
	vmovups zmm1, zmmword ptr [rsp + 55]
	vmovups zmmword ptr [rbp + 32], zmm1
	vmovups zmmword ptr [rbp + 9], zmm0
	vmovups ymm0, ymmword ptr [rsp]
	vmovups ymmword ptr [rbp + 96], ymm0
.LBB1_26:
	mov qword ptr [rbp], r12
.LBB1_27:
	vmovups ymm0, ymmword ptr [r13]
	vmovups ymmword ptr [rsp + 128], ymm0
.LBB1_30:
	vmovups ymmword ptr [rbx], ymm0
	add rsp, 168
	.cfi_def_cfa_offset 56
	pop rbx
	.cfi_def_cfa_offset 48
	pop r12
	.cfi_def_cfa_offset 40
	pop r13
	.cfi_def_cfa_offset 32
	pop r14
	.cfi_def_cfa_offset 24
	pop r15
	.cfi_def_cfa_offset 16
	pop rbp
	.cfi_def_cfa_offset 8
	vzeroupper
	ret
.LBB1_31:
	.cfi_def_cfa_offset 224
	vmovups ymm0, ymmword ptr [rbp + 96]
	lea r13, [rsp + 32]
	vmovups ymmword ptr [rsp + 32], ymm0
	jmp .LBB1_26
.LBB1_28:
	test r14, r14
	je .LBB1_29
	mov rdi, rbx
	mov rsi, r15
	mov rdx, r14
	add rsp, 168
	.cfi_def_cfa_offset 56
	pop rbx
	.cfi_def_cfa_offset 48
	pop r12
	.cfi_def_cfa_offset 40
	pop r13
	.cfi_def_cfa_offset 32
	pop r14
	.cfi_def_cfa_offset 24
	pop r15
	.cfi_def_cfa_offset 16
	pop rbp
	.cfi_def_cfa_offset 8
	jmp qword ptr [rip + alloy_primitives::utils::keccak256_impl@GOTPCREL]
.LBB1_12:
	.cfi_def_cfa_offset 224
	movabs rdx, 4053791697709505004
	cmp r14, 3
	jbe .LBB1_13
	xor rdx, r14
	cmp r14, 7
	jbe .LBB1_15
	mov r9, qword ptr [r15]
	jmp .LBB1_17
.LBB1_10:
	mov rcx, rax
	mov r12, rax
	mov rdi, rax
	mov rsi, rax
	mov r10, r15
	mov r9, r14
.LBB1_11:
	mov r13, qword ptr [r10]
	xor rax, qword ptr [r10 + 8]
	movabs rdx, -6065962241257438131
	xor rcx, qword ptr [r10 + 24]
	xor r12, qword ptr [r10 + 40]
	mov rbp, qword ptr [r10 + 48]
	xor rdi, qword ptr [r10 + 56]
	xor rsi, qword ptr [r10 + 72]
	add r9, -80
	xor r13, rdx
	mov rdx, rax
	mulx rax, rdx, r13
	mov r13, qword ptr [r10 + 16]
	xor rax, rdx
	mov rdx, rcx
	xor r13, r8
	mulx rcx, rdx, r13
	mov r13, qword ptr [r10 + 32]
	xor rcx, rdx
	mov rdx, r12
	xor r13, r11
	mulx r13, r12, r13
	movabs rdx, -7978130974970042500
	xor rbp, rdx
	mov rdx, rdi
	mulx rdi, rdx, rbp
	mov rbp, qword ptr [r10 + 64]
	add r10, 80
	xor r13, r12
	mov r12, r13
	xor rdi, rdx
	movabs rdx, 1607827110969640605
	xor rbp, rdx
	mov rdx, rsi
	mulx rsi, rdx, rbp
	xor rsi, rdx
	cmp r9, 80
	ja .LBB1_11
	mov rdx, r12
	xor rcx, rax
	xor rdx, rdi
	xor rdx, rcx
	xor rdx, rsi
	mov rax, rdx
	cmp r9, 17
	jae .LBB1_5
	jmp .LBB1_9
.LBB1_13:
	movzx ecx, byte ptr [r15]
	mov rax, r14
	shr rax
	movzx r9d, byte ptr [r15 + r14 - 1]
	movzx eax, byte ptr [r15 + rax]
	shl rcx, 45
	or r9, rcx
	jmp .LBB1_18
.LBB1_29:
	vmovups ymm0, ymmword ptr [rip + .Lanon.616f245fec68e92a93a4dc78482f842c.5]
	jmp .LBB1_30
.LBB1_15:
	mov r9d, dword ptr [r15]
	mov eax, dword ptr [r15 + r14 - 4]
	jmp .LBB1_18
