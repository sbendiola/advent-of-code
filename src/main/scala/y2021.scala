package y2021:
    
    import java.nio.file.{Paths, Path, Files}
    import scala.io.Source
    import javax.print.attribute.standard.PresentationDirection

    package utils:

        def relativeResource(path: String*): Source =
            val location = Array("src", "main", "resources", "2021") ++ path
            val cd = Paths.get(".", location*).toAbsolutePath
            Source.fromFile(cd.ensuring(Files.exists(_), s"$cd does not exist").toFile)
        
        def using[T, R](t: => T)(cleanup: T => Unit)(use: T => R): R =
            val resource = t
            try
                use(resource)
            finally
                cleanup(resource)

        def withSource[R](day: Int, f: Source => R):R =
            using(relativeResource(s"day_${day}_input"))
                (_.close)
                (f)
    end utils
             
    package day1:
        
        import y2021.utils.*
        import PartialFunction.condOpt

        given intListOrdering: Ordering[List[Int]] with
            def compare(as: List[Int], bs: List[Int]): Int = 
                as.sum.compare(bs.sum)

        case class Result[T](prev: Option[T] = None, count: Int = 0)(using ordered: Ordering[T]):
            def +(value: T): Result[T] = 
                condOpt(prev) {
                    case Some(p) if ordered.compare(p, value) < 0 =>
                        copy(Option(value), count + 1)
                }.getOrElse(copy(prev=Option(value)))
        
        
        def part1(): Int =
            withSource(1, source => 
                largeThanPreviousCount(source.getLines.map(_.trim.toInt)))
            
        /*  
            https://adventofcode.com/2021/day/
        */
        def largeThanPreviousCount[T](iter: Iterator[T])(using ordered: Ordering[T]): Int =
            iter.foldLeft(Result())(_ + _).count
        
        def part2() =
            withSource(1, source => 
                largeThanPreviousCount[List[Int]](
                    source
                        .getLines
                        .map(_.trim.toInt).sliding(3).map(_.toList)))

    end day1

    package day2:
    
        enum Direction:
            case Up, Down, Forward

        case class Command(direction: Direction, steps: Int)
        
        case class Location(depth: Int = 0, horizontal: Int = 0):
            def +(command: Command): Location =
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
                text.map(line => {
                    line.trim.split(" ") match 
                        case Array(d, c) =>
                            val Some(directon) = Direction.values
                                .find(_.productPrefix.toLowerCase == d)
                                .ensuring(_.isDefined, s"could not map $d to a Direction")
                            Command(directon, c.toInt)
                })

        def part1(): Int = 
            utils.withSource(2, { source => 
                val commands = Command(source.getLines)
                location(commands)
            })

        def location(commands: Iterator[Command]): Int = 
            val result = commands.foldLeft(Location())(_ + _)
            result.depth * result.horizontal
            

    end day2

    
end y2021