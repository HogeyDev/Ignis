#include "tree_sitter/parser.h"

#if defined(__GNUC__) || defined(__clang__)
#pragma GCC diagnostic ignored "-Wmissing-field-initializers"
#endif

#define LANGUAGE_VERSION 14
#define STATE_COUNT 53
#define LARGE_STATE_COUNT 2
#define SYMBOL_COUNT 37
#define ALIAS_COUNT 0
#define TOKEN_COUNT 21
#define EXTERNAL_TOKEN_COUNT 0
#define FIELD_COUNT 0
#define MAX_ALIAS_SEQUENCE_LENGTH 5
#define PRODUCTION_ID_COUNT 1

enum ts_symbol_identifiers {
  anon_sym_SEMI = 1,
  anon_sym_return = 2,
  anon_sym_import = 3,
  anon_sym_DOT = 4,
  anon_sym_func = 5,
  anon_sym_COLON_COLON = 6,
  anon_sym_LBRACE = 7,
  anon_sym_RBRACE = 8,
  anon_sym_LPAREN = 9,
  anon_sym_COMMA = 10,
  anon_sym_COLON = 11,
  anon_sym_RPAREN = 12,
  anon_sym_LBRACK = 13,
  anon_sym_RBRACK = 14,
  anon_sym_AT = 15,
  anon_sym_void = 16,
  anon_sym_int = 17,
  anon_sym_char = 18,
  sym_identifier = 19,
  sym_number = 20,
  sym_source_file = 21,
  sym_statement = 22,
  sym_function_call = 23,
  sym_return_statement = 24,
  sym_import_statement = 25,
  sym_function_definition = 26,
  sym_block = 27,
  sym_parameter_list = 28,
  sym_argument_list = 29,
  sym_type = 30,
  sym_primative_type = 31,
  sym_expression = 32,
  aux_sym_source_file_repeat1 = 33,
  aux_sym_import_statement_repeat1 = 34,
  aux_sym_parameter_list_repeat1 = 35,
  aux_sym_argument_list_repeat1 = 36,
};

static const char * const ts_symbol_names[] = {
  [ts_builtin_sym_end] = "end",
  [anon_sym_SEMI] = ";",
  [anon_sym_return] = "return",
  [anon_sym_import] = "import",
  [anon_sym_DOT] = ".",
  [anon_sym_func] = "func",
  [anon_sym_COLON_COLON] = "::",
  [anon_sym_LBRACE] = "{",
  [anon_sym_RBRACE] = "}",
  [anon_sym_LPAREN] = "(",
  [anon_sym_COMMA] = ",",
  [anon_sym_COLON] = ":",
  [anon_sym_RPAREN] = ")",
  [anon_sym_LBRACK] = "[",
  [anon_sym_RBRACK] = "]",
  [anon_sym_AT] = "@",
  [anon_sym_void] = "void",
  [anon_sym_int] = "int",
  [anon_sym_char] = "char",
  [sym_identifier] = "identifier",
  [sym_number] = "number",
  [sym_source_file] = "source_file",
  [sym_statement] = "statement",
  [sym_function_call] = "function_call",
  [sym_return_statement] = "return_statement",
  [sym_import_statement] = "import_statement",
  [sym_function_definition] = "function_definition",
  [sym_block] = "block",
  [sym_parameter_list] = "parameter_list",
  [sym_argument_list] = "argument_list",
  [sym_type] = "type",
  [sym_primative_type] = "primative_type",
  [sym_expression] = "expression",
  [aux_sym_source_file_repeat1] = "source_file_repeat1",
  [aux_sym_import_statement_repeat1] = "import_statement_repeat1",
  [aux_sym_parameter_list_repeat1] = "parameter_list_repeat1",
  [aux_sym_argument_list_repeat1] = "argument_list_repeat1",
};

static const TSSymbol ts_symbol_map[] = {
  [ts_builtin_sym_end] = ts_builtin_sym_end,
  [anon_sym_SEMI] = anon_sym_SEMI,
  [anon_sym_return] = anon_sym_return,
  [anon_sym_import] = anon_sym_import,
  [anon_sym_DOT] = anon_sym_DOT,
  [anon_sym_func] = anon_sym_func,
  [anon_sym_COLON_COLON] = anon_sym_COLON_COLON,
  [anon_sym_LBRACE] = anon_sym_LBRACE,
  [anon_sym_RBRACE] = anon_sym_RBRACE,
  [anon_sym_LPAREN] = anon_sym_LPAREN,
  [anon_sym_COMMA] = anon_sym_COMMA,
  [anon_sym_COLON] = anon_sym_COLON,
  [anon_sym_RPAREN] = anon_sym_RPAREN,
  [anon_sym_LBRACK] = anon_sym_LBRACK,
  [anon_sym_RBRACK] = anon_sym_RBRACK,
  [anon_sym_AT] = anon_sym_AT,
  [anon_sym_void] = anon_sym_void,
  [anon_sym_int] = anon_sym_int,
  [anon_sym_char] = anon_sym_char,
  [sym_identifier] = sym_identifier,
  [sym_number] = sym_number,
  [sym_source_file] = sym_source_file,
  [sym_statement] = sym_statement,
  [sym_function_call] = sym_function_call,
  [sym_return_statement] = sym_return_statement,
  [sym_import_statement] = sym_import_statement,
  [sym_function_definition] = sym_function_definition,
  [sym_block] = sym_block,
  [sym_parameter_list] = sym_parameter_list,
  [sym_argument_list] = sym_argument_list,
  [sym_type] = sym_type,
  [sym_primative_type] = sym_primative_type,
  [sym_expression] = sym_expression,
  [aux_sym_source_file_repeat1] = aux_sym_source_file_repeat1,
  [aux_sym_import_statement_repeat1] = aux_sym_import_statement_repeat1,
  [aux_sym_parameter_list_repeat1] = aux_sym_parameter_list_repeat1,
  [aux_sym_argument_list_repeat1] = aux_sym_argument_list_repeat1,
};

