; Compile, link and run: nasm -fmacho64 ./target/template-x86_64-osx-standalone.nasm -o ./bin/template-x86_64-osx-standalone.o && ld ./bin/template-x86_64-osx-standalone.o -macosx_version_min 10.7.0 -o ./bin/a.out && ./bin/a.out

    global          start
    ;;;___EXTERN_HERE___;;;

    section         .text
start:
    ;;;___CODE_HERE___;;;
    mov rax,0x02000001      ; Syscall for exit
    mov rdi,0               ; EXIT_SUCCESS
    syscall                 ; perform syscall

    section         .data
    ;;;___DATA_HERE___;;;
