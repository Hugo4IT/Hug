; nasm -felf64 test/assembly/hello_world.asm -o bin/test.o && ld -o bin/test bin/test.o && bin/test

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