static const TSSymbolMetadata ts_symbol_metadata[] = {
  [ts_builtin_sym_end] = {
    .visible = false,
    .named = true,
  },
  [anon_sym_SEMI] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_return] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_import] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_DOT] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_func] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_COLON_COLON] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_LBRACE] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_RBRACE] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_LPAREN] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_COMMA] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_COLON] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_RPAREN] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_LBRACK] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_RBRACK] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_AT] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_void] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_int] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_char] = {
    .visible = true,
    .named = false,
  },
  [sym_identifier] = {
    .visible = true,
    .named = true,
  },
  [sym_number] = {
    .visible = true,
    .named = true,
  },
  [sym_source_file] = {
    .visible = true,
    .named = true,
  },
  [sym_statement] = {
    .visible = true,
    .named = true,
  },
  [sym_function_call] = {
    .visible = true,
    .named = true,
  },
  [sym_return_statement] = {
    .visible = true,
    .named = true,
  },
  [sym_import_statement] = {
    .visible = true,
    .named = true,
  },
  [sym_function_definition] = {
    .visible = true,
    .named = true,
  },
  [sym_block] = {
    .visible = true,
    .named = true,
  },
  [sym_parameter_list] = {
    .visible = true,
    .named = true,
  },
  [sym_argument_list] = {
    .visible = true,
    .named = true,
  },
  [sym_type] = {
    .visible = true,
    .named = true,
  },
  [sym_primative_type] = {
    .visible = true,
    .named = true,
  },
  [sym_expression] = {
    .visible = true,
    .named = true,
  },
  [aux_sym_source_file_repeat1] = {
    .visible = false,
    .named = false,
  },
  [aux_sym_import_statement_repeat1] = {
    .visible = false,
    .named = false,
  },
  [aux_sym_parameter_list_repeat1] = {
    .visible = false,
    .named = false,
  },
  [aux_sym_argument_list_repeat1] = {
    .visible = false,
    .named = false,
  },
};

static const TSSymbol ts_alias_sequences[PRODUCTION_ID_COUNT][MAX_ALIAS_SEQUENCE_LENGTH] = {
  [0] = {0},
};

static const uint16_t ts_non_terminal_alias_map[] = {
  0,
};

static const TSStateId ts_primary_state_ids[STATE_COUNT] = {
  [0] = 0,
  [1] = 1,
  [2] = 2,
  [3] = 3,
  [4] = 4,
  [5] = 5,
  [6] = 6,
  [7] = 7,
  [8] = 8,
  [9] = 9,
  [10] = 10,
  [11] = 11,
  [12] = 12,
  [13] = 13,
  [14] = 14,
  [15] = 15,
  [16] = 16,
  [17] = 17,
  [18] = 18,
  [19] = 19,
  [20] = 20,
  [21] = 21,
  [22] = 22,
  [23] = 23,
  [24] = 24,
  [25] = 25,
  [26] = 26,
  [27] = 27,
  [28] = 28,
  [29] = 29,
  [30] = 30,
  [31] = 31,
  [32] = 32,
  [33] = 33,
  [34] = 34,
  [35] = 35,
  [36] = 36,
  [37] = 37,
  [38] = 38,
  [39] = 39,
  [40] = 40,
  [41] = 41,
  [42] = 42,
  [43] = 43,
  [44] = 44,
  [45] = 45,
  [46] = 46,
  [47] = 47,
  [48] = 48,
  [49] = 49,
  [50] = 50,
  [51] = 51,
  [52] = 52,
};

