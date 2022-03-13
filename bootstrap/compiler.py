"""
File: compiler.py
Version: 0.1.0
Author: Hugo4IT
License: MIT
Repository: https://github.com/Hugo4IT/Hug

Description: A Hug compiler implemented in python to compile the official Hug compiler written in Hug.
"""

import os
import logging
import colorama

KEYWORDS = [
    "let",
    "if",
]

HIGHLIGHT_COLORS = [
    colorama.Style.DIM,                                                 # LComment
    colorama.Style.NORMAL + colorama.Back.RED + colorama.Fore.WHITE,    # LUnknown
    colorama.Style.NORMAL,                                              # LWhitespace
    colorama.Fore.LIGHTMAGENTA_EX + colorama.Style.BRIGHT,              # LKeyword
    colorama.Fore.LIGHTCYAN_EX + colorama.Style.NORMAL,                 # LIdent
    colorama.Fore.LIGHTYELLOW_EX + colorama.Style.NORMAL,               # LLiteral
    colorama.Style.NORMAL,                                              # LOperator
    colorama.Style.NORMAL,                                              # LMisc                                            # LOperator
]

class DebugInfo:
    def __init__(self):
        self.sline = 0
        self.scolumn = 0
        self.eline = 0
        self.ecolumn = 0
    
    def newline(self):
        self.eline += 1
        self.ecolumn = 0
    
    def newchar(self):
        self.ecolumn += 1
    
    def starttoken(self):
        self.sline = self.eline
        self.scolumn = self.ecolumn
    
    def clone(self) -> (int, int, int):
        return self.sline, self.scolumn, self.eline, self.ecolumn

class Lexem:
    def __init__(self, text: str, debuginfo: DebugInfo):
        self.text = text
        self.sline, self.scolumn, self.eline, self.ecolumn = debuginfo.clone()
    
    def __str__(self) -> str:
        buffer = self.__class__.__name__ + "("
        for name,value in self.__dict__.items():
            if not name in ["sline", "scolumn", "eline", "ecolumn"]:
                buffer += "\'" + name + "\': " + repr(str(value)) + ", "
        return buffer[0:len(buffer)-2] + ")"

class LComment(Lexem):
    TYPE_ID = 0

    LINE_COMMENT = 0
    BLOCK_COMMENT = 1

    def __init__(self, text: str, debuginfo: DebugInfo, commenttype: int):
        super().__init__(text, debuginfo)
        self.commenttype = commenttype

class LUnknown(Lexem):
    TYPE_ID = 1

    def __init__(self, text: str, debuginfo: DebugInfo):
        super().__init__(text, debuginfo)

class LWhitespace(Lexem):
    TYPE_ID = 2
    
    def __init__(self, text: str, debuginfo: DebugInfo):
        super().__init__(text, debuginfo)

class LKeyword(Lexem):
    TYPE_ID = 3
    
    def __init__(self, text: str, debuginfo: DebugInfo):
        super().__init__(text, debuginfo)

class LIdent(Lexem):
    TYPE_ID = 4
    
    def __init__(self, text: str, debuginfo: DebugInfo):
        super().__init__(text, debuginfo)

class LLiteral(Lexem):
    TYPE_ID = 5

    TYPE_INT8 = 0
    TYPE_INT16 = 1
    TYPE_INT32 = 2
    TYPE_INT64 = 3
    TYPE_INT128 = 4
    TYPE_INTARCH = 5

    TYPE_UINT8 = 6
    TYPE_UINT16 = 7
    TYPE_UINT32 = 8
    TYPE_UINT64 = 9
    TYPE_UINT128 = 10
    TYPE_UINTARCH = 11

    TYPE_FLOAT32 = 12
    TYPE_FLOAT64 = 13

    TYPE_CHAR = 14
    TYPE_STRING = 15

    def __init__(self, text: str, debuginfo: DebugInfo):
        super().__init__(text, debuginfo)

        self.value, self.kind = LLiteral.parse(text)
        if self.value == None:
            logging.error("Invalid literal: %s", text)
            quit()
    
    @staticmethod
    def parse(text: str):
        if text.startswith("\""):
            return None, LLiteral.TYPE_STRING # String
        elif text.startswith("\'"):
            return None, LLiteral.TYPE_CHAR # Char
        elif text == "true":
            return 1, LLiteral.TYPE_INT8
        elif text == "false":
            return 0, LLiteral.TYPE_INT8
        elif "." in text:
            return float(text), LLiteral.TYPE_FLOAT32
        elif text.isnumeric():
            return int(text), LLiteral.TYPE_INT32
        else:
            return None, LLiteral.TYPE_INT8

