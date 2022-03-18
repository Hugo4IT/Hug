import logging
import colorama

class SymbolTable:
    __instance = None
    def __init__(self):
        if SymbolTable.__instance != None:
            raise Exception("SymbolTable is a singleton.")
        SymbolTable.__instance = self

        self.idents = []
    
    @staticmethod
    def instance():
        if SymbolTable.__instance == None:
            SymbolTable()
        return SymbolTable.__instance
    
    @staticmethod
    def push(ident) -> int:
        SymbolTable.instance().idents.append(ident)
        return len(SymbolTable.instance().idents) - 1
    
    def __str__(self) -> str:
        """
        Horrible code for visualising a table, but it works
        """

        rows = []
        longestid = len("id")
        longestname = len("name")
        longestvtype = len("type")
        longestscope = len("scope")
        for ident in self.idents:
            _id, name, vtype, scope, ispublic = [str(getattr(ident, key)) for key in ["_id", "name", "vtype", "scope", "ispublic"]]
            longestid = max(longestid, len(_id))
            longestname = max(longestname, len(name))
            longestvtype = max(longestvtype, len(vtype))
            longestscope = max(longestscope, len(scope))
            rows.append((_id, name, vtype, scope, ispublic))
        buffer = "+"
        for longestitem in [longestid, longestname, longestvtype, longestscope, 5]:
            buffer += "-" * (longestitem + 2) + "+"
        buffer += f"\n| id{' ' * (longestid - 2)} | name{' ' * (longestname - 4)} | type{' ' * (longestvtype - 4)} | scope{' ' * (longestscope - 5)} | pub   |"
        buffer += "\n+"
        for longestitem in [longestid, longestname, longestvtype, longestscope, 5]:
            buffer += "=" * (longestitem + 2) + "+"
        for row in rows:
            buffer += "\n|"
            for column,longestitem in zip(row, [longestid, longestname, longestvtype, longestscope, 5]):
                buffer += " " + column + " " * (longestitem - len(column) + 1) + "|"
        buffer += "\n+"
        for longestitem in [longestid, longestname, longestvtype, longestscope, 5]:
            buffer += "-" * (longestitem + 2) + "+"
        return buffer

class Type:
    INT8 = 0
    INT16 = 1
    INT32 = 2
    INT64 = 3
    INT128 = 4
    INTARCH = 5

    UINT8 = 6
    UINT16 = 7
    UINT32 = 8
    UINT64 = 9
    UINT128 = 10
    UINTARCH = 11

    FLOAT32 = 12
    FLOAT64 = 13

    CHAR = 14
    STRING = 15

    BOOL = 16

    FUNCTION = 17
    POINTER = 18

    ARRAY = 19
    STRUCT = 20

    SCOPE = 21

    def __init__(self, ty: int, resulttype = None, arraytype = None, size: int = 0, identity = None, values: list = []):
        self.ty = ty

        if self.ty == Type.ARRAY:
            if arraytype == None:
                raise Exception("Type.ARRAY(arraytype) must not be None!")
            self.arraytype = arraytype
            self.arraylength = size
        elif self.ty == Type.STRUCT:
            self.structidentity = identity
            self.structvalues = values
        elif self.ty == Type.FUNCTION or self.ty == Type.POINTER:
            if resulttype == None:
                raise Exception()
            self.resulttype = resulttype

    def fromstring(string: str):
        VALID_TYPES = [ "Int8",  "Int16",  "Int32",  "Int64",   "Int128",  "IntArch",
                        "UInt8", "UInt16", "UInt32", "UInt64",  "UInt128", "UIntArch",
                        "Float32", "Float64", "Char", "String", "Bool" ]
        if string in VALID_TYPES:
            return Type(VALID_TYPES.index(string))
        else:
            logging.error("Invalid type: %s", string)
            quit()
    
    def __str__(self) -> str:
        if self.ty == Type.INT8: return "Int8"
        elif self.ty == Type.INT16: return "Int16"
        elif self.ty == Type.INT32: return "Int32"
        elif self.ty == Type.INT64: return "Int64"
        elif self.ty == Type.INT128: return "Int128"
        elif self.ty == Type.INTARCH: return "IntArch"
        elif self.ty == Type.UINT8: return "UInt8"
        elif self.ty == Type.UINT16: return "UInt16"
        elif self.ty == Type.UINT32: return "UInt32"
        elif self.ty == Type.UINT64: return "UInt64"
        elif self.ty == Type.UINT128: return "UInt128"
        elif self.ty == Type.UINTARCH: return "UIntArch"
        elif self.ty == Type.FLOAT32: return "Float32"
        elif self.ty == Type.FLOAT64: return "Float64"
        elif self.ty == Type.CHAR: return "Char"
        elif self.ty == Type.STRING: return "String"
        elif self.ty == Type.BOOL: return "Bool"
        elif self.ty == Type.FUNCTION: return "Function"
        elif self.ty == Type.POINTER: return "Pointer"
        elif self.ty == Type.ARRAY: return f"{str(self.arraytype)}[{self.arraylength}]"
        elif self.ty == Type.STRUCT:
            buffer = ""
            if self.structidentity == None:
                buffer = "struct <anonymous> { VALUES }"
            else:
                buffer = f"struct {str(self.structidentity)} {{VALUES}}"
            return buffer.replace("VALUES", ", ".join([f"{i.name}: {i.vtype}" for i in self.structvalues]))
        elif self.ty == Type.SCOPE: return "Scope"
        else: return "INVALID"

class Symbol:
    def __init__(self, name: str, vtype: Type, scope, ispublic: bool = False):
        self.name = name
        self.vtype = vtype
        self.scope = scope
        self._id = SymbolTable.push(self)
        self.ispublic = ispublic

        if self.name == "":
            self.name = f"<anonymous {self._id}>"
        
        self.children = []
        if scope != None:
            scope.appendchild(self)
    
    def appendchild(self, child):
        self.children.append(child)
    
    def __str__(self) -> str:
        if self.scope == None:
            return self.name
        else:
            return str(self.scope) + "::" + self.name