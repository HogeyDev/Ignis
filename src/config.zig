const std = @import("std");
const cli = @import("cli.zig");

pub const ProgramConfig = struct {
    allocator: std.mem.Allocator,
    main_file: []const u8,
    root_path: []const u8,
    std_path: []const u8,
    imported_files: std.ArrayList([]const u8),
    
    pub fn init(allocator: std.mem.Allocator) ProgramConfig {
        return ProgramConfig{
            .allocator = allocator,
            .main_file = undefined,
            .root_path = undefined,
            .std_path = undefined,
            .imported_files = .empty,
        };
    }
    pub fn deinit(self: *ProgramConfig) void {
        self.allocator.free(self.root_path);
        self.allocator.free(self.std_path);
    }
};

pub fn get_config(allocator: std.mem.Allocator, main_file: []const u8, cli_parser: *cli.CliParser) !ProgramConfig {
    var config = ProgramConfig.init(allocator);

    config.main_file = main_file;
    config.root_path = try std.fs.cwd().realpathAlloc(allocator, ".");

    config.std_path = cli_parser.option_value("stdlib", "/home/iris/Programming/Ignis/std/"); // TODO: replace this with a not hardcoded path please
    config.std_path = try std.fs.cwd().realpathAlloc(allocator, config.std_path);

    return config;
}
