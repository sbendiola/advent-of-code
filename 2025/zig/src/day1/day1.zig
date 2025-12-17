const std = @import("std");
const expect = std.testing.expect;
const assert = @import("std").debug.assert;
const parseInt = std.fmt.parseInt;
const DoublyLinkedList = std.DoublyLinkedList;
pub fn main() !void {
    // 1. Obtain an allocator. ArenaAllocator is convenient for a single function/scope.
    // var arena = std.heap.ArenaAllocator.init(std.heap.page_allocator);
    // defer arena.deinit();
    // const allocator = arena.allocator();

    var arena = std.heap.ArenaAllocator.init(std.heap.page_allocator);
    defer arena.deinit();
    const alloc = arena.allocator();

    // Get the current working directory (cwd) Dir object.
    const cwd_dir = std.fs.cwd();

    // Resolve the absolute path of "." (current directory) using the allocator.
    const cwd_path = try cwd_dir.realpathAlloc(alloc, ".");

    // Print the path to standard error.
    // Use std.debug.print for general debugging output.
    std.debug.print("Current working directory: {s}\n", .{cwd_path});

    // 2. Open the file.
    const file_path = "input1"; // Replace with your file name
    var file = try std.fs.cwd().openFile(file_path, .{ .mode = .read_only });
    defer file.close();

    // 3. Create a buffer for the reader and instantiate a buffered reader.
    // The buffer size should be large enough to hold a typical line or a chunk of data.
    var buffer: [1024]u8 = undefined;
    var reader = file.reader(&buffer);

    // 4. Loop through the file, reading lines until the end.
    // `readUntilDelimiterOrEof` handles reading into the provided buffer until '\n' or EOF.
    var current: usize = 50;
    var result: usize = 0;
    while (try reader.interface.takeDelimiter('\n')) |line| {
        if (line.len == 0) {
            std.debug.print("The empty line is empty.\n", .{});
            continue;
        }
        const cmd = try command(line);
        current = position(current, cmd.direction, cmd.clicks);
        if (current == 0) {
            result += 1;
        }
    }

    std.debug.print("day1 {}!\n", .{result});
}

pub fn add(a: i32, b: i32) i32 {
    return a + b;
}

const Cmd = struct {
    direction: Direction,
    clicks: usize,
};

const Direction = enum(u8) {
    L = 'L',
    R = 'R',
};
const LAST: u8 = 99;
const FIRST: u8 = 0;

fn position(start: usize, direction: Direction, count: usize) usize {
    var currentPosition = start;
    var currentCount = count;
    while (currentCount > 0) {
        switch (direction) {
            .L => {
                if (currentPosition == 0) {
                    currentPosition = LAST;
                    currentCount -= 1;
                } else if (currentPosition > currentCount) {
                    return currentPosition - currentCount;
                } else {
                    currentPosition -= 1;
                    currentCount -= 1;
                }
            },
            .R => {
                if (currentPosition == LAST) {
                    currentPosition = FIRST;
                } else {
                    currentPosition += 1;
                }
                currentCount -= 1;
            },
        }
    }
    return currentPosition;
}

fn pwd(start: usize, cmds: []Cmd) usize {
    var result: usize = 0;
    var current = start;

    for (cmds) |cmd| {
        current = position(current, cmd.direction, cmd.clicks);
        if (current == 0) {
            result += 1;
        }
    }
    return result;
}

fn command(line: []const u8) !Cmd {
    assert(line.len > 1);
    return Cmd{
        .direction = try std.meta.intToEnum(Direction, line[0]),
        .clicks = try parseInt(usize, line[1..], 10),
    };
}

fn commands(text: []const u8, allocator: std.mem.Allocator) !std.ArrayList(Cmd) {
    var result: std.ArrayList(Cmd) = .empty;
    var tk = std.mem.tokenizeAny(u8, text, "\n");
    while (tk.next()) |line| {
        try result.append(allocator, command(line));
    }
    return result;
}

test "navigations" {
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    defer _ = gpa.deinit();
    const allocator = gpa.allocator();
    const text =
        \\L68
        \\L30
        \\R48
        \\L5
        \\R60
        \\L55
        \\L1
        \\L99
        \\R14
        \\L82
    ;
    const cmds = try commands(text, allocator);
    const result = pwd(50, cmds);
    std.debug.print("result {any}\n", .{result});
    try expect(result == 3);
}

const CmdExpected = struct {
    cmd: Cmd,
    expected: usize,
};

test "position" {
    const cases = [_]CmdExpected{
        .{
            .cmd = .{ .clicks = 68, .direction = Direction.L },
            .expected = 82,
        },
        .{
            .cmd = .{ .clicks = 30, .direction = Direction.L },
            .expected = 52,
        },
        .{
            .cmd = .{ .direction = Direction.R, .clicks = 48 },
            .expected = 0,
        },
        .{
            .cmd = .{ .direction = Direction.L, .clicks = 5 },
            .expected = 95,
        },
        .{
            .cmd = .{ .direction = Direction.R, .clicks = 60 },
            .expected = 55,
        },
        .{
            .cmd = .{ .direction = Direction.L, .clicks = 55 },
            .expected = 0,
        },
        .{
            .cmd = .{ .direction = Direction.L, .clicks = 1 },
            .expected = 99,
        },
        .{
            .cmd = .{ .direction = Direction.L, .clicks = 99 },
            .expected = 0,
        },
        .{
            .cmd = .{ .direction = Direction.R, .clicks = 14 },
            .expected = 14,
        },
        .{
            .cmd = .{ .direction = Direction.L, .clicks = 82 },
            .expected = 32,
        },
    };
    var cmds: [cases.len]Cmd = undefined;

    for (cases, 0..) |case, i| {
        cmds[i] = case.cmd;
    }

    var current: usize = 50;
    for (cases) |case| {
        const before = current;
        const after = position(before, case.cmd.direction, case.cmd.clicks);
        std.debug.print("attempt {} start:{} after:{}\n", .{ case, before, after });
        try expect(after == case.expected);
        current = after;
    }

    try expect(pwd(50, &cmds) == 3);

    try expect(position(32, .R, 70) == 2);
    try expect(position(32, .L, 43) == 89);
}
