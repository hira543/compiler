extern printf
section .bss
buffer_0 resb 12
section .data
i_res dq 0
sum_res dq 0
section .text
global _start, int_to_ascii
int_to_ascii:
    push rbx
    mov rbx, rdi
    lea rsi, [rel buffer_0 + 11]
    mov byte [rsi], 0
    sub rsi, 1
    mov rcx, 10
convert_loop:
    xor rdx, rdx
    div rcx
    add dl, '0'
    mov [rsi], dl
    test rax, rax
    jz convert_end
    sub rsi, 1
    mov rbx, rax
    jmp convert_loop
convert_end:
    pop rbx
    ret
_start:
start_0:
    mov rbx, [i_res]
    mov rcx, 100
    cmp rbx, rcx
    setl al
    movzx eax, al
    jge end_1
    mov rbx, [sum_res]
    mov rcx, 1
    add rbx, rcx
    mov rax, rbx
    mov [sum_res], eax
    mov rbx, [i_res]
    mov rcx, 1
    add rbx, rcx
    mov rax, rbx
    mov [i_res], eax
    jmp start_0
end_1:
    mov rax, [sum_res]
    mov rax, rax
    lea rsi, [rel buffer_0]
    call int_to_ascii
    lea rsi, [rel buffer_0]
    mov edi, 1
    mov eax, 1
    mov edx, 12
    syscall
mov rax, 60
xor rdi, rdi
syscall
