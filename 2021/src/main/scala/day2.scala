package day2

import utils.*
import aoc.*

def part1(): Int = 
    utils.withTestData(day=2) { source => 
        location(Command(source.getLines))
    }

def part2(): Int = 
    utils.withTestData(day=2) { source => 
        location(Command(source.getLines), LocationWithAim())
    }

def location(commands: Iterator[Command], init: XYLocation = Location()): Int = 
    val result = commands.foldLeft(init)(_ + _)
    result.depth * result.horizontal
    