class LOperator(Lexem):
    TYPE_ID = 6

    # Basic math
    ADD = 0                         # +
    SUB = 1                         # -
    MUL = 2                         # *
    DIV = 3                         # /
    ASSIGN = 4                      # =
    ADDASSIGN = 5                   # +=
    SUBASSIGN = 6                   # -=
    MULASSIGN = 7                   # *=
    DIVASSIGN = 8                   # /=

    # Conditionals
    EQUALS = 9                      # ==
    LESSTHAN = 10                   # <
    GREATERTHAN = 11                # >
    LESSTHANOREQUALS = 12           # <=
    GREATERTHANOREQUALS = 13        # >=
    NOT = 14                        # !
    NOTEQUALS = 15                  # !=
    AND = 16                        # &&
    OR = 17                         # ||

    # Bit operations
    SHIFTLEFT = 18                  # <<
    SHIFTRIGHT = 19                 # >>
    SHIFTLEFTOVERFLOW = 20          # <<<
    SHIFTRIGHTOVERFLOW = 21         # >>>
    SHIFTLEFTASSIGN = 22            # <<=
    SHIFTRIGHTASSIGN = 23           # >>=
    SHIFTLEFTOVERFLOWASSIGN = 24    # <<<=
    SHIFTRIGHTOVERFLOWASSIGN = 25   # >>>=
    BITWISEAND = 26                 # &
    BITWISEANDASSIGN = 27           # &=
    BITWISEOR = 28                  # |
    BITWISEORASSIGN = 29            # |=
    BITWISEXOR = 30                 # ^
    BITWISEXORASSIGN = 31           # ^=
    BITWISENOT = 32                 # ~

    # Advanced math
    MODULO = 33                     # %
    MODULOASSIGN = 34               # %=
    INCREMENT = 35                  # ++
    DECREMENT = 36                  # --

    def __init__(self, text: str, debuginfo: DebugInfo, operator: int):
        super().__init__(text, debuginfo)
        self.operator = operator

class LMisc(Lexem):
    TYPE_ID = 7

    OPENBRACE = 0                   # {
    CLOSEBRACE = 1                  # }
    OPENBRACKET = 2                 # [
    CLOSEBRACKET = 3                # ]
    OPENPARENTHESIS = 4             # (
    CLOSEPARENTHESIS = 5            # )
    COLON = 6                       # :
    COMMA = 7                       # ,
    DOT = 8                         # .

    def __init__(self, text: str, debuginfo: DebugInfo, value: int):
        super().__init__(text, debuginfo)
        self.value = value

