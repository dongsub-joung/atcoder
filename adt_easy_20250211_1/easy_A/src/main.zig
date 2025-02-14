const std = @import("std");

pub fn main() !void {

    // stdout is for the actual output of your application, for example if you
    // are implementing gzip, then only the compressed bytes should be sent to
    // stdout, not any debugging messages.
    const stdout_file = std.io.getStdOut().writer();
    var bw = std.io.bufferedWriter(stdout_file);
    const stdout = bw.writer();

    const n = 3;
    // const n = 10;
    const times = n / 3;
    const result = times + (n % 3);

    if (times == 0) {}
    try stdout.print("{d}\n", .{result});

    try bw.flush(); // don't forget to flush!
}
