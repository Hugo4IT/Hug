import os
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
    """
    Get formatted location of lexem (provided by debuginfo in lexing phase)
    """
    return "%s:%d:%d" % (lexem.ifile, lexem.sline + 1, lexem.scolumn + 1)

def asserttype(inputtype: Type, targettype: Type | None, loc: str | None, errormessage: str | None = None) -> Type:
    """
    Ensure inputtype == targettype, or when targettype == None, infer the type from inputtype
    """
    if targettype == None:
        return inputtype
    elif targettype.ty != inputtype.ty:
        if loc != None:
            logging.error("Type mismatch, target type was %s, but expression resulted in %s (at %s)", targettype, inputtype, loc)
        else:
            logging.error(errormessage)
        quit()
    
    return targettype

class ParseTree:
    def __init__(self, lexer: Lexer):
        self.scopestack = []
        self.lexems = lexer.lexems
        self.index = -1
        self.ispublic = False

        globalscope = self.parsescope((os.path.basename(lexer.options.inputfile) + ".").split(".")[0], erroroneof = False)

        logging.info("Resulting ParseTree:")
        for line in str(globalscope).split("\n"):
            logging.info(line)
    
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

                #> let myVar: Int8 = 10
                #           ^
                if type(nextsymbol) is LMisc and nextsymbol.kind == LMisc.COLON:
                    #> let myVar: Int8 = 10
                    #             ^^^^
                    typespecifier = self.next()
                    variabletype = Type.fromstring(typespecifier.text)

                    #> let myVar: Int8 = 10
                    #                  ^
                    nextsymbol = self.next()
                
                #> let myVar = 10
                #            ^
                if type(nextsymbol) is LOperator and nextsymbol.operator == LOperator.ASSIGN:
                    #> let myVar = 10
                    #              ^^
                    variabletype, variablevalue = self.parseexpression(vtype = variabletype, converttype = True)

                    return VariableDefinition(self.createsymbol(variablename, variabletype), variablevalue)
                else:
                    logging.error("Expected : or = at %s", getlocation(nextsymbol))
                    quit()
            elif lexem.text == "pub":
                self.flagpublic()
                return self.next_entry()
            elif lexem.text == "if":
                #> if condition == true { print("Hi") } else { print("bye") }
                #     ^^^^^^^^^^^^^^^^^
                resultingtype, condition = self.parseexpression(vtype=Type(Type.BOOL))

                #> if condition == true { print("Hi") } else { print("bye") }
                #                       ^
                self.next()
                #> if condition == true { print("Hi") } else { print("bye") }
                #                         ^^^^^^^^^^^^^
                iftrue = self.parsescope()
                iffalse = None

                #> if condition == true { print("Hi") } else { print("bye") }
                #                                       ^^^^
                if type(self.peek(1)) is LKeyword and self.peek(1).text == "else":
                    self.next() # else
                    self.next() # {

                    #> if condition == true { print("Hi") } else { print("bye") }
                    #                                            ^^^^^^^^^^^^^^^^
                    iffalse = self.parsescope()

                return ConditionalJump(condition, iftrue, iffalse)
            elif lexem.text == "fn":
                functionname = self.next()
            else:
                logging.error("Sorry, but %s is a reserved keyword and may not be used as identitifer", repr(lexem.text))
        elif type(lexem) is LIdent: # <ident>
            nextlexem = self.next()
            if type(nextlexem) is LOperator: # <ident> <operator>
                # ASSIGNMENT_OPERATORS = [+=, *=, <<<=, etc.]
                if nextlexem.operator in ASSIGNMENT_OPERATORS: # <ident> <assignmentoperator>
                    #> number += 5
                    #  ^^^^^^
                    variable = self.findsymbol(lexem.text)

                    #> number += 5
                    #         ^^^^
                    vtype, value = self.parseexpression(variable.vtype)
                    return VariableAssignment(variable, Operation(Variable(variable), nextlexem, value, vtype))
                # elif nextlexem.operator == LOperator.INCREMENT:
                #     variable = self.findsymbol(lexem.text)
                #     return VariableAssignment(variable, Operation(Variable(variable), LOperator("++", None, LOperator.ADD), Constant(1), variable.vtype))
                # elif nextlexem.operator == LOperator.DECREMENT:
                #     variable = self.findsymbol(lexem.text)
                #     return VariableAssignment(variable, Operation(Variable(variable), LOperator("--", None, LOperator.SUB), Constant(1), variable.vtype))
                elif nextlexem.operator == LOperator.ASSIGN: # <ident> <operator '='>
                    #> number = 10
                    #  ^^^^^^
                    variable = self.findsymbol(lexem.text)

                    #> number = 10
                    #         ^^^^
                    vtype, value = self.parseexpression(variable.vtype)
                    return VariableAssignment(variable, value)
                else: # <ident> <operator !invalid>
                    logging.error("Invalid operator at %s, expected assignment", getlocation(nextlexem))
                    quit()
            elif type(nextlexem) is LMisc: # <ident> <misc>
                if nextlexem.kind == LMisc.OPENPARENTHESIS: # <ident> <misc '('>
                    print("Function call")
                else: # <ident> <misc !invalid>
                    logging.error("Unexpected ( at %s", getlocation(nextlexem))
                    quit()
        elif type(lexem) is LMisc:
            # Using {} will create an anonymous scope, discarding all
            # variables created in it when the scope ends
            #> { let scopedVariable = 10 }
            #  ^
            if lexem.kind == LMisc.OPENBRACE:
                #> { let scopedVariable = 10 }
                #    ^^^^^^^^^^^^^^^^^^^^^^^^^
                return self.parsescope()
    
    def getliteral(self, value, vtype = None, converttype: bool = False):
        """
        Converts an self.next() as LLiteral into a Type and a value
        """

        if type(value) is LLiteral:
            if value.kind == LLiteral.TYPE_INT: # Number without decimal point
                vtype = asserttype(Type(Type.INT32), vtype, getlocation(value)) if not converttype or vtype == None else vtype
            elif value.kind == LLiteral.TYPE_FLOAT: # Number with decimal point
                vtype = asserttype(Type(Type.FLOAT32), vtype, getlocation(value)) if not converttype or vtype == None else vtype
            elif value.kind == LLiteral.TYPE_BOOL: # Text equal to true or false
                vtype = asserttype(Type(Type.BOOL), vtype, getlocation(value)) if not converttype or vtype == None else vtype
            elif value.kind == LLiteral.TYPE_CHAR: # Text surrounded by '
                vtype = asserttype(Type(Type.CHAR), vtype, getlocation(value)) if not converttype or vtype == None else vtype
            elif value.kind == LLiteral.TYPE_STRING: # Text surrounded by "
                vtype = asserttype(Type(Type.STRING), vtype, getlocation(value)) if not converttype or vtype == None else vtype
            return vtype, value.value
        else:
            logging.error("Expected literal at %s", getlocation(value))
            quit()
        

    def parseexpression(self, vtype = None, converttype: bool = False):
        """
        Recursively parse an expression, examples of expressions are:

            #>  5
            ->  Constant(5)
        
            #>  50.0 / 30.0
            ->  Operation(Constant(50.0), /, Constant(30.0))
        
            #>  variable % 10 * otherVariable
            ->  Operation(Variable(variable), %, Operation(Constant(10), *, Variable(otherVariable)))
        
        #> = Input Hug code
        -> = Output Expression

        Details: parseexpression() tries to interpret self.next() as some kind of Expression,
          either Variable or Constant, then it checks if the next lexem is an operator (+, -, *, /, etc.),
          if it is an operator it will return an Operation with the first parsed Expression, the operator
          and the result of a (recursive) call to parseexpression to figure out the right side of the operation.
          If there wasn't an operator, it will just return the first parsed Expression as Constant or Variable
        """

        leftvtype = None
        leftvalue = None
        operator = None
        rightvalue = None

        #> 10 + 5
        #  ^^
        nextsymbol = self.next()
        if type(nextsymbol) is LLiteral: # <literal>
            leftvtype, _vvalue = self.getliteral(nextsymbol, vtype, converttype)
            leftvalue = Constant(_vvalue, leftvtype)
        elif type(nextsymbol) is LIdent: # <ident>
            variable = self.findsymbol(nextsymbol.text)
            leftvalue = Variable(variable)
            leftvtype = variable.vtype
        elif type(nextsymbol) is LOperator: # <operator>
            if nextsymbol.operator == LOperator.BITWISENOT or nextsymbol.operator == LOperator.NOT:
                #> ~3
                #  ^
                operator = nextsymbol.operator

                #> ~3
                #   ^
                nextsymbol = self.next()
                if type(nextsymbol) is LLiteral: # <operator '!' | '~'> <ident>
                    leftvtype, literal = self.getliteral(self.next())
                    leftvalue = Operation(Constant(literal, leftvtype), operator, Constant(1, symbol.vtype), leftvtype)
                elif type(nextsymbol) is LIdent:
                    symbol = self.findsymbol(nextsymbol.text, forceexists = True)
                    leftvalue = Operation(Variable(symbol), operator, Constant(1, symbol.vtype), symbol.vtype)
            else: # <operator> <!unknown>
                logging.error("Invalid operator at %s", getlocation(nextsymbol))
                quit()
        else: # <!unknown>
            logging.error("Invalid expression at %s", getlocation(nextsymbol))
            quit()
        
        if type(self.peek(1)) is LOperator: # <parsed Expression> <operator>
            #> 10 + 5
            #     ^
            operator = self.next()
            
            # Checks if operator is a logic operator (==, !=, >=, &&, etc.)
            #> 10 != 5
            #     ^^
            if operator.operator >= LOperator.EQUALS and operator.operator <= LOperator.OR: # <parsed Expression> <logic operator>
                #> 10 != 5
                #        ^
                _vtype, rightvalue = self.parseexpression(leftvtype)
                vtype = Type(Type.BOOL) # Force expression type into Bool
            else:
                #> 10 + 5
                #       ^
                _vtype, rightvalue = self.parseexpression(vtype)
                vtype = asserttype(leftvtype, vtype, getlocation(operator))
                vtype = asserttype(_vtype, vtype, getlocation(operator))
            
            logging.debug("Math expression found: %s %s %s", leftvalue, operator, rightvalue)
            return vtype, Operation(leftvalue, operator, rightvalue, vtype)
        else: # <parsed Expression>
            logging.debug("Expression found: %s", leftvalue)
            return leftvtype, leftvalue

    def parsescope(self, name: str = "", erroroneof: bool = True):
        """
        Redirect all ParseTreeEntrys starting at the nearest { and ending at the
          nearest } into a CodeBlock with an anonymous Symbol as identifier.
        """

        self.startscope(name)

        entries = []
        while self.index < len(self.lexems): # Continue unless unexpected EOF
            peek = self.peek(1)
            if type(peek) is LMisc and peek.kind == LMisc.CLOSEBRACE:
                self.next() # Skip }
                return CodeBlock(self.endscope(), entries)
            
            entry = self.next_entry()
            if entry != None:
                logging.debug("Resulting ParseTreeEntry: %s", entry)
                entries.append(entry)

        if erroroneof:
            logging.error("Unexpected EOF!")
            quit()
        else:
            return CodeBlock(self.endscope(), entries)

    def flagpublic(self):
        self.ispublic = True
    
    def getpublic(self) -> bool:
        _p = self.ispublic
        self.ispublic = False
        return _p

    def findsymbol(self, name: str, forceexists: bool = False) -> Symbol | None:
        """
        Iterates through the scope stack right-to-left (optimization)
          to find a Symbol with the same name, if no Symbol was found and
          forceexists == False it will return None
        """

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
        """
        If `dontsearch` == False will check if a symbol already exists to prevent errors.
          When ensured that no duplicate exists, it will create a new symbol with `name` and `vtype`
        """

        symbol = None
        if not dontsearch:
            symbol = self.findsymbol(name)
            if symbol != None:
                logging.error("Symbol %s already exists! (as %s)", repr(name), str(symbol))
                quit()
        
        logging.debug("Creating symbol %s as %s", repr(name), vtype)
        
        if len(self.scopestack) > 0:
            symbol = Symbol(name, vtype, self.scopestack[-1], self.getpublic())
        else:
            symbol = Symbol(name, vtype, None, self.getpublic())
        return symbol
    
    def findorcreatesymbol(self, name: str, vtype: Type) -> Symbol:
        """
        If a symbol with `name` exists, return that Symbol
          otherwise return a new Symbol with `name` and `type`
        """

        symbol = self.findsymbol(name)
        if symbol == None:
            return self.createsymbol(name, vtype)
        else:
            return symbol

