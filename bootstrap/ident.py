import logging
import colorama

class IdentifierTable:
    __instance = None
    def __init__(self):
        if IdentifierTable.__instance != None:
            raise Exception("IdentifierTable is a singleton.")
        IdentifierTable.__instance = self

        self.entries = []
    
    @staticmethod
    def instance():
        if IdentifierTable.__instance == None:
            IdentifierTable()
        return IdentifierTable.__instance
    
    @staticmethod
    def push(ident) -> int:
        IdentifierTable.instance().entries.append(ident)
        return len(IdentifierTable.instance().entries) - 1

class Identifier:
    def __init__(self, name: str, scope):
        self.name = name
        self.scope = scope
        self.id = IdentifierTable.push(self)