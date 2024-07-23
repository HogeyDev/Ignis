module.exports = grammar({
    name: 'Ignis',

    rules: {
        source_file: $ => repeat($.statement),
        statement: $ => choice(
            // $.function_definition,
            $.import_statement,
        ),
        import_statement: $ => seq(
            'import ',
            $.identifier,
            repeat(
                '.',
                $.identifier,
            ),
            ';',
        ),
        // function_definition: $ => seq(
        //     'func',
        //     $.identifier,
        //     '::',
        //     $.parameter_list,
        //     $.block,
        // ),
        // parameter_list: $ => seq(
        //     '(',
        //     $.type,
        //     repeat(
        //         seq(
        //             ',',
        //             $.identifier,
        //             ':',
        //             $.type,
        //         ),
        //     ),
        //     ')',
        // ),
        // type: $ => choice(
        //     $.primative_type,
        //     seq(
        //         '[',
        //             optional(
        //                 $.number,
        //             ),
        //         ']',
        //         $.primative_type,
        //     ),
        // ),
        // primative_type: $ => choice(
        //     'void',
        //     'int',
        //     'char',
        // ),
        identifier: $ => /[a-zA-Z][a-zA-Z0-9]*/,
        // number: $ => /\d+/,
    }
});
