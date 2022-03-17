"""
File: compiler.py
Version: 0.1.0
Author: Hugo4IT
License: MIT
Repository: https://github.com/Hugo4IT/Hug

Description: A Hug compiler implemented in python to compile the official Hug compiler written in Hug.
"""

import logging
import colorama
from .lexer import Lexer
from .syntax import ParseTree
from .symbol import SymbolTable

def compile(options):
    logging.info("Input file received: %s", colorama.Style.BRIGHT + colorama.Fore.LIGHTYELLOW_EX + options.inputfile)
    logging.info("Entering phase 1 - Lexical Analysis")

    lexer = Lexer(options, options.inputfile)
    lexems = lexer.start()

    if options.highlight:
        print(lexer.highlight())
    
    lexer.trim()
    if options.errorcounter.count > 0 and not options.yestoall:
        while True:
            answer = input("%d error(s) occured during phase 1 of compiling, continue? (y/N)" % options.errorcounter.count).lower()
            if answer == "no" or answer == "n" or answer == "":
                quit()
                break
            elif answer == "yes" or answer == "y":
                break
            else:
                print("Unrecognized input, please choose y or n")
    options.errorcounter.resetcount()

    logging.info("Entering phase 2 - Syntax Analysis")
    parsetree = ParseTree(lexer)

    logging.info("SymbolTable:");
    for line in str(SymbolTable.instance()).split("\n"):
        logging.info(line)