; multi-segment executable file template.

data segment
    ; add static/global variables here 
    
      
    ;heap memory
    free_list dw 0
    heap_start label word
ends

stack segment
    dw   2048  dup(0)
ends

code segment

;standard library functions --------

;MEMORY ----------------------------

Memory_init PROC    
    
    mov ax, offset heap_start
    mov [free_list], ax

    mov cx, 65535
    sub cx, ax

    mov bx, [free_list]
    mov [bx + 0], cx
    mov word ptr [bx + 2], 0

    ret
Memory_init ENDP   

Memory_alloc PROC
    push bp
    mov bp, sp

    mov ax, [bp + 4]
    shl ax, 1
    add ax, 4

    mov si, [free_list]
    mov bx, 0

search_loop:
    cmp si, 0
    je err_mem_alloc

    mov cx, [si]
    cmp cx, ax
    jge allocate_block

    mov bx, si
    mov si, [si + 2] 
    jmp search_loop

allocate_block:
   
    sub cx, ax  
    mov [si], cx           

    add si, cx            

    mov [si], ax      
    mov word ptr [si + 2], 0

    add si, 4
    mov ax, si     

    mov sp, bp
    pop bp
    ret

err_mem_alloc:
    int 3
    hlt
Memory_alloc ENDP         

Memory_deAlloc PROC
    push bp
    mov bp, sp
    
    mov si, [bp + 4]

    sub si, 4

    mov ax, [free_list]

    mov [si + 2], ax

    mov [free_list], si

    mov sp, bp
    pop bp
    ret
Memory_deAlloc ENDP

;VECTOR ---------------------------- 

Vector_new proc near
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
Vector_new endp

Vector_get proc near
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
Vector_get endp

Vector_resize proc near
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
Vector_resize endp

Vector_destroy proc near
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
Vector_destroy endp

; STRING ----------------------------------

String_new proc near
    push bp
    mov bp, sp

    mov ax, 4
    push ax
    call Memory_alloc
    add sp, 2

    mov bx, ax
    mov word ptr [bx + 2], 0

    push bx
    push word ptr [bp + 4]
    call Vector_new
    add sp, 2
    pop bx

    mov word ptr [bx + 0], ax

    mov ax, bx
    pop bp
    ret
String_new endp

String_append proc near
    push bp
    mov bp, sp

    mov bx, word ptr [bp + 4]
    mov cx, word ptr [bx + 2]
    mov dx, word ptr [bp + 6]

    push bx
    push cx
    push dx
    push word ptr [bx + 0]
    call Vector_add
    add sp, 6
    pop bx

    inc word ptr [bx + 2]

    pop bp
    ret
String_append endp

String_get proc near
    push bp
    mov bp, sp

    mov bx, word ptr [bp + 4]

    push word ptr [bp + 6]
    push word ptr [bx + 0]
    call Vector_get
    add sp, 4

    pop bp
    ret
String_get endp

String_put proc near
    push bp
    mov bp, sp

    mov bx, word ptr [bp + 4]
    mov cx, word ptr [bp + 6]
    mov dx, word ptr [bp + 8]

    cmp dx, word ptr [bx + 2]
    jge put_done

    push word ptr [bp + 8]
    push word ptr [bp + 6]
    push word ptr [bx + 0]
    call Vector_add
    add sp, 6

put_done:
    pop bp
    ret
String_put endp

String_length proc near
    push bp
    mov bp, sp

    mov bx, word ptr [bp + 4]
    mov ax, word ptr [bx + 2]

    pop bp
    ret
String_length endp

String_destroy proc near
    push bp
    mov bp, sp

    mov bx, word ptr [bp + 4]

    push bx
    push word ptr [bx + 0]
    call Vector_destroy
    add sp, 2
    pop bx

    push bx
    call Memory_dealloc
    add sp, 2

    pop bp
    ret
String_destroy endp

; END STD LIBRARY -------------------------

;------------------------------------------
; CODE GOES HERE !!!!!
;------------------------------------------

    
;entry point -----------------------
start:
    mov ax, data
    mov ds, ax
    mov es, ax       
    
    call Memory_init
    
    call Main_main
    
    mov ax, 4c00h ; exit to operating system.
    int 21h    
ends

end start
