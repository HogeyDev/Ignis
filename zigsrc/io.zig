const std = @import("std");

pub const SourceFile = struct {
    allocator: std.mem.Allocator,
    path: []const u8,
    contents: []u8,

    pub fn deinit(self: *SourceFile) void {
        self.allocator.free(self.path);
        self.allocator.free(self.contents);
    }

    pub fn read_file(allocator: std.mem.Allocator, file_path: []const u8) !SourceFile {
        var source_file = SourceFile{
            .allocator = allocator,
            .path = undefined,
            .contents = undefined,
        };
        source_file.path = try source_file.allocator.dupe(u8, file_path);

        const file = try std.fs.cwd().openFile(file_path, .{});
        defer file.close();
        source_file.contents = try file.readToEndAlloc(allocator, std.math.maxInt(usize));

        return source_file;
    }
};

