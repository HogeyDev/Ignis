#include "tree_sitter/parser.h"

#if defined(__GNUC__) || defined(__clang__)
#pragma GCC diagnostic ignored "-Wmissing-field-initializers"
#endif

#define LANGUAGE_VERSION 14
#define STATE_COUNT 36
#define LARGE_STATE_COUNT 2
#define SYMBOL_COUNT 27
#define ALIAS_COUNT 0
#define TOKEN_COUNT 17
#define EXTERNAL_TOKEN_COUNT 0
#define FIELD_COUNT 0
#define MAX_ALIAS_SEQUENCE_LENGTH 4
#define PRODUCTION_ID_COUNT 1

enum ts_symbol_identifiers {
  anon_sym_import = 1,
  anon_sym_DOT = 2,
  anon_sym_SEMI = 3,
  anon_sym_func = 4,
  anon_sym_COLON_COLON = 5,
  anon_sym_LPAREN = 6,
  anon_sym_COMMA = 7,
  anon_sym_COLON = 8,
  anon_sym_RPAREN = 9,
  anon_sym_LBRACK = 10,
  anon_sym_RBRACK = 11,
  anon_sym_void = 12,
  anon_sym_int = 13,
  anon_sym_char = 14,
  sym_identifier = 15,
  sym_number = 16,
  sym_source_file = 17,
  sym_statement = 18,
  sym_import_statement = 19,
  sym_function_definition = 20,
  sym_parameter_list = 21,
  sym_type = 22,
  sym_primative_type = 23,
  aux_sym_source_file_repeat1 = 24,
  aux_sym_import_statement_repeat1 = 25,
  aux_sym_parameter_list_repeat1 = 26,
};

static const char * const ts_symbol_names[] = {
  [ts_builtin_sym_end] = "end",
  [anon_sym_import] = "import",
  [anon_sym_DOT] = ".",
  [anon_sym_SEMI] = ";",
  [anon_sym_func] = "func",
  [anon_sym_COLON_COLON] = "::",
  [anon_sym_LPAREN] = "(",
  [anon_sym_COMMA] = ",",
  [anon_sym_COLON] = ":",
  [anon_sym_RPAREN] = ")",
  [anon_sym_LBRACK] = "[",
  [anon_sym_RBRACK] = "]",
  [anon_sym_void] = "void",
  [anon_sym_int] = "int",
  [anon_sym_char] = "char",
  [sym_identifier] = "identifier",
  [sym_number] = "number",
  [sym_source_file] = "source_file",
  [sym_statement] = "statement",
  [sym_import_statement] = "import_statement",
  [sym_function_definition] = "function_definition",
  [sym_parameter_list] = "parameter_list",
  [sym_type] = "type",
  [sym_primative_type] = "primative_type",
  [aux_sym_source_file_repeat1] = "source_file_repeat1",
  [aux_sym_import_statement_repeat1] = "import_statement_repeat1",
  [aux_sym_parameter_list_repeat1] = "parameter_list_repeat1",
};

static const TSSymbol ts_symbol_map[] = {
  [ts_builtin_sym_end] = ts_builtin_sym_end,
  [anon_sym_import] = anon_sym_import,
  [anon_sym_DOT] = anon_sym_DOT,
  [anon_sym_SEMI] = anon_sym_SEMI,
  [anon_sym_func] = anon_sym_func,
  [anon_sym_COLON_COLON] = anon_sym_COLON_COLON,
  [anon_sym_LPAREN] = anon_sym_LPAREN,
  [anon_sym_COMMA] = anon_sym_COMMA,
  [anon_sym_COLON] = anon_sym_COLON,
  [anon_sym_RPAREN] = anon_sym_RPAREN,
  [anon_sym_LBRACK] = anon_sym_LBRACK,
  [anon_sym_RBRACK] = anon_sym_RBRACK,
  [anon_sym_void] = anon_sym_void,
  [anon_sym_int] = anon_sym_int,
  [anon_sym_char] = anon_sym_char,
  [sym_identifier] = sym_identifier,
  [sym_number] = sym_number,
  [sym_source_file] = sym_source_file,
  [sym_statement] = sym_statement,
  [sym_import_statement] = sym_import_statement,
  [sym_function_definition] = sym_function_definition,
  [sym_parameter_list] = sym_parameter_list,
  [sym_type] = sym_type,
  [sym_primative_type] = sym_primative_type,
  [aux_sym_source_file_repeat1] = aux_sym_source_file_repeat1,
  [aux_sym_import_statement_repeat1] = aux_sym_import_statement_repeat1,
  [aux_sym_parameter_list_repeat1] = aux_sym_parameter_list_repeat1,
};

