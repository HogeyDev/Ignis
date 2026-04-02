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
    var it = cli_parser.options.iterator();
    while (it.next()) |entry| {
        std.debug.print("\t{s}: {s}\n", .{ entry.key_ptr.*, entry.value_ptr.* });
    }
}