class Lexer:
    def __init__(self, options, filepath):
        self.options = options
        self.filepath = filepath
        self.file = requestfile(filepath)
        self.index = -1
        self.lexems = []

        self.debuginfo = DebugInfo()
    
    def next(self) -> str:
        self.index += 1
        if self.index < len(self.file):
            ch = self.file[self.index]
            if ch == "\n":
                self.debuginfo.newline()
            else:
                self.debuginfo.newchar()
            return ch
        else:
            return ""

    def peek(self, peek: int) -> str:
        if self.index + peek < len(self.file):
            return self.file[self.index + peek]
        else:
            return ""
    
    def next_lexem(self):
        self.debuginfo.starttoken()
        ch = self.next()
        if ch != "":
            textbuffer = ch
            if ch.isalpha():
                while self.peek(1).isalnum():
                    textbuffer += self.next()
                
                if textbuffer in ["true", "false"]:
                    return LLiteral(textbuffer, self.debuginfo)
                if textbuffer in KEYWORDS:
                    return LKeyword(textbuffer, self.debuginfo)
                else:
                    return LIdent(textbuffer, self.debuginfo)
            elif ch.isspace():
                while self.peek(1).isspace():
                    textbuffer += self.next()
                return LWhitespace(textbuffer, self.debuginfo)
            elif ch == "/":
                nch = self.peek(1)
                if nch == "/":
                    while self.peek(1) != "\n" and self.peek(1) != "":
                        textbuffer += self.next()
                    return LComment(textbuffer, self.debuginfo, LComment.LINE_COMMENT)
                elif nch == "*":
                    while self.peek(1) != "*" or self.peek(2) != "/":
                        textbuffer += self.next()
                    textbuffer += self.next() # *
                    textbuffer += self.next() # /
                    return LComment(textbuffer, self.debuginfo, LComment.BLOCK_COMMENT)
                elif nch == "=":
                    textbuffer += self.next() # =
                    return LOperator(textbuffer, self.debuginfo, LOperator.DIVASSIGN)
                else:
                    return LOperator(ch, self.debuginfo, LOperator.DIV)
            elif ch == "=":
                nch = self.peek(1)
                if nch == "=":
                    textbuffer += self.next()
                    return LOperator(textbuffer, self.debuginfo, LOperator.EQUALS)
                else:
                    return LOperator(textbuffer, self.debuginfo, LOperator.ASSIGN)
            elif ch == "+":
                if self.peek(1) == "=":
                    return LOperator(ch + self.next(), self.debuginfo, LOperator.ADDASSIGN)
                elif self.peek(1) == "+":
                    return LOperator(ch + self.next(), self.debuginfo, LOperator.INCREMENT)
                else:
                    return LOperator(ch, self.debuginfo, LOperator.ADD)
            elif ch == "-":
                if self.peek(1) == "=":
                    return LOperator(ch + self.next(), self.debuginfo, LOperator.SUBASSIGN)
                elif self.peek(1) == "-":
                    return LOperator(ch + self.next(), self.debuginfo, LOperator.DECREMENT)
                else:
                    return LOperator(ch, self.debuginfo, LOperator.SUB)
            elif ch == "*":
                if self.peek(1) == "=":
                    return LOperator(ch + self.next(), self.debuginfo, LOperator.MULASSIGN)
                else:
                    return LOperator(ch, self.debuginfo, LOperator.MUL)
            elif ch == "&":
                if self.peek(1) == "=":
                    return LOperator(ch + self.next(), self.debuginfo, LOperator.BITWISEANDASSIGN)
                elif self.peek(1) == "&":
                    return LOperator(ch + self.next(), self.debuginfo, LOperator.AND)
                else:
                    return LOperator(ch, self.debuginfo, LOperator.BITWISEAND)
            elif ch == "|":
                if self.peek(1) == "=":
                    return LOperator(ch + self.next(), self.debuginfo, LOperator.BITWISEORASSIGN)
                elif self.peek(1) == "|":
                    return LOperator(ch + self.next(), self.debuginfo, LOperator.OR)
                else:
                    return LOperator(ch, self.debuginfo, LOperator.BITWISEORASSIGN)
            elif ch == "!":
                if self.peek(1) == "=":
                    return LOperator(ch + self.next(), self.debuginfo, LOperator.NOTEQUALS)
                else:
                    return LOperator(ch, self.debuginfo, LOperator.NOT)
            elif ch == "%":
                if self.peek(1) == "=":
                    return LOperator(ch + self.next(), self.debuginfo, LOperator.MODULOASSIGN)
                else:
                    return LOperator(ch, self.debuginfo, LOperator.MODULO)
            elif ch == "^":
                if self.peek(1) == "=":
                    return LOperator(ch + self.next(), self.debuginfo, LOperator.BITWISEXORASSIGN)
                else:
                    return LOperator(ch, self.debuginfo, LOperator.BITWISEXOR)
            elif ch == "~":
                return LOperator(ch, self.debuginfo, LOperator.BITWISENOT)
            elif ch == "<":
                nch = self.peek(1)
                if nch == "<":
                    nch2 = self.peek(2)
                    if nch2 == "=":
                        return LOperator(ch + self.next() + self.next(), self.debuginfo, LOperator.SHIFTLEFTASSIGN)
                    elif nch2 == "<":
                        if self.peek(3) == "=":
                            return LOperator(ch + self.next() + self.next() + self.next(), self.debuginfo, LOperator.SHIFTLEFTOVERFLOWASSIGN)
                        else:
                            return LOperator(ch + self.next() + self.next(), self.debuginfo, LOperator.SHIFTLEFTOVERFLOW)
                    else:
                        return LOperator(ch + self.next(), self.debuginfo, LOperator.SHIFTLEFT)
                elif nch == "=":
                    return LOperator(ch + self.next(), self.debuginfo, LOperator.LESSTHANOREQUALS)
                else:
                    return LOperator(ch, self.debuginfo, LOperator.LESSTHAN)
            elif ch == ">":
                nch = self.peek(1)
                if nch == ">":
                    nch2 = self.peek(2)
                    if nch2 == "=":
                        return LOperator(ch + self.next() + self.next(), self.debuginfo, LOperator.SHIFTRIGHTASSIGN)
                    elif nch2 == ">":
                        if self.peek(3) == "=":
                            return LOperator(ch + self.next() + self.next() + self.next(), self.debuginfo, LOperator.SHIFTRIGHTOVERFLOWASSIGN)
                        else:
                            return LOperator(ch + self.next() + self.next(), self.debuginfo, LOperator.SHIFTRIGHTOVERFLOW)
                    else:
                        return LOperator(ch + self.next(), self.debuginfo, LOperator.SHIFTRIGHT)
                elif nch == "=":
                    return LOperator(ch + self.next(), self.debuginfo, LOperator.GREATERTHANOREQUALS)
                else:
                    return LOperator(ch, self.debuginfo, LOperator.GREATERTHAN)
            elif ch == "{":
                return LMisc(ch, self.debuginfo, LMisc.OPENBRACE)
            elif ch == "}":
                return LMisc(ch, self.debuginfo, LMisc.CLOSEBRACE)
            elif ch == "[":
                return LMisc(ch, self.debuginfo, LMisc.OPENBRACKET)
            elif ch == "]":
                return LMisc(ch, self.debuginfo, LMisc.CLOSEBRACKET)
            elif ch == "(":
                return LMisc(ch, self.debuginfo, LMisc.OPENPARENTHESIS)
            elif ch == ")":
                return LMisc(ch, self.debuginfo, LMisc.CLOSEPARENTHESIS)
            elif ch.isnumeric():
                ty = LLiteral.TYPE_INT32
                while self.peek(1).isnumeric() or (self.peek(1) == "." and ty == LLiteral.TYPE_INT32):
                    nch = self.next()
                    textbuffer += nch
                    if nch == ".":
                        ty = LLiteral.TYPE_FLOAT32
                return LLiteral(textbuffer, self.debuginfo)
            else:
                return LUnknown(ch, self.debuginfo)
        else:
            return None
    
    def start(self) -> list[Lexem]:
        while self.index < len(self.file):
            _l = self.next_lexem()
            if _l != None:
                self.lexems.append(_l)
                logging.debug("Found lexem: %s", str(_l))
        
        return self.lexems
    
    def trim(self):
        logging.info("Filtering LWhitespace, LComment and LUnknown...")

        comments = 0
        unknowns = 0
        whitespaces = 0
        _lexems = []
        for lexem in self.lexems:
            if lexem.__class__.TYPE_ID == LComment.TYPE_ID:
                logging.debug("Removing: %s", str(lexem))
                comments += 1
            elif lexem.__class__.TYPE_ID == LWhitespace.TYPE_ID:
                logging.debug("Removing: %s", str(lexem))
                whitespaces += 1
            elif lexem.__class__.TYPE_ID == LUnknown.TYPE_ID:
                logging.warning("Unknown token: %s (in %s:%d:%d)", repr(lexem.text), self.filepath, lexem.sline, lexem.scolumn)
                logging.debug("Removing: %s", str(lexem))
                unknowns += 1
            else:
                _lexems.append(lexem)
        self.lexems = _lexems
        logging.info("Removed %d whitespace(s), %d comment(s) and %d unknown(s)", whitespaces, comments, unknowns)

def requestfile(filename) -> str:
    if not os.path.exists(filename):
        logging.error("Requested file not found: %s", colorama.Style.BRIGHT + colorama.Fore.LIGHTYELLOW_EX + filename)
        quit()
    else:
        logging.info("File requested: %s", colorama.Style.BRIGHT + colorama.Fore.LIGHTYELLOW_EX + filename)

    contents = ""
    with open(filename, "r") as file:
        contents = file.read()
    
    return contents

def highlight(lexems: list[Lexem]) -> str:
    buffer = ""
    for lexem in lexems:
        buffer += f"{HIGHLIGHT_COLORS[lexem.__class__.TYPE_ID]}{lexem.text}{colorama.Style.RESET_ALL}"
    return buffer

def compile(options):
    logging.info("Input file received: %s", colorama.Style.BRIGHT + colorama.Fore.LIGHTYELLOW_EX + options.inputfile)
    logging.info("Entering phase 1 - Lexical Analysis")

    lexer = Lexer(options, options.inputfile)
    lexems = lexer.start()

    if options.highlight:
        print(highlight(lexems))
    
    logging.info("Preparing for phase 2...")
    lexer.trim()