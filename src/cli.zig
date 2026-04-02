const std = @import("std");

pub const CliParser = struct {
    allocator: std.mem.Allocator,
    flags: std.StringHashMap(void),
    options: std.StringHashMap([]const u8),
    arguments: std.ArrayList([]const u8),

    pub fn init(allocator: std.mem.Allocator) CliParser {
        return CliParser{
            .allocator = allocator,
            .flags = std.StringHashMap(void).init(allocator),
            .options = std.StringHashMap([]const u8).init(allocator),
            .arguments = .empty,
        };
    }
    pub fn deinit(self: *CliParser) void {
        self.flags.deinit();
        self.options.deinit();
        self.arguments.deinit(self.allocator);
    }

    pub fn from(allocator: std.mem.Allocator, args_iter: *std.process.ArgIterator) !CliParser {
        var cli_parser = CliParser.init(allocator);

        _ = args_iter.next();
        while (args_iter.next()) |arg| {
            if (arg.len > 0 and arg[0] == '-') {
                if (arg.len > 1 and arg[1] == '-') {
                    const flag = arg[2..];
                    try cli_parser.flags.put(flag, {});
                } else {
                    const option = arg[1..];
                    if (args_iter.next()) |value| {
                        try cli_parser.options.put(option, value);
                    } else {
                        std.debug.print("expected argument after `{s}`\n", .{ option });
                        std.process.exit(1);
                    }
                }
            } else {
                try cli_parser.arguments.append(cli_parser.allocator, arg);
            }
        }

        return cli_parser;
    }

    pub fn flag_value(self: *CliParser, flag: []const u8) bool {
        return self.flags.contains(flag);
    }
    pub fn option_value(self: *CliParser, option: []const u8, fallback: []const u8) []const u8 {
        return self.options.get(option) orelse fallback;
    }
};