static const TSSymbolMetadata ts_symbol_metadata[] = {
  [ts_builtin_sym_end] = {
    .visible = false,
    .named = true,
  },
  [anon_sym_import] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_DOT] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_SEMI] = {
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
  [sym_import_statement] = {
    .visible = true,
    .named = true,
  },
  [sym_function_definition] = {
    .visible = true,
    .named = true,
  },
  [sym_parameter_list] = {
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
};

static bool ts_lex(TSLexer *lexer, TSStateId state) {
  START_LEXER();
  eof = lexer->eof(lexer);
  switch (state) {
    case 0:
      if (eof) ADVANCE(19);
      ADVANCE_MAP(
        '(', 25,
        ')', 29,
        ',', 26,
        '.', 21,
        ':', 28,
        ';', 22,
        '[', 30,
        ']', 31,
        'c', 7,
        'f', 18,
        'i', 9,
        'v', 11,
      );
      if (('\t' <= lookahead && lookahead <= '\r') ||
          lookahead == ' ') SKIP(0);
      if (('0' <= lookahead && lookahead <= '9')) ADVANCE(36);
      END_STATE();
    case 1:
      if (lookahead == ':') ADVANCE(24);
      END_STATE();
    case 2:
      if (lookahead == ':') ADVANCE(27);
      if (('\t' <= lookahead && lookahead <= '\r') ||
          lookahead == ' ') SKIP(2);
      if (('A' <= lookahead && lookahead <= 'Z') ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(35);
      END_STATE();
    case 3:
      if (lookahead == ':') ADVANCE(1);
      if (('\t' <= lookahead && lookahead <= '\r') ||
          lookahead == ' ') SKIP(3);
      END_STATE();
    case 4:
      if (lookahead == 'a') ADVANCE(14);
      END_STATE();
    case 5:
      if (lookahead == 'c') ADVANCE(23);
      END_STATE();
    case 6:
      if (lookahead == 'd') ADVANCE(32);
      END_STATE();
    case 7:
      if (lookahead == 'h') ADVANCE(4);
      END_STATE();
    case 8:
      if (lookahead == 'i') ADVANCE(6);
      END_STATE();
    case 9:
      if (lookahead == 'm') ADVANCE(13);
      if (lookahead == 'n') ADVANCE(16);
      END_STATE();
    case 10:
      if (lookahead == 'n') ADVANCE(5);
      END_STATE();
    case 11:
      if (lookahead == 'o') ADVANCE(8);
      END_STATE();
    case 12:
      if (lookahead == 'o') ADVANCE(15);
      END_STATE();
    case 13:
      if (lookahead == 'p') ADVANCE(12);
      END_STATE();
    case 14:
      if (lookahead == 'r') ADVANCE(34);
      END_STATE();
    case 15:
      if (lookahead == 'r') ADVANCE(17);
      END_STATE();
    case 16:
      if (lookahead == 't') ADVANCE(33);
      END_STATE();
    case 17:
      if (lookahead == 't') ADVANCE(20);
      END_STATE();
    case 18:
      if (lookahead == 'u') ADVANCE(10);
      END_STATE();
    case 19:
      ACCEPT_TOKEN(ts_builtin_sym_end);
      END_STATE();
    case 20:
      ACCEPT_TOKEN(anon_sym_import);
      END_STATE();
    case 21:
      ACCEPT_TOKEN(anon_sym_DOT);
      END_STATE();
    case 22:
      ACCEPT_TOKEN(anon_sym_SEMI);
      END_STATE();
    case 23:
      ACCEPT_TOKEN(anon_sym_func);
      END_STATE();
    case 24:
      ACCEPT_TOKEN(anon_sym_COLON_COLON);
      END_STATE();
    case 25:
      ACCEPT_TOKEN(anon_sym_LPAREN);
      END_STATE();
    case 26:
      ACCEPT_TOKEN(anon_sym_COMMA);
      END_STATE();
    case 27:
      ACCEPT_TOKEN(anon_sym_COLON);
      END_STATE();
    case 28:
      ACCEPT_TOKEN(anon_sym_COLON);
      if (lookahead == ':') ADVANCE(24);
      END_STATE();
    case 29:
      ACCEPT_TOKEN(anon_sym_RPAREN);
      END_STATE();
    case 30:
      ACCEPT_TOKEN(anon_sym_LBRACK);
      END_STATE();
    case 31:
      ACCEPT_TOKEN(anon_sym_RBRACK);
      END_STATE();
    case 32:
      ACCEPT_TOKEN(anon_sym_void);
      END_STATE();
    case 33:
      ACCEPT_TOKEN(anon_sym_int);
      END_STATE();
    case 34:
      ACCEPT_TOKEN(anon_sym_char);
      END_STATE();
    case 35:
      ACCEPT_TOKEN(sym_identifier);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(35);
      END_STATE();
    case 36:
      ACCEPT_TOKEN(sym_number);
      if (('0' <= lookahead && lookahead <= '9')) ADVANCE(36);
      END_STATE();
    default:
      return false;
  }
}

static const TSLexMode ts_lex_modes[STATE_COUNT] = {
  [0] = {.lex_state = 0},
  [1] = {.lex_state = 0},
  [2] = {.lex_state = 0},
  [3] = {.lex_state = 0},
  [4] = {.lex_state = 0},
  [5] = {.lex_state = 0},
  [6] = {.lex_state = 0},
  [7] = {.lex_state = 0},
  [8] = {.lex_state = 0},
  [9] = {.lex_state = 0},
  [10] = {.lex_state = 0},
  [11] = {.lex_state = 0},
  [12] = {.lex_state = 0},
  [13] = {.lex_state = 0},
  [14] = {.lex_state = 0},
  [15] = {.lex_state = 0},
  [16] = {.lex_state = 0},
  [17] = {.lex_state = 0},
  [18] = {.lex_state = 0},
  [19] = {.lex_state = 0},
  [20] = {.lex_state = 0},
  [21] = {.lex_state = 0},
  [22] = {.lex_state = 0},
  [23] = {.lex_state = 0},
  [24] = {.lex_state = 0},
  [25] = {.lex_state = 0},
  [26] = {.lex_state = 0},
  [27] = {.lex_state = 0},
  [28] = {.lex_state = 2},
  [29] = {.lex_state = 0},
  [30] = {.lex_state = 2},
  [31] = {.lex_state = 0},
  [32] = {.lex_state = 2},
  [33] = {.lex_state = 2},
  [34] = {.lex_state = 3},
  [35] = {.lex_state = 2},
};

static const uint16_t ts_parse_table[LARGE_STATE_COUNT][SYMBOL_COUNT] = {
  [0] = {
    [ts_builtin_sym_end] = ACTIONS(1),
    [anon_sym_import] = ACTIONS(1),
    [anon_sym_DOT] = ACTIONS(1),
    [anon_sym_SEMI] = ACTIONS(1),
    [anon_sym_func] = ACTIONS(1),
    [anon_sym_COLON_COLON] = ACTIONS(1),
    [anon_sym_LPAREN] = ACTIONS(1),
    [anon_sym_COMMA] = ACTIONS(1),
    [anon_sym_COLON] = ACTIONS(1),
    [anon_sym_RPAREN] = ACTIONS(1),
    [anon_sym_LBRACK] = ACTIONS(1),
    [anon_sym_RBRACK] = ACTIONS(1),
    [anon_sym_void] = ACTIONS(1),
    [anon_sym_int] = ACTIONS(1),
    [anon_sym_char] = ACTIONS(1),
    [sym_number] = ACTIONS(1),
  },
  [1] = {
    [sym_source_file] = STATE(31),
    [sym_statement] = STATE(2),
    [sym_import_statement] = STATE(18),
    [sym_function_definition] = STATE(18),
    [aux_sym_source_file_repeat1] = STATE(2),
    [ts_builtin_sym_end] = ACTIONS(3),
    [anon_sym_import] = ACTIONS(5),
    [anon_sym_func] = ACTIONS(7),
  },
};

static const uint16_t ts_small_parse_table[] = {
  [0] = 5,
    ACTIONS(5), 1,
      anon_sym_import,
    ACTIONS(7), 1,
      anon_sym_func,
    ACTIONS(9), 1,
      ts_builtin_sym_end,
    STATE(3), 2,
      sym_statement,
      aux_sym_source_file_repeat1,
    STATE(18), 2,
      sym_import_statement,
      sym_function_definition,
  [18] = 5,
    ACTIONS(11), 1,
      ts_builtin_sym_end,
    ACTIONS(13), 1,
      anon_sym_import,
    ACTIONS(16), 1,
      anon_sym_func,
    STATE(3), 2,
      sym_statement,
      aux_sym_source_file_repeat1,
    STATE(18), 2,
      sym_import_statement,
      sym_function_definition,
  [36] = 4,
    ACTIONS(19), 1,
      anon_sym_LBRACK,
    STATE(19), 1,
      sym_type,
    STATE(22), 1,
      sym_primative_type,
    ACTIONS(21), 3,
      anon_sym_void,
      anon_sym_int,
      anon_sym_char,
  [51] = 4,
    ACTIONS(19), 1,
      anon_sym_LBRACK,
    STATE(22), 1,
      sym_primative_type,
    STATE(27), 1,
      sym_type,
    ACTIONS(21), 3,
      anon_sym_void,
      anon_sym_int,
      anon_sym_char,
  [66] = 2,
    STATE(26), 1,
      sym_primative_type,
    ACTIONS(21), 3,
      anon_sym_void,
      anon_sym_int,
      anon_sym_char,
  [75] = 2,
    STATE(25), 1,
      sym_primative_type,
    ACTIONS(21), 3,
      anon_sym_void,
      anon_sym_int,
      anon_sym_char,
  [84] = 1,
    ACTIONS(23), 3,
      ts_builtin_sym_end,
      anon_sym_import,
      anon_sym_func,
  [90] = 3,
    ACTIONS(25), 1,
      anon_sym_COMMA,
    ACTIONS(27), 1,
      anon_sym_RPAREN,
    STATE(10), 1,
      aux_sym_parameter_list_repeat1,
  [100] = 3,
    ACTIONS(29), 1,
      anon_sym_COMMA,
    ACTIONS(32), 1,
      anon_sym_RPAREN,
    STATE(10), 1,
      aux_sym_parameter_list_repeat1,
  [110] = 1,
    ACTIONS(34), 3,
      ts_builtin_sym_end,
      anon_sym_import,
      anon_sym_func,
  [116] = 1,
    ACTIONS(36), 3,
      ts_builtin_sym_end,
      anon_sym_import,
      anon_sym_func,
  [122] = 3,
    ACTIONS(38), 1,
      anon_sym_DOT,
    ACTIONS(40), 1,
      anon_sym_SEMI,
    STATE(17), 1,
      aux_sym_import_statement_repeat1,
  [132] = 3,
    ACTIONS(38), 1,
      anon_sym_DOT,
    ACTIONS(42), 1,
      anon_sym_SEMI,
    STATE(13), 1,
      aux_sym_import_statement_repeat1,
  [142] = 1,
    ACTIONS(44), 3,
      ts_builtin_sym_end,
      anon_sym_import,
      anon_sym_func,
  [148] = 1,
    ACTIONS(46), 3,
      ts_builtin_sym_end,
      anon_sym_import,
      anon_sym_func,
  [154] = 3,
    ACTIONS(48), 1,
      anon_sym_DOT,
    ACTIONS(51), 1,
      anon_sym_SEMI,
    STATE(17), 1,
      aux_sym_import_statement_repeat1,
  [164] = 1,
    ACTIONS(53), 3,
      ts_builtin_sym_end,
      anon_sym_import,
      anon_sym_func,
  [170] = 3,
    ACTIONS(25), 1,
      anon_sym_COMMA,
    ACTIONS(55), 1,
      anon_sym_RPAREN,
    STATE(9), 1,
      aux_sym_parameter_list_repeat1,
  [180] = 1,
    ACTIONS(57), 2,
      anon_sym_COMMA,
      anon_sym_RPAREN,
  [185] = 2,
    ACTIONS(59), 1,
      anon_sym_RBRACK,
    ACTIONS(61), 1,
      sym_number,
  [192] = 1,
    ACTIONS(63), 2,
      anon_sym_COMMA,
      anon_sym_RPAREN,
  [197] = 1,
    ACTIONS(51), 2,
      anon_sym_DOT,
      anon_sym_SEMI,
  [202] = 2,
    ACTIONS(65), 1,
      anon_sym_LPAREN,
    STATE(8), 1,
      sym_parameter_list,
  [209] = 1,
    ACTIONS(67), 2,
      anon_sym_COMMA,
      anon_sym_RPAREN,
  [214] = 1,
    ACTIONS(69), 2,
      anon_sym_COMMA,
      anon_sym_RPAREN,
  [219] = 1,
    ACTIONS(71), 2,
      anon_sym_COMMA,
      anon_sym_RPAREN,
  [224] = 1,
    ACTIONS(73), 1,
      sym_identifier,
  [228] = 1,
    ACTIONS(75), 1,
      anon_sym_RBRACK,
  [232] = 1,
    ACTIONS(77), 1,
      sym_identifier,
  [236] = 1,
    ACTIONS(79), 1,
      ts_builtin_sym_end,
  [240] = 1,
    ACTIONS(81), 1,
      anon_sym_COLON,
  [244] = 1,
    ACTIONS(83), 1,
      sym_identifier,
  [248] = 1,
    ACTIONS(85), 1,
      anon_sym_COLON_COLON,
  [252] = 1,
    ACTIONS(87), 1,
      sym_identifier,
};

static const uint32_t ts_small_parse_table_map[] = {
  [SMALL_STATE(2)] = 0,
  [SMALL_STATE(3)] = 18,
  [SMALL_STATE(4)] = 36,
  [SMALL_STATE(5)] = 51,
  [SMALL_STATE(6)] = 66,
  [SMALL_STATE(7)] = 75,
  [SMALL_STATE(8)] = 84,
  [SMALL_STATE(9)] = 90,
  [SMALL_STATE(10)] = 100,
  [SMALL_STATE(11)] = 110,
  [SMALL_STATE(12)] = 116,
  [SMALL_STATE(13)] = 122,
  [SMALL_STATE(14)] = 132,
  [SMALL_STATE(15)] = 142,
  [SMALL_STATE(16)] = 148,
  [SMALL_STATE(17)] = 154,
  [SMALL_STATE(18)] = 164,
  [SMALL_STATE(19)] = 170,
  [SMALL_STATE(20)] = 180,
  [SMALL_STATE(21)] = 185,
  [SMALL_STATE(22)] = 192,
  [SMALL_STATE(23)] = 197,
  [SMALL_STATE(24)] = 202,
  [SMALL_STATE(25)] = 209,
  [SMALL_STATE(26)] = 214,
  [SMALL_STATE(27)] = 219,
  [SMALL_STATE(28)] = 224,
  [SMALL_STATE(29)] = 228,
  [SMALL_STATE(30)] = 232,
  [SMALL_STATE(31)] = 236,
  [SMALL_STATE(32)] = 240,
  [SMALL_STATE(33)] = 244,
  [SMALL_STATE(34)] = 248,
  [SMALL_STATE(35)] = 252,
};

static const TSParseActionEntry ts_parse_actions[] = {
  [0] = {.entry = {.count = 0, .reusable = false}},
  [1] = {.entry = {.count = 1, .reusable = false}}, RECOVER(),
  [3] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_source_file, 0, 0, 0),
  [5] = {.entry = {.count = 1, .reusable = true}}, SHIFT(28),
  [7] = {.entry = {.count = 1, .reusable = true}}, SHIFT(35),
  [9] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_source_file, 1, 0, 0),
  [11] = {.entry = {.count = 1, .reusable = true}}, REDUCE(aux_sym_source_file_repeat1, 2, 0, 0),
  [13] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_source_file_repeat1, 2, 0, 0), SHIFT_REPEAT(28),
  [16] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_source_file_repeat1, 2, 0, 0), SHIFT_REPEAT(35),
  [19] = {.entry = {.count = 1, .reusable = true}}, SHIFT(21),
  [21] = {.entry = {.count = 1, .reusable = true}}, SHIFT(20),
  [23] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_function_definition, 4, 0, 0),
  [25] = {.entry = {.count = 1, .reusable = true}}, SHIFT(30),
  [27] = {.entry = {.count = 1, .reusable = true}}, SHIFT(11),
  [29] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_parameter_list_repeat1, 2, 0, 0), SHIFT_REPEAT(30),
  [32] = {.entry = {.count = 1, .reusable = true}}, REDUCE(aux_sym_parameter_list_repeat1, 2, 0, 0),
  [34] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_parameter_list, 4, 0, 0),
  [36] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_import_statement, 3, 0, 0),
  [38] = {.entry = {.count = 1, .reusable = true}}, SHIFT(33),
  [40] = {.entry = {.count = 1, .reusable = true}}, SHIFT(16),
  [42] = {.entry = {.count = 1, .reusable = true}}, SHIFT(12),
  [44] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_parameter_list, 3, 0, 0),
  [46] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_import_statement, 4, 0, 0),
  [48] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_import_statement_repeat1, 2, 0, 0), SHIFT_REPEAT(33),
  [51] = {.entry = {.count = 1, .reusable = true}}, REDUCE(aux_sym_import_statement_repeat1, 2, 0, 0),
  [53] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_statement, 1, 0, 0),
  [55] = {.entry = {.count = 1, .reusable = true}}, SHIFT(15),
  [57] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_primative_type, 1, 0, 0),
  [59] = {.entry = {.count = 1, .reusable = true}}, SHIFT(7),
  [61] = {.entry = {.count = 1, .reusable = true}}, SHIFT(29),
  [63] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_type, 1, 0, 0),
  [65] = {.entry = {.count = 1, .reusable = true}}, SHIFT(4),
  [67] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_type, 3, 0, 0),
  [69] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_type, 4, 0, 0),
  [71] = {.entry = {.count = 1, .reusable = true}}, REDUCE(aux_sym_parameter_list_repeat1, 4, 0, 0),
  [73] = {.entry = {.count = 1, .reusable = true}}, SHIFT(14),
  [75] = {.entry = {.count = 1, .reusable = true}}, SHIFT(6),
  [77] = {.entry = {.count = 1, .reusable = true}}, SHIFT(32),
  [79] = {.entry = {.count = 1, .reusable = true}},  ACCEPT_INPUT(),
  [81] = {.entry = {.count = 1, .reusable = true}}, SHIFT(5),
  [83] = {.entry = {.count = 1, .reusable = true}}, SHIFT(23),
  [85] = {.entry = {.count = 1, .reusable = true}}, SHIFT(24),
  [87] = {.entry = {.count = 1, .reusable = true}}, SHIFT(34),
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
