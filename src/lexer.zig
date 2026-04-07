const std = @import("std");

pub const Token = union(enum) {
    Ident: []u8,
    String: []u8,
    Integer: struct { []u8, []u8 },
    Char: u8,

    LCurly,
    RCurly,
    LParen,
    RParen,
    LSquare,
    RSquare,
    
    Colon,
    Semi,
    Comma,

    SingleEquals,
    LogOr,
    LogAnd,
    LogNot,
    DoubleEquals,
    NotEquals,
    LessThan,
    MoreThan,
    LessThanEq,
    MoreThanEq,
    BitOr,
    BitXor,
    BitNeg,
    LShift,
    RShift,
    Plus,
    Minus,
    Star,
    Slash,
    Percent,
    Ampersand,
    At,

    Dot,
    Arrow,
};

pub const Lexer = struct {
    allocator: std.mem.Allocator,
    filename: []const u8,
    source: []u8,
    lines: std.ArrayList([]const u8),
    index: usize,
    pos: struct { usize, usize },
    
    pub fn init(allocator: std.mem.Allocator, filename: []const u8, source: []u8) !Lexer {
        var lines_iter = std.mem.splitSequence(u8, source, "\n");
        var lines: std.ArrayList([]const u8) = .empty;
        while (lines_iter.next()) |line| {
            try lines.append(allocator, line);
        }

        return Lexer{
            .allocator = allocator,
            .filename = filename,
            .source = source,
            .lines = lines,
            .index = 0,
            .pos = .{ 0, 0 },
        };
    }
    pub fn deinit(self: *Lexer) void {
        self.lines.deinit(self.allocator);
    }

    pub fn run(self: *Lexer) !std.ArrayList(Token) {
        var ts: std.ArrayList(Token) = .empty;

        var buffer = "123".*;
        const slice: []u8 = &buffer;
        var buffer2 = "u8".*;
        const slice2: []u8 = &buffer2;
        try ts.append(self.allocator, .{ .Integer = .{ slice, slice2 } });

        return ts;
    }
};
