#!/bin/sh

TARGET=$1
FILE=$2
LINKER=$3

SetColor() {
    if [ $1 = 'dark' ]
    then
        if [ $2 ]
        then
            printf "\e[01;30m"
        else
            printf "\e[00;30m"
        fi
    elif [ $1 = 'red' ]
    then
        if [ $2 ]
        then
            printf "\e[01;31m"
        else
            printf "\e[00;31m"
        fi
    elif [ $1 = 'green' ]
    then
        if [ $2 ]
        then
            printf "\e[01;32m"
        else
            printf "\e[00;32m"
        fi
    elif [ $1 = 'yellow' ]
    then
        if [ $2 ]
        then
            printf "\e[01;33m"
        else
            printf "\e[00;33m"
        fi
    elif [ $1 = 'blue' ]
    then
        if [ $2 ]
        then
            printf "\e[01;34m"
        else
            printf "\e[00;34m"
        fi
    elif [ $1 = 'magenta' ]
    then
        if [ $2 ]
        then
            printf "\e[01;34m"
        else
            printf "\e[00;34m"
        fi
    else
        if [ $2 ]
        then
            printf "\e[01;00m"
        else
            printf "\e[00;00m"
        fi
    fi
}

ResetColor() {
    SetColor reset
}

Prefix() {
    printf "$(SetColor green bold)[run.sh | $(SetColor $2 bold)$1$(SetColor green bold)]:"
    ResetColor
}

ExitWithError() {
    echo "$(Prefix Error red) $(SetColor red)$1"
    exit 1
}

PrintMessage() {
    echo "$(Prefix Message blue) $1"
}

Task() {
    echo "$(Prefix Task reset) $1"
    $1 || ExitWithError "Task failed, code: $?"
}

if [ $LINKER ]
then
    LINKER="gcc -static"
else
    LINKER=ld
fi

PrintMessage "Using linker $LINKER"

if [ $TARGET = 'linux64' ]
then
    Task "nasm -felf64 test/assembly/$FILE.$TARGET.asm -o bin/$FILE.o"
    Task "$LINKER -o bin/$FILE bin/$FILE.o"
elif [ $TARGET = 'apple64' ]
then
    PrintMessage "apple64 platform support coming soon..."
else
    PrintMessage "Unsupported platform, currently only linux64 is supported, with apple64 on the way"
fi

Task "bin/$FILE"

exit 0