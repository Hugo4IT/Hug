import logging
import colorama
from .symbol import Symbol, Type
from .lexer import Lexer, Lexem, LIdent, LKeyword, LMisc, LLiteral, LOperator

ASSIGNMENT_OPERATORS = [
    LOperator.ASSIGN, LOperator.ADDASSIGN, LOperator.BITWISEANDASSIGN,
    LOperator.BITWISEORASSIGN, LOperator.BITWISEXORASSIGN, LOperator.DIVASSIGN,
    LOperator.MODULOASSIGN, LOperator.MULASSIGN, LOperator.SHIFTLEFTASSIGN,
    LOperator.SHIFTLEFTOVERFLOWASSIGN, LOperator.SHIFTRIGHTASSIGN,
    LOperator.SHIFTRIGHTOVERFLOWASSIGN, LOperator.SUBASSIGN,
]

def getlocation(symbol: Symbol) -> str:
    return "%s:%d:%d" % (symbol.ifile, symbol.sline + 1, symbol.scolumn + 1)

def asserttype(inputtype: Type, targettype: Type | None, loc: str) -> Type:
    if targettype == None:
        targettype = inputtype
    elif targettype.ty != inputtype.ty:
        logging.error("Type mismatch, target type was %s, but expression resulted in %s (at %s)", targettype, inputtype, loc)
        quit()
    
    return targettype

class ParseTree:
    def __init__(self, lexer: Lexer):
        self.entries = []
        self.scopestack = []
        self.lexems = lexer.lexems
        self.index = -1

        self.startscope("__HUG__main")

        while self.index < len(self.lexems):
            entry = self.next_entry()
            if entry != None:
                logging.debug("Resulting ParseTreeEntry: %s", entry)
                self.entries.append(entry)
        
        self.endscope()

        logging.debug("Resulting parse tree")
        for entry in self.entries:
            logging.debug(str(entry))
    
    def next(self) -> Lexem:
        self.index += 1
        if self.index < len(self.lexems):
            logging.debug("Processing %s", str(self.lexems[self.index]))
            return self.lexems[self.index]
        else:
            return None

    def peek(self, peek: int) -> str | None:
        if self.index + peek < len(self.lexems):
            return self.lexems[self.index + peek]
        else:
            return None
    
    def next_entry(self):
        lexem = self.next()
        if lexem == None:
            return None
        if type(lexem) is LKeyword:
            if lexem.text == "let":
                variablename = self.next()
                if type(variablename) is not LIdent:
                    logging.error("Expected identifier at %s", getlocation(variablename))
                    quit()
                variablename = variablename.text

                variabletype = None
                variablevalue = None

                nextsymbol = self.next()

                ## let myVar: Int8 = 10
                ##          ^
                if type(nextsymbol) is LMisc and nextsymbol.kind == LMisc.COLON:
                    ## let myVar: Int8 = 10
                    ##            ^^^^
                    typespecifier = self.next()
                    variabletype = Type.fromstring(typespecifier.text)

                    ## let myVar: Int8 = 10
                    ##                 ^
                    nextsymbol = self.next()
                
                ## let myVar = 10
                ##           ^
                if type(nextsymbol) is LOperator and nextsymbol.operator == LOperator.ASSIGN:
                    ## let myVar = 10
                    ##             ^^
                    variabletype, variablevalue = self.parseexpression(vtype = variabletype)

                    return VariableDefinition(self.createsymbol(variablename, variabletype), variablevalue)
                    print("Variable:", variablename, variabletype, variablevalue)
                else:
                    logging.error("Expected : or = at %s", getlocation(nextsymbol))
                    quit()
        elif type(lexem) is LIdent:
            nextlexem = self.next()
            if type(nextlexem) is LOperator:
                if nextlexem.operator in ASSIGNMENT_OPERATORS:
                    print("Assignment")
                elif nextlexem.operator == LOperator.INCREMENT or nextlexem.operator == LOperator.DECREMENT:
                    print("++ or --")
                else:
                    logging.error("Invalid operator at %s, expected assignment", getlocation(nextlexem))
                    quit()
            elif type(nextlexem) is LMisc:
                if nextlexem.kind == LMisc.OPENPARENTHESIS:
                    print("Function call")
                else:
                    logging.error("Unexpected ( at %s", getlocation(nextlexem))
                    quit()

    
    def getliteral(self, value, vtype = None):
        if type(value) is LLiteral:
            if vtype == None:
                if value.kind == LLiteral.TYPE_INT:
                    vtype = Type(Type.INT32)
                elif value.kind == LLiteral.TYPE_FLOAT:
                    vtype = Type(Type.FLOAT32)
                elif value.kind == LLiteral.TYPE_BOOL:
                    vtype = Type(Type.BOOL)
                elif value.kind == LLiteral.TYPE_CHAR:
                    vtype = Type(Type.CHAR)
                elif value.kind == LLiteral.TYPE_STRING:
                    vtype = Type(Type.STRING)
                return vtype, value.value
            else:
                return vtype, value.value

    def parseexpression(self, vtype = None):
        leftvtype = None
        leftvalue = None
        operator = None
        rightvalue = None

        nextsymbol = self.next()
        if type(nextsymbol) is LLiteral:
            leftvtype, _vvalue = self.getliteral(nextsymbol, vtype)
            leftvalue = Constant(_vvalue)
        elif type(nextsymbol) is LIdent:
            leftvalue = Variable(nextsymbol)
        elif type(nextsymbol) is LOperator:
            if nextsymbol.operator == LOperator.BITWISENOT:
                print("~")
            elif nextsymbol.operator == LOperator.NOT:
                print("!")
            else:
                logging.error("Invalid operator at %s", getlocation(nextsymbol))
                quit()
        else:
            logging.error("Invalid expression at %s", getlocation(nextsymbol))
            quit()
        
        if type(self.peek(1)) is LOperator:
            operator = self.next()
            
            _vtype, rightvalue = self.parseexpression(vtype)
            if operator.operator >= LOperator.EQUALS and operator.operator <= LOperator.OR:
                vtype = Type.BOOL
            else:
                vtype = asserttype(leftvtype, vtype, getlocation(operator))
                vtype = asserttype(_vtype, vtype, getlocation(operator))
            logging.debug("Math expression found: %s %s %s", leftvalue, operator, rightvalue)
            return vtype, Operation(leftvalue, operator, rightvalue, vtype)
        else:
            logging.debug("Expression found: %s", leftvalue)
            return leftvtype, leftvalue

    def findsymbol(self, name: str) -> Symbol | None:
        for i in range(len(self.scopestack)):
            scope = self.scopestack[len(self.scopestack) - 1 - i]
            for child in scope.children:
                if child.name == name:
                    return child
        return None
    
    def startscope(self, name: str):
        self.scopestack.append(self.createsymbol(name, Type(Type.SCOPE)))
    
    def endscope(self):
        self.scopestack.pop()

    def createsymbol(self, name: str, vtype: Type) -> Symbol:
        symbol = self.findsymbol(name)
        if symbol != None:
            logging.error("Symbol %s already exists! (%s)", repr(name), str(scope))
            quit()
        
        if len(self.scopestack) > 0:
            return Symbol(name, vtype, self.scopestack[-1])
        else:
            return Symbol(name, vtype, None)

