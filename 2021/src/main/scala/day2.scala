package day2

import utils.*
import aoc.*

case class LocationWithAim(depth: Int = 0, horizontal: Int = 0, aim: Int=0) extends XYLocation:
    type Type = LocationWithAim
    def +(command: Command) =
        import Direction.*
        command match
            case Command(Forward, count) =>
                copy(horizontal=horizontal+count, depth=depth + (aim * count))
            case Command(Up, count) =>
                copy(aim=aim - count)
            case Command(Down, count) =>
                copy(aim=aim + count)

                
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
    