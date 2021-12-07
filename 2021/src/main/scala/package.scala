package aoc

enum Direction:
    case Up, Down, Forward

object Direction:
    def from(text: String): Direction = 
        Direction.values.find(_.productPrefix.equalsIgnoreCase(text))
            .getOrElse(scala.sys.error(s"no Direction for $text"))


case class Command(direction: Direction, steps: Int)

trait XYLocation:
    type Type <: XYLocation
    def depth: Int
    def horizontal: Int
    def aim: Int
    def +(command: Command): Type

    
case class Location(depth: Int = 0, horizontal: Int = 0, aim: Int=0) extends XYLocation:
    type Type = Location
    def +(command: Command) =
        import Direction.*
        command match
            case Command(Forward, count) =>
                copy(horizontal=horizontal+count)
            case Command(Up, count) =>
                copy(depth=depth-count)
            case Command(Down, count) =>
                copy(depth=depth+count)
    
object Command:

    def apply(text: Iterator[String]): Iterator[Command] = 
        text.map(_ match 
            case s"$direction $count" => Command(Direction.from(direction), count.toInt))

