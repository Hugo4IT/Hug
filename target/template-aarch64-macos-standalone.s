// Compile, link and run: arch -arm64 as ./target/template-aarch64-macos-standalone.s -o ./bin/template-aarch64-macos-standalone.o && ld ./bin/template-aarch64-macos-standalone.o -o ./bin/out -lSystem -syslibroot `xcrun -sdk macosx --show-sdk-path` -e _start -arch arm64  && ./bin/out

.global _start
.align 4

///___EXTERN_HERE___///

_start:
    ///___CODE_HERE___///
    mov X0,#0   // EXIT_SUCCESS
    mov X16,#1  // syscall exit
    svc #0x80   // syscall()

///___DATA_HERE___///