static bool ts_lex(TSLexer *lexer, TSStateId state) {
  START_LEXER();
  eof = lexer->eof(lexer);
  switch (state) {
    case 0:
      if (eof) ADVANCE(5);
      ADVANCE_MAP(
        '(', 14,
        ')', 18,
        ',', 15,
        '.', 9,
        ':', 17,
        ';', 6,
        '@', 21,
        '[', 19,
        ']', 20,
        'c', 29,
        'f', 45,
        'i', 31,
        'r', 28,
        'v', 36,
        '{', 12,
        '}', 13,
      );
      if (('\t' <= lookahead && lookahead <= '\r') ||
          lookahead == ' ') SKIP(0);
      if (('0' <= lookahead && lookahead <= '9')) ADVANCE(48);
      if (('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(47);
      END_STATE();
    case 1:
      if (lookahead == ')') ADVANCE(18);
      if (lookahead == ';') ADVANCE(6);
      if (('\t' <= lookahead && lookahead <= '\r') ||
          lookahead == ' ') SKIP(1);
      if (('0' <= lookahead && lookahead <= '9')) ADVANCE(48);
      if (('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(47);
      END_STATE();
    case 2:
      if (lookahead == ':') ADVANCE(11);
      END_STATE();
    case 3:
      if (lookahead == ':') ADVANCE(16);
      if (lookahead == '@') ADVANCE(21);
      if (lookahead == '[') ADVANCE(19);
      if (lookahead == 'c') ADVANCE(29);
      if (lookahead == 'i') ADVANCE(33);
      if (lookahead == 'v') ADVANCE(36);
      if (('\t' <= lookahead && lookahead <= '\r') ||
          lookahead == ' ') SKIP(3);
      if (('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(47);
      END_STATE();
    case 4:
      if (eof) ADVANCE(5);
      if (lookahead == ':') ADVANCE(2);
      if (lookahead == ';') ADVANCE(6);
      if (lookahead == 'f') ADVANCE(45);
      if (lookahead == 'i') ADVANCE(32);
      if (lookahead == 'r') ADVANCE(28);
      if (lookahead == '{') ADVANCE(12);
      if (lookahead == '}') ADVANCE(13);
      if (('\t' <= lookahead && lookahead <= '\r') ||
          lookahead == ' ') SKIP(4);
      if (('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(47);
      END_STATE();
    case 5:
      ACCEPT_TOKEN(ts_builtin_sym_end);
      END_STATE();
    case 6:
      ACCEPT_TOKEN(anon_sym_SEMI);
      END_STATE();
    case 7:
      ACCEPT_TOKEN(anon_sym_return);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(47);
      END_STATE();
    case 8:
      ACCEPT_TOKEN(anon_sym_import);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(47);
      END_STATE();
    case 9:
      ACCEPT_TOKEN(anon_sym_DOT);
      END_STATE();
    case 10:
      ACCEPT_TOKEN(anon_sym_func);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(47);
      END_STATE();
    case 11:
      ACCEPT_TOKEN(anon_sym_COLON_COLON);
      END_STATE();
    case 12:
      ACCEPT_TOKEN(anon_sym_LBRACE);
      END_STATE();
    case 13:
      ACCEPT_TOKEN(anon_sym_RBRACE);
      END_STATE();
    case 14:
      ACCEPT_TOKEN(anon_sym_LPAREN);
      END_STATE();
    case 15:
      ACCEPT_TOKEN(anon_sym_COMMA);
      END_STATE();
    case 16:
      ACCEPT_TOKEN(anon_sym_COLON);
      END_STATE();
    case 17:
      ACCEPT_TOKEN(anon_sym_COLON);
      if (lookahead == ':') ADVANCE(11);
      END_STATE();
    case 18:
      ACCEPT_TOKEN(anon_sym_RPAREN);
      END_STATE();
    case 19:
      ACCEPT_TOKEN(anon_sym_LBRACK);
      END_STATE();
    case 20:
      ACCEPT_TOKEN(anon_sym_RBRACK);
      END_STATE();
    case 21:
      ACCEPT_TOKEN(anon_sym_AT);
      END_STATE();
    case 22:
      ACCEPT_TOKEN(anon_sym_void);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(47);
      END_STATE();
    case 23:
      ACCEPT_TOKEN(anon_sym_int);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(47);
      END_STATE();
    case 24:
      ACCEPT_TOKEN(anon_sym_char);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(47);
      END_STATE();
    case 25:
      ACCEPT_TOKEN(sym_identifier);
      if (lookahead == 'a') ADVANCE(39);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('b' <= lookahead && lookahead <= 'z')) ADVANCE(47);
      END_STATE();
    case 26:
      ACCEPT_TOKEN(sym_identifier);
      if (lookahead == 'c') ADVANCE(10);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(47);
      END_STATE();
    case 27:
      ACCEPT_TOKEN(sym_identifier);
      if (lookahead == 'd') ADVANCE(22);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(47);
      END_STATE();
    case 28:
      ACCEPT_TOKEN(sym_identifier);
      if (lookahead == 'e') ADVANCE(44);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(47);
      END_STATE();
    case 29:
      ACCEPT_TOKEN(sym_identifier);
      if (lookahead == 'h') ADVANCE(25);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(47);
      END_STATE();
    case 30:
      ACCEPT_TOKEN(sym_identifier);
      if (lookahead == 'i') ADVANCE(27);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(47);
      END_STATE();
    case 31:
      ACCEPT_TOKEN(sym_identifier);
      if (lookahead == 'm') ADVANCE(38);
      if (lookahead == 'n') ADVANCE(42);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(47);
      END_STATE();
    case 32:
      ACCEPT_TOKEN(sym_identifier);
      if (lookahead == 'm') ADVANCE(38);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(47);
      END_STATE();
    case 33:
      ACCEPT_TOKEN(sym_identifier);
      if (lookahead == 'n') ADVANCE(42);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(47);
      END_STATE();
    case 34:
      ACCEPT_TOKEN(sym_identifier);
      if (lookahead == 'n') ADVANCE(26);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(47);
      END_STATE();
    case 35:
      ACCEPT_TOKEN(sym_identifier);
      if (lookahead == 'n') ADVANCE(7);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(47);
      END_STATE();
    case 36:
      ACCEPT_TOKEN(sym_identifier);
      if (lookahead == 'o') ADVANCE(30);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(47);
      END_STATE();
    case 37:
      ACCEPT_TOKEN(sym_identifier);
      if (lookahead == 'o') ADVANCE(41);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(47);
      END_STATE();
    case 38:
      ACCEPT_TOKEN(sym_identifier);
      if (lookahead == 'p') ADVANCE(37);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(47);
      END_STATE();
    case 39:
      ACCEPT_TOKEN(sym_identifier);
      if (lookahead == 'r') ADVANCE(24);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(47);
      END_STATE();
    case 40:
      ACCEPT_TOKEN(sym_identifier);
      if (lookahead == 'r') ADVANCE(35);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(47);
      END_STATE();
    case 41:
      ACCEPT_TOKEN(sym_identifier);
      if (lookahead == 'r') ADVANCE(43);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(47);
      END_STATE();
    case 42:
      ACCEPT_TOKEN(sym_identifier);
      if (lookahead == 't') ADVANCE(23);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(47);
      END_STATE();
    case 43:
      ACCEPT_TOKEN(sym_identifier);
      if (lookahead == 't') ADVANCE(8);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(47);
      END_STATE();
    case 44:
      ACCEPT_TOKEN(sym_identifier);
      if (lookahead == 't') ADVANCE(46);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(47);
      END_STATE();
    case 45:
      ACCEPT_TOKEN(sym_identifier);
      if (lookahead == 'u') ADVANCE(34);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(47);
      END_STATE();
    case 46:
      ACCEPT_TOKEN(sym_identifier);
      if (lookahead == 'u') ADVANCE(40);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(47);
      END_STATE();
    case 47:
      ACCEPT_TOKEN(sym_identifier);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(47);
      END_STATE();
    case 48:
      ACCEPT_TOKEN(sym_number);
      if (('0' <= lookahead && lookahead <= '9')) ADVANCE(48);
      END_STATE();
    default:
      return false;
  }
}

static const TSLexMode ts_lex_modes[STATE_COUNT] = {
  [0] = {.lex_state = 0},
  [1] = {.lex_state = 4},
  [2] = {.lex_state = 4},
  [3] = {.lex_state = 4},
  [4] = {.lex_state = 4},
  [5] = {.lex_state = 4},
  [6] = {.lex_state = 4},
  [7] = {.lex_state = 3},
  [8] = {.lex_state = 3},
  [9] = {.lex_state = 3},
  [10] = {.lex_state = 4},
  [11] = {.lex_state = 4},
  [12] = {.lex_state = 1},
  [13] = {.lex_state = 1},
  [14] = {.lex_state = 4},
  [15] = {.lex_state = 1},
  [16] = {.lex_state = 0},
  [17] = {.lex_state = 0},
  [18] = {.lex_state = 1},
  [19] = {.lex_state = 0},
  [20] = {.lex_state = 1},
  [21] = {.lex_state = 0},
  [22] = {.lex_state = 0},
  [23] = {.lex_state = 0},
  [24] = {.lex_state = 0},
  [25] = {.lex_state = 0},
  [26] = {.lex_state = 0},
  [27] = {.lex_state = 0},
  [28] = {.lex_state = 0},
  [29] = {.lex_state = 0},
  [30] = {.lex_state = 0},
  [31] = {.lex_state = 0},
  [32] = {.lex_state = 0},
  [33] = {.lex_state = 0},
  [34] = {.lex_state = 0},
  [35] = {.lex_state = 0},
  [36] = {.lex_state = 1},
  [37] = {.lex_state = 0},
  [38] = {.lex_state = 1},
  [39] = {.lex_state = 1},
  [40] = {.lex_state = 0},
  [41] = {.lex_state = 4},
  [42] = {.lex_state = 0},
  [43] = {.lex_state = 0},
  [44] = {.lex_state = 0},
  [45] = {.lex_state = 0},
  [46] = {.lex_state = 3},
  [47] = {.lex_state = 0},
  [48] = {.lex_state = 0},
  [49] = {.lex_state = 0},
  [50] = {.lex_state = 0},
  [51] = {.lex_state = 1},
  [52] = {.lex_state = 0},
};

static const uint16_t ts_parse_table[LARGE_STATE_COUNT][SYMBOL_COUNT] = {
  [0] = {
    [ts_builtin_sym_end] = ACTIONS(1),
    [anon_sym_SEMI] = ACTIONS(1),
    [anon_sym_return] = ACTIONS(1),
    [anon_sym_import] = ACTIONS(1),
    [anon_sym_DOT] = ACTIONS(1),
    [anon_sym_func] = ACTIONS(1),
    [anon_sym_COLON_COLON] = ACTIONS(1),
    [anon_sym_LBRACE] = ACTIONS(1),
    [anon_sym_RBRACE] = ACTIONS(1),
    [anon_sym_LPAREN] = ACTIONS(1),
    [anon_sym_COMMA] = ACTIONS(1),
    [anon_sym_COLON] = ACTIONS(1),
    [anon_sym_RPAREN] = ACTIONS(1),
    [anon_sym_LBRACK] = ACTIONS(1),
    [anon_sym_RBRACK] = ACTIONS(1),
    [anon_sym_AT] = ACTIONS(1),
    [anon_sym_void] = ACTIONS(1),
    [anon_sym_int] = ACTIONS(1),
    [anon_sym_char] = ACTIONS(1),
    [sym_identifier] = ACTIONS(1),
    [sym_number] = ACTIONS(1),
  },
  [1] = {
    [sym_source_file] = STATE(47),
    [sym_statement] = STATE(5),
    [sym_function_call] = STATE(45),
    [sym_return_statement] = STATE(45),
    [sym_import_statement] = STATE(45),
    [sym_function_definition] = STATE(45),
    [aux_sym_source_file_repeat1] = STATE(5),
    [ts_builtin_sym_end] = ACTIONS(3),
    [anon_sym_return] = ACTIONS(5),
    [anon_sym_import] = ACTIONS(7),
    [anon_sym_func] = ACTIONS(9),
    [sym_identifier] = ACTIONS(11),
  },
};

static const uint16_t ts_small_parse_table[] = {
  [0] = 7,
    ACTIONS(15), 1,
      anon_sym_return,
    ACTIONS(18), 1,
      anon_sym_import,
    ACTIONS(21), 1,
      anon_sym_func,
    ACTIONS(24), 1,
      sym_identifier,
    ACTIONS(13), 2,
      ts_builtin_sym_end,
      anon_sym_RBRACE,
    STATE(2), 2,
      sym_statement,
      aux_sym_source_file_repeat1,
    STATE(45), 4,
      sym_function_call,
      sym_return_statement,
      sym_import_statement,
      sym_function_definition,
  [27] = 8,
    ACTIONS(5), 1,
      anon_sym_return,
    ACTIONS(7), 1,
      anon_sym_import,
    ACTIONS(9), 1,
      anon_sym_func,
    ACTIONS(11), 1,
      sym_identifier,
    ACTIONS(27), 1,
      anon_sym_LBRACE,
    STATE(48), 1,
      sym_block,
    STATE(50), 1,
      sym_statement,
    STATE(45), 4,
      sym_function_call,
      sym_return_statement,
      sym_import_statement,
      sym_function_definition,
  [55] = 7,
    ACTIONS(5), 1,
      anon_sym_return,
    ACTIONS(7), 1,
      anon_sym_import,
    ACTIONS(9), 1,
      anon_sym_func,
    ACTIONS(11), 1,
      sym_identifier,
    ACTIONS(29), 1,
      anon_sym_RBRACE,
    STATE(2), 2,
      sym_statement,
      aux_sym_source_file_repeat1,
    STATE(45), 4,
      sym_function_call,
      sym_return_statement,
      sym_import_statement,
      sym_function_definition,
  [81] = 7,
    ACTIONS(5), 1,
      anon_sym_return,
    ACTIONS(7), 1,
      anon_sym_import,
    ACTIONS(9), 1,
      anon_sym_func,
    ACTIONS(11), 1,
      sym_identifier,
    ACTIONS(31), 1,
      ts_builtin_sym_end,
    STATE(2), 2,
      sym_statement,
      aux_sym_source_file_repeat1,
    STATE(45), 4,
      sym_function_call,
      sym_return_statement,
      sym_import_statement,
      sym_function_definition,
  [107] = 7,
    ACTIONS(5), 1,
      anon_sym_return,
    ACTIONS(7), 1,
      anon_sym_import,
    ACTIONS(9), 1,
      anon_sym_func,
    ACTIONS(11), 1,
      sym_identifier,
    ACTIONS(33), 1,
      anon_sym_RBRACE,
    STATE(4), 2,
      sym_statement,
      aux_sym_source_file_repeat1,
    STATE(45), 4,
      sym_function_call,
      sym_return_statement,
      sym_import_statement,
      sym_function_definition,
  [133] = 6,
    ACTIONS(35), 1,
      anon_sym_LBRACK,
    ACTIONS(37), 1,
      anon_sym_AT,
    ACTIONS(41), 1,
      sym_identifier,
    STATE(29), 1,
      sym_type,
    STATE(31), 1,
      sym_primative_type,
    ACTIONS(39), 3,
      anon_sym_void,
      anon_sym_int,
      anon_sym_char,
  [154] = 6,
    ACTIONS(35), 1,
      anon_sym_LBRACK,
    ACTIONS(37), 1,
      anon_sym_AT,
    ACTIONS(41), 1,
      sym_identifier,
    STATE(21), 1,
      sym_type,
    STATE(31), 1,
      sym_primative_type,
    ACTIONS(39), 3,
      anon_sym_void,
      anon_sym_int,
      anon_sym_char,
  [175] = 6,
    ACTIONS(35), 1,
      anon_sym_LBRACK,
    ACTIONS(37), 1,
      anon_sym_AT,
    ACTIONS(41), 1,
      sym_identifier,
    STATE(26), 1,
      sym_type,
    STATE(31), 1,
      sym_primative_type,
    ACTIONS(39), 3,
      anon_sym_void,
      anon_sym_int,
      anon_sym_char,
  [196] = 2,
    ACTIONS(43), 3,
      ts_builtin_sym_end,
      anon_sym_SEMI,
      anon_sym_RBRACE,
    ACTIONS(45), 4,
      anon_sym_return,
      anon_sym_import,
      anon_sym_func,
      sym_identifier,
  [208] = 2,
    ACTIONS(49), 1,
      anon_sym_LBRACE,
    ACTIONS(47), 4,
      anon_sym_return,
      anon_sym_import,
      anon_sym_func,
      sym_identifier,
  [218] = 3,
    ACTIONS(51), 1,
      anon_sym_RPAREN,
    ACTIONS(53), 2,
      sym_identifier,
      sym_number,
    STATE(12), 2,
      sym_expression,
      aux_sym_argument_list_repeat1,
  [230] = 3,
    ACTIONS(56), 1,
      anon_sym_RPAREN,
    ACTIONS(58), 2,
      sym_identifier,
      sym_number,
    STATE(12), 2,
      sym_expression,
      aux_sym_argument_list_repeat1,
  [242] = 2,
    ACTIONS(62), 1,
      anon_sym_LBRACE,
    ACTIONS(60), 4,
      anon_sym_return,
      anon_sym_import,
      anon_sym_func,
      sym_identifier,
  [252] = 3,
    ACTIONS(64), 1,
      anon_sym_RPAREN,
    ACTIONS(58), 2,
      sym_identifier,
      sym_number,
    STATE(13), 2,
      sym_expression,
      aux_sym_argument_list_repeat1,
  [264] = 2,
    STATE(33), 1,
      sym_primative_type,
    ACTIONS(66), 3,
      anon_sym_void,
      anon_sym_int,
      anon_sym_char,
  [273] = 2,
    STATE(35), 1,
      sym_primative_type,
    ACTIONS(66), 3,
      anon_sym_void,
      anon_sym_int,
      anon_sym_char,
  [282] = 1,
    ACTIONS(68), 4,
      anon_sym_SEMI,
      anon_sym_RPAREN,
      sym_identifier,
      sym_number,
  [289] = 3,
    ACTIONS(70), 1,
      anon_sym_SEMI,
    ACTIONS(72), 1,
      anon_sym_DOT,
    STATE(24), 1,
      aux_sym_import_statement_repeat1,
  [299] = 2,
    STATE(43), 1,
      sym_expression,
    ACTIONS(58), 2,
      sym_identifier,
      sym_number,
  [307] = 3,
    ACTIONS(74), 1,
      anon_sym_COMMA,
    ACTIONS(76), 1,
      anon_sym_RPAREN,
    STATE(23), 1,
      aux_sym_parameter_list_repeat1,
  [317] = 3,
    ACTIONS(72), 1,
      anon_sym_DOT,
    ACTIONS(78), 1,
      anon_sym_SEMI,
    STATE(19), 1,
      aux_sym_import_statement_repeat1,
  [327] = 3,
    ACTIONS(74), 1,
      anon_sym_COMMA,
    ACTIONS(80), 1,
      anon_sym_RPAREN,
    STATE(25), 1,
      aux_sym_parameter_list_repeat1,
  [337] = 3,
    ACTIONS(82), 1,
      anon_sym_SEMI,
    ACTIONS(84), 1,
      anon_sym_DOT,
    STATE(24), 1,
      aux_sym_import_statement_repeat1,
  [347] = 3,
    ACTIONS(87), 1,
      anon_sym_COMMA,
    ACTIONS(90), 1,
      anon_sym_RPAREN,
    STATE(25), 1,
      aux_sym_parameter_list_repeat1,
  [357] = 1,
    ACTIONS(92), 2,
      anon_sym_COMMA,
      anon_sym_RPAREN,
  [362] = 2,
    ACTIONS(94), 1,
      anon_sym_LPAREN,
    STATE(3), 1,
      sym_parameter_list,
  [369] = 2,
    ACTIONS(96), 1,
      anon_sym_RBRACK,
    ACTIONS(98), 1,
      sym_number,
  [376] = 1,
    ACTIONS(100), 2,
      anon_sym_COMMA,
      anon_sym_RPAREN,
  [381] = 1,
    ACTIONS(102), 2,
      anon_sym_COMMA,
      anon_sym_RPAREN,
  [386] = 1,
    ACTIONS(104), 2,
      anon_sym_COMMA,
      anon_sym_RPAREN,
  [391] = 1,
    ACTIONS(82), 2,
      anon_sym_SEMI,
      anon_sym_DOT,
  [396] = 1,
    ACTIONS(106), 2,
      anon_sym_COMMA,
      anon_sym_RPAREN,
  [401] = 2,
    ACTIONS(108), 1,
      anon_sym_LPAREN,
    STATE(40), 1,
      sym_argument_list,
  [408] = 1,
    ACTIONS(110), 2,
      anon_sym_COMMA,
      anon_sym_RPAREN,
  [413] = 1,
    ACTIONS(112), 1,
      sym_identifier,
  [417] = 1,
    ACTIONS(114), 1,
      anon_sym_RBRACK,
  [421] = 1,
    ACTIONS(116), 1,
      sym_identifier,
  [425] = 1,
    ACTIONS(118), 1,
      sym_identifier,
  [429] = 1,
    ACTIONS(120), 1,
      anon_sym_SEMI,
  [433] = 1,
    ACTIONS(122), 1,
      anon_sym_COLON_COLON,
  [437] = 1,
    ACTIONS(124), 1,
      anon_sym_SEMI,
  [441] = 1,
    ACTIONS(126), 1,
      anon_sym_SEMI,
  [445] = 1,
    ACTIONS(128), 1,
      anon_sym_SEMI,
  [449] = 1,
    ACTIONS(130), 1,
      anon_sym_SEMI,
  [453] = 1,
    ACTIONS(132), 1,
      anon_sym_COLON,
  [457] = 1,
    ACTIONS(134), 1,
      ts_builtin_sym_end,
  [461] = 1,
    ACTIONS(136), 1,
      anon_sym_SEMI,
  [465] = 1,
    ACTIONS(138), 1,
      anon_sym_SEMI,
  [469] = 1,
    ACTIONS(140), 1,
      anon_sym_SEMI,
  [473] = 1,
    ACTIONS(142), 1,
      sym_identifier,
  [477] = 1,
    ACTIONS(144), 1,
      anon_sym_SEMI,
};

static const uint32_t ts_small_parse_table_map[] = {
  [SMALL_STATE(2)] = 0,
  [SMALL_STATE(3)] = 27,
  [SMALL_STATE(4)] = 55,
  [SMALL_STATE(5)] = 81,
  [SMALL_STATE(6)] = 107,
  [SMALL_STATE(7)] = 133,
  [SMALL_STATE(8)] = 154,
  [SMALL_STATE(9)] = 175,
  [SMALL_STATE(10)] = 196,
  [SMALL_STATE(11)] = 208,
  [SMALL_STATE(12)] = 218,
  [SMALL_STATE(13)] = 230,
  [SMALL_STATE(14)] = 242,
  [SMALL_STATE(15)] = 252,
  [SMALL_STATE(16)] = 264,
  [SMALL_STATE(17)] = 273,
  [SMALL_STATE(18)] = 282,
  [SMALL_STATE(19)] = 289,
  [SMALL_STATE(20)] = 299,
  [SMALL_STATE(21)] = 307,
  [SMALL_STATE(22)] = 317,
  [SMALL_STATE(23)] = 327,
  [SMALL_STATE(24)] = 337,
  [SMALL_STATE(25)] = 347,
  [SMALL_STATE(26)] = 357,
  [SMALL_STATE(27)] = 362,
  [SMALL_STATE(28)] = 369,
  [SMALL_STATE(29)] = 376,
  [SMALL_STATE(30)] = 381,
  [SMALL_STATE(31)] = 386,
  [SMALL_STATE(32)] = 391,
  [SMALL_STATE(33)] = 396,
  [SMALL_STATE(34)] = 401,
  [SMALL_STATE(35)] = 408,
  [SMALL_STATE(36)] = 413,
  [SMALL_STATE(37)] = 417,
  [SMALL_STATE(38)] = 421,
  [SMALL_STATE(39)] = 425,
  [SMALL_STATE(40)] = 429,
  [SMALL_STATE(41)] = 433,
  [SMALL_STATE(42)] = 437,
  [SMALL_STATE(43)] = 441,
  [SMALL_STATE(44)] = 445,
  [SMALL_STATE(45)] = 449,
  [SMALL_STATE(46)] = 453,
  [SMALL_STATE(47)] = 457,
  [SMALL_STATE(48)] = 461,
  [SMALL_STATE(49)] = 465,
  [SMALL_STATE(50)] = 469,
  [SMALL_STATE(51)] = 473,
  [SMALL_STATE(52)] = 477,
};

static const TSParseActionEntry ts_parse_actions[] = {
  [0] = {.entry = {.count = 0, .reusable = false}},
  [1] = {.entry = {.count = 1, .reusable = false}}, RECOVER(),
  [3] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_source_file, 0, 0, 0),
  [5] = {.entry = {.count = 1, .reusable = false}}, SHIFT(20),
  [7] = {.entry = {.count = 1, .reusable = false}}, SHIFT(39),
  [9] = {.entry = {.count = 1, .reusable = false}}, SHIFT(51),
  [11] = {.entry = {.count = 1, .reusable = false}}, SHIFT(34),
  [13] = {.entry = {.count = 1, .reusable = true}}, REDUCE(aux_sym_source_file_repeat1, 2, 0, 0),
  [15] = {.entry = {.count = 2, .reusable = false}}, REDUCE(aux_sym_source_file_repeat1, 2, 0, 0), SHIFT_REPEAT(20),
  [18] = {.entry = {.count = 2, .reusable = false}}, REDUCE(aux_sym_source_file_repeat1, 2, 0, 0), SHIFT_REPEAT(39),
  [21] = {.entry = {.count = 2, .reusable = false}}, REDUCE(aux_sym_source_file_repeat1, 2, 0, 0), SHIFT_REPEAT(51),
  [24] = {.entry = {.count = 2, .reusable = false}}, REDUCE(aux_sym_source_file_repeat1, 2, 0, 0), SHIFT_REPEAT(34),
  [27] = {.entry = {.count = 1, .reusable = true}}, SHIFT(6),
  [29] = {.entry = {.count = 1, .reusable = true}}, SHIFT(49),
  [31] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_source_file, 1, 0, 0),
  [33] = {.entry = {.count = 1, .reusable = true}}, SHIFT(42),
  [35] = {.entry = {.count = 1, .reusable = true}}, SHIFT(28),
  [37] = {.entry = {.count = 1, .reusable = true}}, SHIFT(9),
  [39] = {.entry = {.count = 1, .reusable = false}}, SHIFT(30),
  [41] = {.entry = {.count = 1, .reusable = false}}, SHIFT(31),
  [43] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_statement, 2, 0, 0),
  [45] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_statement, 2, 0, 0),
  [47] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_parameter_list, 4, 0, 0),
  [49] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_parameter_list, 4, 0, 0),
  [51] = {.entry = {.count = 1, .reusable = true}}, REDUCE(aux_sym_argument_list_repeat1, 2, 0, 0),
  [53] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_argument_list_repeat1, 2, 0, 0), SHIFT_REPEAT(18),
  [56] = {.entry = {.count = 1, .reusable = true}}, SHIFT(44),
  [58] = {.entry = {.count = 1, .reusable = true}}, SHIFT(18),
  [60] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_parameter_list, 3, 0, 0),
  [62] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_parameter_list, 3, 0, 0),
  [64] = {.entry = {.count = 1, .reusable = true}}, SHIFT(52),
  [66] = {.entry = {.count = 1, .reusable = true}}, SHIFT(30),
  [68] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_expression, 1, 0, 0),
  [70] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_import_statement, 3, 0, 0),
  [72] = {.entry = {.count = 1, .reusable = true}}, SHIFT(38),
  [74] = {.entry = {.count = 1, .reusable = true}}, SHIFT(36),
  [76] = {.entry = {.count = 1, .reusable = true}}, SHIFT(14),
  [78] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_import_statement, 2, 0, 0),
  [80] = {.entry = {.count = 1, .reusable = true}}, SHIFT(11),
  [82] = {.entry = {.count = 1, .reusable = true}}, REDUCE(aux_sym_import_statement_repeat1, 2, 0, 0),
  [84] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_import_statement_repeat1, 2, 0, 0), SHIFT_REPEAT(38),
  [87] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_parameter_list_repeat1, 2, 0, 0), SHIFT_REPEAT(36),
  [90] = {.entry = {.count = 1, .reusable = true}}, REDUCE(aux_sym_parameter_list_repeat1, 2, 0, 0),
  [92] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_type, 2, 0, 0),
  [94] = {.entry = {.count = 1, .reusable = true}}, SHIFT(8),
  [96] = {.entry = {.count = 1, .reusable = true}}, SHIFT(17),
  [98] = {.entry = {.count = 1, .reusable = true}}, SHIFT(37),
  [100] = {.entry = {.count = 1, .reusable = true}}, REDUCE(aux_sym_parameter_list_repeat1, 4, 0, 0),
  [102] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_primative_type, 1, 0, 0),
  [104] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_type, 1, 0, 0),
  [106] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_type, 4, 0, 0),
  [108] = {.entry = {.count = 1, .reusable = true}}, SHIFT(15),
  [110] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_type, 3, 0, 0),
  [112] = {.entry = {.count = 1, .reusable = true}}, SHIFT(46),
  [114] = {.entry = {.count = 1, .reusable = true}}, SHIFT(16),
  [116] = {.entry = {.count = 1, .reusable = true}}, SHIFT(32),
  [118] = {.entry = {.count = 1, .reusable = true}}, SHIFT(22),
  [120] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_function_call, 2, 0, 0),
  [122] = {.entry = {.count = 1, .reusable = true}}, SHIFT(27),
  [124] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_block, 2, 0, 0),
  [126] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_return_statement, 2, 0, 0),
  [128] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_argument_list, 3, 0, 0),
  [130] = {.entry = {.count = 1, .reusable = true}}, SHIFT(10),
  [132] = {.entry = {.count = 1, .reusable = true}}, SHIFT(7),
  [134] = {.entry = {.count = 1, .reusable = true}},  ACCEPT_INPUT(),
  [136] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_function_definition, 5, 0, 0),
  [138] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_block, 3, 0, 0),
  [140] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_block, 1, 0, 0),
  [142] = {.entry = {.count = 1, .reusable = true}}, SHIFT(41),
  [144] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_argument_list, 2, 0, 0),
};

