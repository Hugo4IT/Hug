import logging
import colorama
from .ident import Identifier

class ParseTree:
    def __init__(self):
        self.entries = [
            VariableDefinition([], Identifier("main", None), Function()) # Create global main scope
        ]

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
    def __init__(self, specifiers: list[DefinitionSpecifier], ident: Identifier, value: Expression):
        super().__init__()
        self.specifiers = specifiers
        self.ident = ident
        self.value = value