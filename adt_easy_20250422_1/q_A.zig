const std = @import("std");

const stdout = std.io.getStdOut().writer();
const stdin = std.io.getStdIn().reader();

// not working
pub fn main() !void {
    var buffer: [10]u8 = undefined;
    @memset(buffer[0..], 0);
    _ = try stdin.readUntilDelimiterOrEof(buffer[0..], '\n');

    if (std.mem.eql(buffer, "ACE")) {
        try stdout.print("Yes");
    }
    if (std.mem.eql(buffer, "BDF")) {
        try stdout.print("Yes");
    }
    if (std.mem.eql(buffer, "CEG")) {
        try stdout.print("Yes");
    }
    if (std.mem.eql(buffer, "DFA")) {
        try stdout.print("Yes");
    }
    if (std.mem.eql(buffer, "EGB")) {
        try stdout.print("Yes");
    }
    if (std.mem.eql(buffer, "FAC")) {
        try stdout.print("Yes");
    }
    if (std.mem.eql(buffer, "GBD")) {
        try stdout.print("Yes");
    }

    try stdout.print("No")
}
