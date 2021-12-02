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

        case class Result[T](prev: Option[T] = None, 
            greaterThanPrevious: Int = 0, 
            comparison: (T, T) => Boolean):
            def +(value: T): Result[T] = 
                condOpt(prev) {
                    case Some(p) if comparison(p, value) =>
                        copy(Option(value), greaterThanPrevious + 1)
                }.getOrElse(copy(prev=Option(value)))
        
        
        def run(): Int =
            using(relativeResource("day_1_input"))
                (_.close)
                (source => largeThanPreviousCount[Int](
                    source
                        .getLines
                        .map(_.trim.toInt), comparison = (p, v) => p < v))

        /*  
            https://adventofcode.com/2021/day/
        */
        def largeThanPreviousCount[T](iter: Iterator[T], comparison: (T, T) => Boolean): Int =
            iter.foldLeft(Result(comparison = comparison))(_ + _).greaterThanPrevious
        
        def run2() =
            using(relativeResource("day_1_input"))
                (_.close)
                (source => largeThanPreviousCount[List[Int]](
                    source
                        .getLines
                        .map(_.trim.toInt).sliding(3).map(_.toList), 
                        comparison = (p, v) => p.sum < v.sum))


    end day1
    
end y2021