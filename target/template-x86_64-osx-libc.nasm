; Compile, link and run: nasm -fmacho64 ./target/template-x86_64-osx-libc.nasm -o ./bin/template-x86_64-osx-libc.o && gcc ./bin/template-x86_64-osx-libc.o -o ./bin/out && ./bin/out

    global          _main
    ;;;___EXTERN_HERE___;;;

    section         .text
_main:
    push            rbx                     ; Align call stack
    ;;;___CODE_HERE___;;;
    pop             rbx                     ; Fix stack before returning
    ret

    section         .data
    ;;;___DATA_HERE___;;;
