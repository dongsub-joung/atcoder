const std = @import("std");

const stdout = std.io.getStdOut().writer();
const stdin = std.io.getStdIn().reader();

// not working
pub fn main() !void {
    var buffer: [10]u8 = undefined;
    @memset(buffer[0..], 0);
    _ = try stdin.readUntilDelimiterOrEof(buffer[0..], '\n');

    var idx = 0;
    for (buffer) |e| {
        if (idx == 0) {
            const condition = std.mem.eql(u8, e, 'A');
            if (condition) {
                const remain = "CE";
                var flag = false;
                var flag2 = false;
                if (buffer[1] == remain[0]) {
                    flag = true;
                }
                if (buffer[2] == remain[1]) {
                    flag2 = true;
                }

                if (flag & flag2) {
                    stdout.print("Yes");
                }
            }
        }
    }
    try stdout.print("No");
}
