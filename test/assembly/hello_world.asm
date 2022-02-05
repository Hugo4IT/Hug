; nasm -felf64 test/assembly/hello_world.asm -o bin/test.o && ld -o bin/test bin/test.o && bin/test

; C equivalent of `for (int i = 0; i < 30; i++) { for (int j = 0; j < i; j++) printf("*"); printf("\n"); }`

            global      _start                  ; linux entry point
            segment     .text                   ; code segment
_start:     
            mov         rax, SC_PRINT           ; syscall for print
            mov         rdi, STDOUT             ; 1 = stdout
            mov         rsi, msg                ; msg = pointer to first byte of attached string (see EQU explanation at bottom)
            mov         rdx, msglen             ; Constant expression
            syscall                             ; perform syscall print(stdout, msg, msglen)

start:
            mov         rdx, dataPtr            ; Currently pointing as start of data
            mov         r8, 1                   ; Line length starts at 1
            mov         r9, 0                   ; 0 Starts have been written so far

line:
            mov         byte [rdx], '*'         ; Interpret rdx as a [memory address] (pointer) and write * there
            inc         rdx                     ; Increment pointer
            inc         r9                      ; One more star has been drawn
            cmp         r9, r8                  ; r9 ? r8
            jne         line                    ; if (r9 != r8) goto line;

lineEnd:
            mov         byte [rdx], 10          ; Write newline
            inc         rdx                     ; Increment pointer again
            inc         r8                      ; Next line, 1 more star
            mov         r9, 0                   ; The next line has 0 stars
            cmp         r8, lines               ; r8 ? lines
            jng         line                    ; if (r8 < lines) goto line;

finalPrint:
            mov         rax, SC_PRINT           ; Syscall for print()
            mov         rdi, STDOUT             ; print(stdout, ..
            mov         rsi, dataPtr            ; print(stdout, dataPtr, ..
            mov         rdx, dataSize           ; print(stdout, dataPtr, dataSize)
            syscall                             ; print(stdout, dataPtr, dataSize);

exit:
            mov         rax, SC_EXIT            ; syscall for exit()
            mov         rdi, SUCCESS            ; rdi = 0
            syscall                             ; perform syscall exit(0)

            segment     .data                   ; initialized (= already assigned a value) constant data segment
msg:        db          "Triangle program:", 10
msglen      equ         $-msg
lines       equ         30
dataSize    equ         (lines*lines)/2+lines*2

SC_PRINT    equ         1                       ; print() syscall
SC_EXIT     equ         60                      ; exit()  syscall
STDOUT      equ         1                       ; print to stdout
SUCCESS     equ         0                       ; C: EXIT_SUCCESS
FAILURE     equ         1                       ; C: EXIT_FAILURE

            segment     .bss                    ; Uninitialized, reserved data
dataPtr:    resb        dataSize

; What i've picked up (not necessarily true :/):

;           db          "Hello, World!", 10
;           
;           Is equivalent to:
;
;           db          'H','e','l','l','o',',',' ','W','o','r','l','d','!',10
;           dw          'He','ll','o,',' W','or','ld','!',10
;           dd          'Hell','o, W','orld','!',10,0,0
;           dq          'Hello, W','orld!',10,0,0

; d* = data-*
; Example: db = data-byte, dq = data-quadruple-word

; ===========================================================
; Name      d*          Size            Long name
; ===========================================================
; BYTE      db          1  (8)          Byte
; WORD      dw          2  (16)         Word
; DWORD     dd          4  (32)         Double WORD
; QWORD     dq          8  (64)         Quadruple WORD
; TWORD     dt          16 (128)        exTended WORD
; ===========================================================

; $ - Current position in assembly (in bytes), see EQU for more info.
;
; EQU - A constant expression
;
; For example:
;   msg:    db          "Hello, World!", 10
;   msglen: equ         $-msg
;
; `msglen` is now defined to the current position, minus the position of `msg`. Say msg was at position 150, after some instructions:
;
;   Stage 1:
;
;   [0x00000096]    msg:    db          "Hello, World!", 10
;   [0x000000A4]    msglen: equ         $-msg
;
;   Stage 2:
;
;   [0x00000096]            db          'H'                     ; <- msg (0x00000096)
;   [0x00000097]            db          'e'
;   [0x00000098]            db          'l'
;   [0x00000099]            db          'l'
;   [0x0000009A]            db          'o'
;   [0x0000009B]            db          ','
;   [0x0000009C]            db          ' '
;   [0x0000009D]            db          'W'
;   [0x0000009E]            db          'o'
;   [0x0000009F]            db          'r'
;   [0x000000A0]            db          'l'
;   [0x000000A1]            db          'd'
;   [0x000000A2]            db          '!'
;   [0x000000A3]            db          10                      ; ASCII 10 = \n
;   [0x000000A4]    msglen: equ         $-msg                   ; <- $ (0x000000A4)
;
;   Stage 3:
;
;   msg is now constant pointer to address 0x00000096 (Start of the string), so occurrences of `msg` will be replaced with [0x00000096]
;
;   [0x00000096]            db          'H'                     ; <- msg (0x00000096)
;   [0x00000097]            db          'e'
;   [0x00000098]            db          'l'
;   [0x00000099]            db          'l'
;   [0x0000009A]            db          'o'
;   [0x0000009B]            db          ','
;   [0x0000009C]            db          ' '
;   [0x0000009D]            db          'W'
;   [0x0000009E]            db          'o'
;   [0x0000009F]            db          'r'
;   [0x000000A0]            db          'l'
;   [0x000000A1]            db          'd'
;   [0x000000A2]            db          '!'
;   [0x000000A3]            db          10                      ; ASCII 10 = \n
;   [0x000000A4]    msglen: equ         0x000000A4-0x00000096   ; <- $ (0x000000A4)
;
;   Final:
;
;   [0x00000096]            db          'H'
;   [0x00000097]            db          'e'
;   [0x00000098]            db          'l'
;   [0x00000099]            db          'l'
;   [0x0000009A]            db          'o'
;   [0x0000009B]            db          ','
;   [0x0000009C]            db          ' '
;   [0x0000009D]            db          'W'
;   [0x0000009E]            db          'o'
;   [0x0000009F]            db          'r'
;   [0x000000A0]            db          'l'
;   [0x000000A1]            db          'd'
;   [0x000000A2]            db          '!'
;   [0x000000A3]            db          10                      ; ASCII \n
;   [0x000000A4]    msglen: equ         14                      ; 164 - 150 = 14

; Calling (system) functions:
;
; On 64-bit linux and x86_64 MacOS, functions are called like this:
;   syscall -> rax(rdi, rsi, rdx, rcx, r8, r9);
;
; So to choose a function, just set the `rax` register, for parameters set the `rdi` - `r9` registers