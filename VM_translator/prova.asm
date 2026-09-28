ArrayUtils_new:
push bp
mov bp, sp
sub sp, 0

mov ax, 2
push ax

call Memory_alloc
add sp, 2
push ax

pop ax
mov si, ax

mov ax, [bp + 4]
push ax

pop ax
mov [si + 0], ax

mov ax, [bp + 4]
push ax

call Array_new
add sp, 2
push ax

pop ax
mov [si + 2], ax

push si

pop ax
mov sp, bp
pop bp
ret

ArrayUtils_fillSequential:
push bp
mov bp, sp
sub sp, 2

mov ax, [bp + 4]
push ax

pop ax
mov si, ax

mov ax, 0
push ax

pop ax
mov [bp - 2], ax

l_ArrayUtils_fillSequential_WHILE_EXP_55:

mov ax, [bp - 2]
push ax

mov ax, [si + 0]
push ax

pop bx
    pop ax
    cmp ax, bx
    jl TRUE_0
    push 0
    jmp END_0
TRUE_0:
    push -1
END_0:

pop ax
not ax
push ax
pop ax
test ax,ax
jne l_ArrayUtils_fillSequential_WHILE_END_55

mov ax, [bp - 2]
push ax

mov ax, [si + 2]
push ax

pop bx
pop ax
add ax, bx
push ax

mov ax, [bp + 6]
push ax

mov ax, [bp - 2]
push ax

pop bx
pop ax
add ax, bx
push ax

pop ax
mov [temp_segment + 0], ax

pop ax
mov di, ax

mov ax, [temp_segment + 0]
push ax

pop ax
mov [di + 0], ax

mov ax, [bp - 2]
push ax

mov ax, 1
push ax

pop bx
pop ax
add ax, bx
push ax

pop ax
mov [bp - 2], ax

jmp l_ArrayUtils_fillSequential_WHILE_EXP_55

l_ArrayUtils_fillSequential_WHILE_END_55:

mov ax, 0
push ax

pop ax
mov sp, bp
pop bp
ret

ArrayUtils_sum:
push bp
mov bp, sp
sub sp, 4

mov ax, [bp + 4]
push ax

pop ax
mov si, ax

mov ax, 0
push ax

pop ax
mov [bp - 2], ax

mov ax, 0
push ax

pop ax
mov [bp - 4], ax

l_ArrayUtils_sum_WHILE_EXP_105:

mov ax, [bp - 2]
push ax

mov ax, [si + 0]
push ax

pop bx
    pop ax
    cmp ax, bx
    jl TRUE_1
    push 0
    jmp END_1
TRUE_1:
    push -1
END_1:

pop ax
not ax
push ax
pop ax
test ax,ax
jne l_ArrayUtils_sum_WHILE_END_105

mov ax, [bp - 4]
push ax

mov ax, [bp - 2]
push ax

mov ax, [si + 2]
push ax

pop bx
pop ax
add ax, bx
push ax

pop ax
mov di, ax

mov ax, [di + 0]
push ax

pop bx
pop ax
add ax, bx
push ax

pop ax
mov [bp - 4], ax

mov ax, [bp - 2]
push ax

mov ax, 1
push ax

pop bx
pop ax
add ax, bx
push ax

pop ax
mov [bp - 2], ax

jmp l_ArrayUtils_sum_WHILE_EXP_105

l_ArrayUtils_sum_WHILE_END_105:

mov ax, [bp - 4]
push ax

pop ax
mov sp, bp
pop bp
ret

ArrayUtils_print:
push bp
mov bp, sp
sub sp, 2

mov ax, [bp + 4]
push ax

pop ax
mov si, ax

mov ax, 0
push ax

pop ax
mov [bp - 2], ax

l_ArrayUtils_print_WHILE_EXP_149:

mov ax, [bp - 2]
push ax

mov ax, [si + 0]
push ax

pop bx
    pop ax
    cmp ax, bx
    jl TRUE_2
    push 0
    jmp END_2
TRUE_2:
    push -1
END_2:

pop ax
not ax
push ax
pop ax
test ax,ax
jne l_ArrayUtils_print_WHILE_END_149

mov ax, [bp - 2]
push ax

mov ax, [si + 2]
push ax

pop bx
pop ax
add ax, bx
push ax

pop ax
mov di, ax

mov ax, [di + 0]
push ax

call Output_printInt
add sp, 2
push ax

pop ax
mov [temp_segment + 0], ax

mov ax, 17
push ax

call String_new
add sp, 2
push ax

mov ax, 60
push ax

call String_appendChar
add sp, 4
push ax

mov ax, 47
push ax

call String_appendChar
add sp, 4
push ax

mov ax, 115
push ax

call String_appendChar
add sp, 4
push ax

mov ax, 116
push ax

call String_appendChar
add sp, 4
push ax

mov ax, 114
push ax

call String_appendChar
add sp, 4
push ax

mov ax, 105
push ax

call String_appendChar
add sp, 4
push ax

mov ax, 110
push ax

call String_appendChar
add sp, 4
push ax

mov ax, 103
push ax

call String_appendChar
add sp, 4
push ax

mov ax, 67
push ax

call String_appendChar
add sp, 4
push ax

mov ax, 111
push ax

call String_appendChar
add sp, 4
push ax

mov ax, 110
push ax

call String_appendChar
add sp, 4
push ax

mov ax, 115
push ax

call String_appendChar
add sp, 4
push ax

mov ax, 116
push ax

call String_appendChar
add sp, 4
push ax

mov ax, 97
push ax

call String_appendChar
add sp, 4
push ax

mov ax, 110
push ax

call String_appendChar
add sp, 4
push ax

mov ax, 116
push ax

call String_appendChar
add sp, 4
push ax

mov ax, 62
push ax

call String_appendChar
add sp, 4
push ax

call Output_printString
add sp, 2
push ax

pop ax
mov [temp_segment + 0], ax

mov ax, [bp - 2]
push ax

mov ax, 1
push ax

pop bx
pop ax
add ax, bx
push ax

pop ax
mov [bp - 2], ax

jmp l_ArrayUtils_print_WHILE_EXP_149

l_ArrayUtils_print_WHILE_END_149:

call Output_println
add sp, 0
push ax

pop ax
mov [temp_segment + 0], ax

mov ax, 0
push ax

pop ax
mov sp, bp
pop bp
ret

ArrayUtils_dispose:
push bp
mov bp, sp
sub sp, 0

mov ax, [bp + 4]
push ax

pop ax
mov si, ax

mov ax, [si + 2]
push ax

call Array_dispose
add sp, 2
push ax

pop ax
mov [temp_segment + 0], ax

push si

call Memory_deAlloc
add sp, 2
push ax

pop ax
mov [temp_segment + 0], ax

mov ax, 0
push ax

pop ax
mov sp, bp
pop bp
ret

