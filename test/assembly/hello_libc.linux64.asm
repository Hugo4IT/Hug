            global      main
            extern      puts
            section     .text

main:                                       ; C entry point
            mov         rdi, msg            ; Standard string pointer
            call        puts                ; puts(msg);
            mov         rax, 0              ; Return value
            ret                             ; return EXIT_SUCCESS;

            section     .data
msg:        db          "Hello, libc!", 0   ; Terminate with \0 instead of \n for libc