class ParseTreeEntry:
    def __init__(self):
        pass

class Expression(ParseTreeEntry):
    def __init__(self):
        super().__init__()

class Variable(Expression):
    def __init__(self, symbol: Symbol):
        super().__init__()
        self.symbol = symbol
    
    def __str__(self) -> str:
        return f"Variable({self.symbol})"

class Operation(Expression):
    def __init__(self, left: Expression, operator: LOperator, right: Expression, vtype: Type):
        super().__init__()
        self.left = left
        self.operator = operator
        self.right = right
        self.vtype = vtype
    
    def __str__(self) -> str:
        return f"Operation({self.left} {self.operator} {self.right}) as {self.vtype}"
    
class Constant(Expression):
    def __init__(self, value):
        super().__init__()
        self.value = value
    
    def __str__(self) -> str:
        return f"Constant({self.value})"

class Function(Expression):
    def __init__(self):
        super().__init__()
    
    def __str__(self) -> str:
        return "Function()"

class VariableDefinition(ParseTreeEntry):
    def __init__(self, ident: Symbol, value: Expression):
        super().__init__()
        self.ident = ident
        self.value = value
    
    def __str__(self) -> str:
        return f"VariableDefinition: {self.ident} = {self.value}"

class VariableAssignment(ParseTreeEntry):
    def __init__(self, var: Symbol, value: Expression):
        super().__init__()
        self.var = var
        self.value = value

    def __str__(self) -> str:
        return f"VariableAssignment: {self.var} = {self.value}"