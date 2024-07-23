module.exports = grammar({
    name: 'Ignis',

    rules: {
        source_file: $ => repeat($.statement),
        statement: $ => seq(
            choice(
                $.function_definition,
                $.import_statement,
                $.return_statement,
                $.function_call,
            ),
            ';',
        ),
        function_call: $ => seq(
            $.identifier,
            $.argument_list,
        ),
        return_statement: $ => seq(
            'return',
            $.expression,
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
        ),
        function_definition: $ => seq(
            'func',
            $.identifier,
            '::',
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
            repeat($.expression),
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
                $.primative_type,
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
        ),
        expression: $ => choice(
            $.number,
            $.identifier,
        ),
        identifier: $ => /[_a-zA-Z][_a-zA-Z0-9]*/,
        number: $ => /\d+/,
    }
});
