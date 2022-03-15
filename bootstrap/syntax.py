import logging
import colorama
from .ident import Symbol, Type
from .lexer import Lexer, Lexem, LIdent, LKeyword, LMisc, LLiteral, LOperator

class ParseTree:
    def __init__(self, lexer: Lexer):
        globalscope = VariableDefinition([], Symbol("__HUG__main", Type(Type.SCOPE), None), Function())

        self.entries = [globalscope]
        self.scopestack = [globalscope]
        self.lexems = lexer.lexems
        self.index = 0

        while self.index < len(self.lexems):
            self.entries.append(self.next_entry())
    
    def next(self) -> Lexem:
        self.index += 1
        if self.index < len(self.lexems):
            return self.lexems[self.index - 1]
        else:
            lastlexem = self.lexems[self.index - 2]
            logging.error("Expected token at %s:%d:%d", lastlexem.ifile, lastlexem.eline, lastlexem.ecolumn)
            quit()

    def peek(self, peek: int) -> str | None:
        if self.index + peek < len(self.lexems):
            return self.lexems[self.index + peek]
        else:
            return None
    
    def next_entry(self):
        lexem = self.next()
        if type(lexem) is LKeyword:
            if lexem.text == "let":
                variablename = self.next()
                if type(variablename) is not LIdent:
                    logging.error("Expected identifier at %s:%d:%d", variablename.ifile, variablename.sline, variablename.scolumn)
                    quit()
                variablename = variablename.text

                variabletype = None
                variablevalue = None

                nextinstruction = self.next()
                if type(nextinstruction) is LMisc and nextinstruction.value == LMisc.COLON:
                    print("process type")
                    nextinstruction = self.next()
                
                if type(nextinstruction) is LOperator and nextinstruction.operator == LOperator.ASSIGN:
                    nextinstruction = self.next()
                    if type(nextinstruction) is LLiteral:
                        if variabletype == None:
                            if nextinstruction.vtype.ty == LLiteral.TYPE_INT:
                                variabletype = Type.INT32
                            elif nextinstruction.vtype.ty == LLiteral.TYPE_FLOAT:
                                variabletype = Type.FLOAT32
                            elif nextinstruction.vtype.ty == LLiteral.TYPE_BOOL:
                                variabletype = Type.BOOL
                            elif nextinstruction.vtype.ty == LLiteral.TYPE_CHAR:
                                variabletype = Type.CHAR
                            elif nextinstruction.vtype.ty == LLiteral.TYPE_STRING:
                                variabletype = Type.STRING
                            
                            variablevalue = nextinstruction.value

                            print("Variable:", variablename, variabletype, variablevalue)
                        else:
                            print("process variable assignment to literal enforcing type")
                    elif type(nextinstruction) is LIdent:
                        print("process variable assignment to ident")
                else:
                    logging.error("Expected : or = at %s:%d:%d", nextinstruction.ifile, nextinstruction.sline, nextinstruction.scolumn)
                    quit()
    
    def findsymbol(self, name: str) -> Symbol | None:
        for i in range(len(self.scopestack)):
            scope = self.scopestack[len(self.scopestack) - 1 - i]
            for child in scope.children:
                if child.name == name:
                    return child
        return None


class ParseTreeEntry:
    def __init__(self):
        pass

class Expression(ParseTreeEntry):
    def __init__(self):
        super().__init__()

class Function(Expression):
    def __init__(self):
        super().__init__()

class DefinitionSpecifier(ParseTreeEntry):
    def __init__(self):
        super().__init__()

class VariableDefinition(ParseTreeEntry):
    def __init__(self, specifiers: list[DefinitionSpecifier], ident: Symbol, value: Expression):
        super().__init__()
        self.specifiers = specifiers
        self.ident = ident
        self.value = value