class ParseTreeEntry:
    def __init__(self):
        pass

class Expression(ParseTreeEntry):
    def __init__(self, vtype: Type):
        super().__init__()
        self.vtype = vtype

class Variable(Expression):
    def __init__(self, symbol: Symbol):
        self.symbol = symbol
        super().__init__(symbol.vtype)
    
    def __str__(self) -> str:
        return f"Variable({self.symbol} as {self.vtype})"

class Operation(Expression):
    def __init__(self, left: Expression, operator: LOperator, right: Expression, vtype: Type):
        self.left = left
        self.operator = operator
        self.right = right
        super().__init__(vtype)
    
    def __str__(self) -> str:
        return f"Operation({self.left} {self.operator} {self.right}) as {self.vtype}"
    
class Constant(Expression):
    def __init__(self, value, vtype: Type):
        self.value = value
        super().__init__(vtype)
    
    def __str__(self) -> str:
        return f"Constant({self.value} as {self.vtype})"

class Function(Expression):
    def __init__(self, vtype: Type):
        super().__init__(vtype)
    
    def __str__(self) -> str:
        return f"Function() -> {self.vtype}"

#> let <var> = <value>
# <var> may NOT exist in reachable scope
class VariableDefinition(ParseTreeEntry):
    def __init__(self, var: Symbol, value: Expression):
        super().__init__()
        self.var = var
        self.value = value
    
    def __str__(self) -> str:
        return f"define {self.var} as {self.var.vtype} = {self.value}"

