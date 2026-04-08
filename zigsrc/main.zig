const std = @import("std");
const config = @import("config.zig");
const lexer = @import("lexer.zig");
const cli = @import("cli.zig");
const io = @import("io.zig");

pub fn main() !void {
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    defer _ = gpa.deinit();
    const allocator = gpa.allocator();

    var args = try std.process.argsWithAllocator(allocator);
    defer args.deinit();

    var cli_parser = try cli.CliParser.from(allocator, &args);
    defer cli_parser.deinit();

    var egg = false;
    if (cli_parser.flag_value("nut")) {
        try std.fs.File.stdout().writeAll("hey hazel! :3\n");
        egg = true;
    }
    if (cli_parser.flag_value("rizz")) {
        try std.fs.File.stdout().writeAll("hey iris! :3\n");
        egg = true;
    }
    if (egg) {
        std.process.exit(0);
    }

    if (cli_parser.arguments.items.len == 0) {
        std.debug.print("error: no main file specified\n\tusage: {s} main.is -o output\n", .{ cli_parser.args.items[0] });
        std.process.exit(1);
    } else if (cli_parser.arguments.items.len > 1) {
        std.debug.print("error: more than one main file specified\n\tusage: {s} main.is -o output\n", .{ cli_parser.args.items[0] });
        std.process.exit(1);
    }

    const input_file_path: []const u8 = cli_parser.arguments.items[0];
    var input_file = try io.SourceFile.read_file(allocator, input_file_path);
    defer input_file.deinit();

    var program_config = try config.get_config(allocator, input_file_path, &cli_parser);
    defer program_config.deinit();

    var tokenizer = try lexer.Lexer.init(allocator, program_config.main_file, input_file.contents);
    defer tokenizer.deinit();
    const tokens = try tokenizer.run();

    for (tokens.items) |token| {
        std.debug.print("{}\n", .{ token });
    }
}
