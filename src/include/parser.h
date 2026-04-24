#ifndef PARSER_H
#define PARSER_H

#include "da.h"
#include "tokenizer.h"
#include <stddef.h>

typedef enum {
    Ast_ParseError,

    Ast_Type_Prim,
    Ast_Type_Array,
    Ast_Type_Pointer,
    Ast_Type_Ident,
    Ast_Type_Function,

    Ast_Decl_Struct,
    Ast_Decl_Enum,
    Ast_Decl_Function,
    Ast_Decl_Typedef,

    Ast_Stmt_Import,
    Ast_Stmt_Return,
    Ast_Stmt_VarDecl,
    Ast_Stmt_If,
    Ast_Stmt_While,
    Ast_Stmt_Break,
    Ast_Stmt_Continue,
    Ast_Stmt_Asm,
    Ast_Stmt_Block,

    Ast_Expr_Unary,
    Ast_Expr_Binary,
    Ast_Expr_FunctionCall,
    Ast_Expr_ArrayAccess,
    Ast_Expr_MemberAccess,
    Ast_Expr_StructInitializer,
    Ast_Expr_TypeCast,
    Ast_Expr_Integer,
    Ast_Expr_String,
    Ast_Expr_Char,
    Ast_Expr_Identifier,
    Ast_Expr_Group,
} AstVariant;

typedef struct Ast Ast;

typedef struct {
    char *name;
    Ast *type;
} StringAstTuple;
DECLARE_DYNAMIC_ARRAY(StringAstTuple, StringAstTupleArray);

DECLARE_DYNAMIC_ARRAY(char *, StringArray);

DECLARE_DYNAMIC_ARRAY(Ast, AstArray);

struct Ast {
    AstVariant variant;
    union {
        // Ast_Type
        char *type_prim;
        struct {
            Ast *size;
            Ast *kind;
        } type_array;
        struct {
            Ast *kind;
        } type_pointer;
        char *type_ident;
        struct {
            Ast *ret;
            AstArray *params;
        } type_function;

        // Ast_Decl
        struct {
            char *name;
            StringAstTupleArray fields;
        } decl_struct;
        struct {
            char *name;
            StringArray modifiers;
            StringArray variants;
        } decl_enum;
        struct {
            char *name;
            Ast *return_type;
            StringAstTupleArray params;
        } decl_function;
        struct {
            char *name;
            Ast *type;
        } decl_typedef;

        // Ast_stmt
        char *stmt_import;
        struct {
            int is_some;
            Ast *type;
        } stmt_return;
        struct {
            int is_static;
            char *name;
            struct {
                int is_some;
                Ast *type;
            } kind;
            struct {
                int is_some;
                Ast *type;
            } value;
        } stmt_var_decl;
        struct {
            Ast *condition;
            Ast *body;
            struct {
                int is_some;
                Ast *type;
            } alt;
        } stmt_if;
        struct {
            Ast *condition;
            Ast *body;
        } stmt_while;
        char *stmt_asm;
        AstArray stmt_block;

        struct {
            Ast *child;
            Token *op;
        } expr_unary;
        struct {
            Ast *lhs;
            Ast *rhs;
            Token *op;
        } expr_binary;
        struct {
            Ast *designator;
            AstArray args;
        } expr_function_call;
        struct {
            Ast *lhs;
            Ast *index;
        } expr_array_access;
        struct {
            Ast *lhs;
            char *member;
        } expr_member_access;
        struct {
            char *name;
            StringAstTupleArray values;
        } expr_struct_intializer;
        struct {
            Ast *to;
            Ast *value;
        } expr_typecast;
        struct {
            char *value;
            char *type;
        } expr_integer;
        char *expr_string;
        char expr_char;
        char *expr_identifier;
        Ast *group;
    } value;
};

typedef struct Parser {
    size_t index;
    Tokens *tokens;
} Parser;

Ast *parse_tokens(Tokens *tokens);

#endif