#ifdef __cplusplus
extern "C" {
#endif
#ifdef TREE_SITTER_HIDE_SYMBOLS
#define TS_PUBLIC
#elif defined(_WIN32)
#define TS_PUBLIC __declspec(dllexport)
#else
#define TS_PUBLIC __attribute__((visibility("default")))
#endif

TS_PUBLIC const TSLanguage *tree_sitter_Ignis(void) {
  static const TSLanguage language = {
    .version = LANGUAGE_VERSION,
    .symbol_count = SYMBOL_COUNT,
    .alias_count = ALIAS_COUNT,
    .token_count = TOKEN_COUNT,
    .external_token_count = EXTERNAL_TOKEN_COUNT,
    .state_count = STATE_COUNT,
    .large_state_count = LARGE_STATE_COUNT,
    .production_id_count = PRODUCTION_ID_COUNT,
    .field_count = FIELD_COUNT,
    .max_alias_sequence_length = MAX_ALIAS_SEQUENCE_LENGTH,
    .parse_table = &ts_parse_table[0][0],
    .small_parse_table = ts_small_parse_table,
    .small_parse_table_map = ts_small_parse_table_map,
    .parse_actions = ts_parse_actions,
    .symbol_names = ts_symbol_names,
    .symbol_metadata = ts_symbol_metadata,
    .public_symbol_map = ts_symbol_map,
    .alias_map = ts_non_terminal_alias_map,
    .alias_sequences = &ts_alias_sequences[0][0],
    .lex_modes = ts_lex_modes,
    .lex_fn = ts_lex,
    .primary_state_ids = ts_primary_state_ids,
  };
  return &language;
}
#ifdef __cplusplus
}
#endif
