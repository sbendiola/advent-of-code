const std = @import("std");
const expect = std.testing.expect;
const assert = @import("std").debug.assert;
const parseInt = std.fmt.parseInt;
const DoublyLinkedList = std.DoublyLinkedList;
pub fn main() !void {
    std.debug.print("Hello, {s}!\n", .{"World"});
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
fn position(start: usize, direction: Direction, count: usize) usize {
    if (count == 0) return start;

    switch (direction) {
        .L => {
            if (start == 0) {
                return position(LAST, direction, count - 1);
            } else if (start > count) {
                return start - count;
            }
            return position(start - 1, direction, count - 1);
        },
        .R => {
            if (start == LAST) {
                return position(0, direction, count - 1);
            }
            return position(start + 1, direction, count - 1);
        },
    }
}

fn pwd(start: usize, cmds: std.ArrayList(Cmd)) usize {
    var list = DoublyLinkedList(usize);
    for (0..99) |value| {
        try list.append(list, value);
    }
    var current = list.first;
    //move to the current position
    while (current) |node| {
        if (node == start) {
            break;
        }
        std.debug.print("start: {} node:{}\n ", .{ start, node });
        current = node.next;
    }

    const len: usize = cmds.items.len;
    // for (cmds.items) |cmd| {
    //     const before = current;
    //     const after: usize = undefined;
    //     switch (cmd.direction) {
    //         .L => {},
    //         .R => {},
    //     }
    // }
    return start + len;
}

fn commands(text: []const u8, allocator: std.mem.Allocator) !std.ArrayList(Cmd) {
    var result: std.ArrayList(Cmd) = .empty;
    var tk = std.mem.tokenizeAny(u8, text, "\n");
    while (tk.next()) |line| {
        assert(line.len > 1);
        const cmd = Cmd{
            .direction = try std.meta.intToEnum(Direction, line[0]),
            .clicks = try parseInt(usize, line[1..], 10),
        };
        try result.append(allocator, cmd);
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
    var current: usize = 50;
    for (cases) |case| {
        const before = current;
        const after = position(before, case.cmd.direction, case.cmd.clicks);
        std.debug.print("attempt {} start:{} after:{}\n", .{ case, before, after });
        try expect(after == case.expected);
        current = after;
    }
}
