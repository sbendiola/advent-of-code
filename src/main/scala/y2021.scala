package y2021:
    
    import java.nio.file.{Paths, Path, Files}
    import scala.io.Source

    def relativeResource(path: String*): Source =
        val location = Array("src", "main", "resources", "2021") ++ path
        val cd = Paths.get(".", location*).toAbsolutePath
        Source.fromFile(cd.ensuring(Files.exists(_), s"$cd does not exist").toFile)
    
    def using[T, R](t: => T)(cleanup: T => Unit)(use: T => R): R =
        lazy val resource = t
        try
            use(resource)
        finally
            cleanup(resource)
        

            
    package day1:
        
        import PartialFunction.condOpt

        case class Result(prev: Option[Int] = None, greaterThanPrevious: Int = 0):
            def +(value: Int): Result = 
                condOpt(prev) {
                    case Some(p) if value > p =>
                        copy(Option(value), greaterThanPrevious + 1)
                }.getOrElse(copy(prev=Option(value)))
        
        
        def run(): Int =
            using(relativeResource("day_1_input"))
                (_.close)
                (source => largeThanPreviousCount(
                    source
                        .getLines
                        .map(_.trim.toInt)))

        /*  
            https://adventofcode.com/2021/day/
        */
        def largeThanPreviousCount(iter: Iterator[Int]): Int =
            iter.foldLeft(Result())(_ + _).greaterThanPrevious

    end day1
    
end y2021