#> <var: Symbol> = <value: Expression -> var.vtype>
# <var> MUST already exist as a variable
class VariableAssignment(ParseTreeEntry):
    def __init__(self, var: Symbol, value: Expression):
        super().__init__()
        var.vtype = asserttype(value.vtype, var.vtype, None, errormessage=f"Cannot assign value of type {value.vtype} to a variable of type {var.vtype} (variable {var})")
        self.var = var
        self.value = value

    def __str__(self) -> str:
        return f"{self.var} = {self.value}"

#> if <condition: Expression -> Bool> <iftrue: Symbol -> CodeBlock> else <iffalse: Symbol -> CodeBlock>
class ConditionalJump(ParseTreeEntry):
    def __init__(self, condition: Expression, iftrue: Symbol, iffalse: Symbol | None):
        super().__init__()
        self.condition = condition
        self.iftrue = iftrue
        self.iffalse = iffalse
    
    def __str__(self) -> str:
        return f"if {self.condition} {self.iftrue}" + "" if self.iffalse == None else f"else {self.iffalse}"

#> <symbol?> { <entries> }
# A list of instructions, with scope
class CodeBlock(ParseTreeEntry):
    def __init__(self, symbol: Symbol, entries: list[ParseTreeEntry]):
        super().__init__()
        self.symbol = symbol
        self.entries = entries
    
    def __str__(self) -> str:
        return f"scope {self.symbol}" + " {\n    " + "\n    ".join([str(e).replace("\n", "\n    ") for e in self.entries]) + "\n}"