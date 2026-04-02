const std = @import("std");
const cli = @import("cli.zig");

pub fn main() !void {
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    defer _ = gpa.deinit();
    const allocator = gpa.allocator();

    var args = try std.process.argsWithAllocator(allocator);
    defer args.deinit();

    var cli_parser = try cli.CliParser.from(allocator, &args);
    defer cli_parser.deinit();

    try std.fs.File.stdout().writeAll("hey nut! :3\n");
}
