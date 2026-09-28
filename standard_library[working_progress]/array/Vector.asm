vector_new proc near
    push bp
    mov bp, sp
    
    mov ax, 4
    push ax
    call Memory_alloc
    add sp, 2
    
    mov bx, ax

    mov cx, word ptr [bp + 4]
    mov word ptr [bx + 0], cx

    push bx
    push cx
    call Memory_alloc
    add sp, 2
    pop bx

    mov word ptr [bx + 2], ax

    mov ax, bx
    pop bp
    ret
vector_new endp

vector_get proc near
    push bp
    mov bp, sp
    push si

    mov bx, word ptr [bp + 4]
    mov si, word ptr [bp + 6]

    cmp si, 0
    jl get_out_of_bounds
    cmp si, word ptr [bx + 0]
    jge get_out_of_bounds

    mov bx, word ptr [bx + 2]
    
    shl si, 1
    mov ax, word ptr [bx + si]

    pop si
    pop bp
    ret

get_out_of_bounds:
    xor ax, ax
    pop si
    pop bp
    ret
vector_get endp

vector_resize proc near
    push bp
    mov bp, sp
    push si
    push di

    mov cx, word ptr [bp + 6]
    cmp cx, 0
    jle resize_done

    mov bx, word ptr [bp + 4]

    mov ax, word ptr [bx + 0]
    cmp cx, ax
    jge resize_expanding

    dec cx
    mov si, cx
    jmp resize_alloc

resize_expanding:
    dec ax
    mov si, ax

resize_alloc:
    mov dx, word ptr [bp + 6]
    push bx
    push si
    push dx
    call Memory_alloc
    add sp, 2
    pop si
    pop bx

    mov di, word ptr [bx + 2]

resize_copyloop:
    cmp si, 0
    jl resize_copydone

    mov dx, si
    shl dx, 1

    push bx
    mov bx, di
    add bx, dx
    mov cx, word ptr [bx]

    mov bx, ax
    add bx, dx
    mov word ptr [bx], cx
    pop bx

    dec si
    jmp resize_copyloop

resize_copydone:
    push ax
    push bx
    push word ptr [bx + 2]
    call Memory_deAlloc
    add sp, 2
    pop bx
    pop ax

    mov word ptr [bx + 2], ax
    mov dx, word ptr [bp + 6]
    mov word ptr [bx + 0], dx

resize_done:
    pop di
    pop si
    pop bp
    ret
vector_resize endp

vector_destroy proc near
    push bp
    mov bp, sp

    mov bx, word ptr [bp + 4]

    push bx
    push word ptr [bx + 2]
    call Memory_deAlloc
    add sp, 2
    pop bx

    push bx
    call Memory_deAlloc
    add sp, 2

    pop bp
    ret
vector_destroy endp