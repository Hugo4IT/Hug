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