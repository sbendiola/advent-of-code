import scala.io.Source

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

enum DayId:
    case Day1
    case Day2
    case Day3

import java.nio.file.Paths
import java.nio.file.Files

trait BaseDay(day: Int):
    val id = DayId.fromOrdinal(day - 1)
        
    //def withTestData[R](f: Source => R): R =
    //    utils.withTestData(id)(f)
    
object utils:

    def withTestData[R](id: DayId)(f: Source => R): R =
        using(relativeResource(s"day_${id.ordinal + 1}_input"))
            (_.close)(f)
    
    def relativeResource(path: String*): Source =
        val location = Array("2021", "src", "main", "resources", "2021") ++ path
        val cd = Paths.get(".", location*).toAbsolutePath
        Source.fromFile(cd.ensuring(Files.exists(_), s"$cd does not exist").toFile)

    def using[T, R](t: => T)(cleanup: T => Unit)(use: T => R): R =
        val resource = t
        try
            use(resource)
        finally
            cleanup(resource)
