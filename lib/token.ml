type kind =
    | Continue
    | Function
    | TypeDef
    | Import
    | Return
    | Static
    | Struct
    | Break
    | Defer
    | While
    | Cast
    | Else
    | Enum
    | Spec
    | Asm
    | For
    | Let
    | If
    | Ident of string
    | StringLit of string
    | Integer of string * string (* (value, suffix_type) *)
    | CharLit of char
    | PrimType of string
    | FuncType
    | LBrace
    | RBrace
    | LParen
    | RParen
    | LBracket
    | RBracket
    | Colon
    | Semi
    | Comma
    | Equals
    | LogOr
    | LogAnd
    | LogNot
    | DoubleEquals
    | NotEquals
    | LessThan
    | MoreThan
    | LessThanEq
    | MoreThanEq
    | BitOr
    | BitXor
    | BitNeg
    | LShift
    | RShift
    | Plus
    | Minus
    | Star
    | Slash
    | Percent
    | Ampersand
    | At
    | Dot
    | Arrow

(*                             (line, col)                *)
type meta = { file_id : int; pos : int * int; value : kind }
