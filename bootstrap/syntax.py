import logging
import colorama
from .symbol import Symbol, Type
from .lexer import Lexer, Lexem, LIdent, LKeyword, LMisc, LLiteral, LOperator

ASSIGNMENT_OPERATORS = [
    LOperator.ADDASSIGN, LOperator.BITWISEANDASSIGN, LOperator.BITWISEORASSIGN,
    LOperator.BITWISEXORASSIGN, LOperator.DIVASSIGN, LOperator.MODULOASSIGN,
    LOperator.MULASSIGN, LOperator.SHIFTLEFTASSIGN, LOperator.SHIFTLEFTOVERFLOWASSIGN,
    LOperator.SHIFTRIGHTASSIGN, LOperator.SHIFTRIGHTOVERFLOWASSIGN, LOperator.SUBASSIGN,
]

def getlocation(lexem) -> str:
    return "%s:%d:%d" % (lexem.ifile, lexem.sline + 1, lexem.scolumn + 1)

def asserttype(inputtype: Type, targettype: Type | None, loc: str) -> Type:
    """
    Ensure inputtype == targettype, or when targettype == None, infer the type from inputtype
    """
    if targettype == None:
        return inputtype
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
        self.ispublic = False

        self.startscope("__HUG__main")

        while self.index < len(self.lexems):
            entry = self.next_entry()
            if entry != None:
                logging.debug("Resulting ParseTreeEntry: %s", entry)
                self.entries.append(entry)
        
        self.endscope()

        logging.info("Resulting instructions:")
        for entry in self.entries:
            logging.info(str(entry))
    
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
                    variabletype, variablevalue = self.parseexpression(vtype = variabletype, converttype = True)

                    return VariableDefinition(self.createsymbol(variablename, variabletype), variablevalue)
                else:
                    logging.error("Expected : or = at %s", getlocation(nextsymbol))
                    quit()
            elif lexem.text == "pub":
                self.flagpublic()
                return self.next_entry()
            elif lexem.text == "if":
                resultingtype, condition = self.parseexpression(vtype=Type(Type.BOOL))
                iftrue = self.parsescope()
                iffalse = None
                if type(self.peek(1)) is LKeyword and self.peek(1).text == "else":
                    self.next()
                    iffalse = self.parsescope()

                return ConditionalJump(condition, iftrue, iffalse)
        elif type(lexem) is LIdent:
            nextlexem = self.next()
            if type(nextlexem) is LOperator:
                if nextlexem.operator in ASSIGNMENT_OPERATORS:
                    variable = self.findsymbol(lexem.text)
                    vtype, value = self.parseexpression(variable.vtype)
                    return VariableAssignment(variable, Operation(Variable(variable), nextlexem, value, vtype))
                elif nextlexem.operator == LOperator.INCREMENT:
                    variable = self.findsymbol(lexem.text)
                    return VariableAssignment(variable, Operation(Variable(variable), LOperator("++", None, LOperator.ADD), Constant(1), variable.vtype))
                elif nextlexem.operator == LOperator.DECREMENT:
                    variable = self.findsymbol(lexem.text)
                    return VariableAssignment(variable, Operation(Variable(variable), LOperator("--", None, LOperator.SUB), Constant(1), variable.vtype))
                elif nextlexem.operator == LOperator.ASSIGN:
                    variable = self.findsymbol(lexem.text)
                    vtype, value = self.parseexpression(variable.vtype)
                    return VariableAssignment(variable, value)
                else:
                    logging.error("Invalid operator at %s, expected assignment", getlocation(nextlexem))
                    quit()
            elif type(nextlexem) is LMisc:
                if nextlexem.kind == LMisc.OPENPARENTHESIS:
                    print("Function call")
                else:
                    logging.error("Unexpected ( at %s", getlocation(nextlexem))
                    quit()

    
    def getliteral(self, value, vtype = None, converttype: bool = False):
        if type(value) is LLiteral:
            if value.kind == LLiteral.TYPE_INT:
                vtype = asserttype(Type(Type.INT32), vtype, getlocation(value)) if not converttype or vtype == None else vtype
            elif value.kind == LLiteral.TYPE_FLOAT:
                vtype = asserttype(Type(Type.FLOAT32), vtype, getlocation(value)) if not converttype or vtype == None else vtype
            elif value.kind == LLiteral.TYPE_BOOL:
                vtype = asserttype(Type(Type.BOOL), vtype, getlocation(value)) if not converttype or vtype == None else vtype
            elif value.kind == LLiteral.TYPE_CHAR:
                vtype = asserttype(Type(Type.CHAR), vtype, getlocation(value)) if not converttype or vtype == None else vtype
            elif value.kind == LLiteral.TYPE_STRING:
                vtype = asserttype(Type(Type.STRING), vtype, getlocation(value)) if not converttype or vtype == None else vtype
            return vtype, value.value
        else:
            logging.error("Expected literal at %s", getlocation(value))
            quit()
        

    def parseexpression(self, vtype = None, converttype: bool = False):
        leftvtype = None
        leftvalue = None
        operator = None
        rightvalue = None

        nextsymbol = self.next()
        if type(nextsymbol) is LLiteral:
            leftvtype, _vvalue = self.getliteral(nextsymbol, vtype, converttype)
            leftvalue = Constant(_vvalue)
        elif type(nextsymbol) is LIdent:
            variable = self.findsymbol(nextsymbol.text)
            leftvalue = Variable(variable)
            leftvtype = variable.vtype
        elif type(nextsymbol) is LOperator:
            if nextsymbol.operator == LOperator.BITWISENOT or nextsymbol.operator == LOperator.NOT:
                operator = nextsymbol.operator
                nextsymbol = self.next()
                if type(nextsymbol) is LLiteral:
                    leftvtype, literal = self.getliteral(self.next())
                    leftvalue = Operation(Constant(literal), operator, Constant(1), leftvtype)
                elif type(nextsymbol) is LIdent:
                    symbol = self.findsymbol(nextsymbol.text, forceexists = True)
                    leftvalue = Operation(Variable(symbol), operator, Constant(1), symbol.vtype)
            else:
                logging.error("Invalid operator at %s", getlocation(nextsymbol))
                quit()
        else:
            logging.error("Invalid expression at %s", getlocation(nextsymbol))
            quit()
        
        if type(self.peek(1)) is LOperator:
            operator = self.next()
            
            if operator.operator >= LOperator.EQUALS and operator.operator <= LOperator.OR:
                _vtype, rightvalue = self.parseexpression(leftvtype)
                vtype = Type(Type.BOOL)
            else:
                _vtype, rightvalue = self.parseexpression(vtype)
                vtype = asserttype(leftvtype, vtype, getlocation(operator))
                vtype = asserttype(_vtype, vtype, getlocation(operator))
            logging.debug("Math expression found: %s %s %s", leftvalue, operator, rightvalue)
            return vtype, Operation(leftvalue, operator, rightvalue, vtype)
        else:
            logging.debug("Expression found: %s", leftvalue)
            return leftvtype, leftvalue

    def parsescope(self, name: str = ""):
        nextlexem = self.next()
        if type(nextlexem) is not LMisc or nextlexem.kind != LMisc.OPENBRACE:
            logging.error("Expected code block at %s", getlocation(nextlexem))

        self.startscope(name)

        entries = []
        while self.index < len(self.lexems):
            peek = self.peek(1)
            if type(peek) is LMisc and peek.kind == LMisc.CLOSEBRACE:
                self.next()
                break
            
            entry = self.next_entry()
            if entry != None:
                logging.debug("Resulting ParseTreeEntry: %s", entry)
                entries.append(entry)

        scope = self.endscope()

        return CodeBlock(scope, entries)

    def flagpublic(self):
        self.ispublic = True
    
    def getpublic(self) -> bool:
        _p = self.ispublic
        self.ispublic = False
        return _p

    def findsymbol(self, name: str, forceexists: bool = False) -> Symbol | None:
        logging.debug("Trying to find symbol %s", repr(name))
        for i in range(len(self.scopestack)):
            scope = self.scopestack[len(self.scopestack) - 1 - i]
            for child in scope.children:
                if child.name == name:
                    logging.debug("Found: %s as %s", str(child), child.vtype)
                    return child
        logging.debug("Symbol did not exist")
        if forceexists:
            logging.error("Variable %s not found!", repr(name))
            quit()
        return None
    
    def startscope(self, name: str):
        self.scopestack.append(self.createsymbol(name, Type(Type.SCOPE)))
    
    def endscope(self):
        return self.scopestack.pop()

    def createsymbol(self, name: str, vtype: Type, dontsearch: bool = False) -> Symbol:
        symbol = None
        if not dontsearch:
            symbol = self.findsymbol(name)
            if symbol != None:
                logging.error("Symbol %s already exists! (%s)", repr(name), str(scope))
                quit()
        
        logging.debug("Creating symbol %s as %s", repr(name), vtype)
        
        if len(self.scopestack) > 0:
            symbol = Symbol(name, vtype, self.scopestack[-1], self.getpublic())
        else:
            symbol = Symbol(name, vtype, None, self.getpublic())
        return symbol
    
    def findorcreatesymbol(self, name: str, vtype: Type) -> Symbol:
        symbol = self.findsymbol(name)
        if symbol == None:
            return self.createsymbol(name, vtype)
        else:
            return symbol

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

class ConditionalJump(ParseTreeEntry):
    def __init__(self, condition: Expression, iftrue: Symbol, iffalse: Symbol | None):
        super().__init__()
        self.condition = condition
        self.iftrue = iftrue
        self.iffalse = iffalse
    
    def __str__(self) -> str:
        return f"ConditionalJump: if {self.condition} {{ {self.iftrue} }} else {{ {self.iffalse} }}"

class CodeBlock(ParseTreeEntry):
    def __init__(self, symbol: Symbol, entries: list[ParseTreeEntry]):
        super().__init__()
        self.symbol = symbol
        self.entries = entries