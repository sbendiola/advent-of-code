object day2 extends BaseDay(2):
    
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
        withTestData { source => 
            location(Command(source.getLines))
        }

    def part2(): Int = 
        withTestData { source => 
            location(Command(source.getLines), LocationWithAim())
        }

    def location(commands: Iterator[Command], init: XYLocation = Location()): Int = 
        val result = commands.foldLeft(init)(_ + _)
        result.depth * result.horizontal

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

