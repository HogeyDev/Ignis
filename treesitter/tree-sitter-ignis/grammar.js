module.exports = grammar({
    name: 'Ignis',

    extras: $ => [
        $.comment,
        /\s/,
    ],

    rules: {
        source_file: $ => repeat($.statement),
        statement: $ => seq(
            choice(
                $.function_definition,
                $.import_statement,
                $.return_statement, ';',
                $.while_statement,
                $.if_statement,
                $.enum_statement,
                $.struct_statement,
                $.expression_statement,
                $.variable_declaration,
            ),
        ),
        while_statement: $ => seq(
            'while',
            '(',
            $.expression,
            ')',
            $.block,
        ),
        if_statement: $ => seq(
            'if',
            '(',
            $.expression,
            ')',
            $.block,
        ),
        variable_declaration: $ => seq(
            'let', $.identifier, ':', $.type, choice(
                ';',
                seq(
                    '=',
                    $.expression,
                    ';',
                )
            ),
        ),
        enum_statement: $ => seq(
            'enum', $.identifier, '{',
                sepBy(',', $.identifier),
            '}',
        ),
        struct_statement: $ => seq(
            'struct', $.identifier, '{',
                repeat($.struct_field),
            '}',
        ),
        struct_field: $ => seq(
            $.identifier, ':', $.type, ';',
        ),
        function_call: $ => prec(5, seq(
            $.identifier,
            $.argument_list,
        )),
        return_statement: $ => seq(
            'return',
            $.expression,
            ';',
        ),
        import_statement: $ => seq(
            'import',
            $.identifier,
            repeat(
                seq(
                    '.',
                    $.identifier,
                ),
            ),
            ';',
        ),
        function_definition: $ => seq(
            'func',
            $.identifier,
            $.parameter_list,
            $.block,
        ),
        block: $ => choice(
            seq(
                '{',
                repeat($.statement),
                '}',
            ),
            $.statement,
        ),
        parameter_list: $ => seq(
            '(',
            $.type,
            repeat(
                seq(
                    ',',
                    $.identifier,
                    ':',
                    $.type,
                ),
            ),
            ')',
        ),
        argument_list: $ => seq(
            '(',
            optional(
                seq(
                    $.expression,
                    repeat(
                        seq(',', $.expression),
                    ),
                )
            ),
            ')',
        ),
        type: $ => choice(
            $.primative_type,
            $.identifier,
            seq(
                '[',
                    optional(
                        $.number,
                    ),
                ']',
                $.type,
            ),
            seq(
                '@',
                $.type,
            ),
        ),
        primative_type: $ => choice(
            'void',
            'int',
            'char',
            'usize',
        ),
        expression_statement: $ => seq(
            $.expression, ';',
        ),
        enum_variant: $ => seq(
            $.identifier,
            repeat1(seq('::', $.identifier)),
        ),
        expression: $ => choice(
            $.number,
            $.identifier,
            $.string,
            $.grouping,
            $.unary_expression,
            $.binary_expression,
            $.function_call,
            $.enum_variant
        ),
        unary_expression: $ => prec(15, seq(choice('@', '-', '!', '@', '&'), $.expression)),
        binary_expression: $ => prec.left(10, choice(
            seq($.expression, choice('+', '-', '*', '/', '%', '||', '&&', '==', '!=', '<', '>', '<=', '>=', '='), $.expression),
            seq($.expression, '[', $.expression, ']'),
            seq($.expression, '.', $.identifier),
        )),
        grouping: $ => seq(
            '(',
            $.expression,
            ')',
        ),
        comment: $ => seq('//', /.*/),
        identifier: $ => /[_a-zA-Z][_a-zA-Z0-9]*/,
        number: $ => /\d+/,
        string: $ => /"[^"]*"/,
    }
});

function sepBy(separator, rule) {
  return optional(seq(rule, repeat(seq(separator, rule))));
}
