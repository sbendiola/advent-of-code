package y2021:
    
    import java.nio.file.{Paths, Path, Files}
    import scala.io.Source

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
        
        def withSource[R](f: Source => R):R =
            using(relativeResource("day_1_input"))
                (_.close)
                (f)

        
        def part1(): Int =
            withSource(source => 
                largeThanPreviousCount(source.getLines.map(_.trim.toInt)))
            
        /*  
            https://adventofcode.com/2021/day/
        */
        def largeThanPreviousCount[T](iter: Iterator[T])(using ordered: Ordering[T]): Int =
            iter.foldLeft(Result())(_ + _).count
        
        def part2() =
            withSource(source => 
                largeThanPreviousCount[List[Int]](
                    source
                        .getLines
                        .map(_.trim.toInt).sliding(3).map(_.toList)))

    end day1
    
end y2021