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

    std.debug.print("flags:\n", .{});
    var flags_iter = cli_parser.flags.iterator();
    while (flags_iter.next()) |entry| {
        std.debug.print("\t{s}\n", .{ entry.key_ptr.* });
    }
    std.debug.print("options:\n", .{});
    var options_iter = cli_parser.options.iterator();
    while (options_iter.next()) |entry| {
        std.debug.print("\t{s}: {s}\n", .{ entry.key_ptr.*, entry.value_ptr.* });
    }
    std.debug.print("arguments:\n", .{});
    for (cli_parser.arguments.items) |arg| {
        std.debug.print("\t{s}\n", .{ arg });
    }

    try std.fs.File.stdout().writeAll("Hey, Nut!\n");
}
