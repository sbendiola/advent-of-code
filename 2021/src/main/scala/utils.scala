package utils

import java.nio.file.{Paths, Path, Files}
import scala.io.Source
import javax.print.attribute.standard.PresentationDirection

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

def withTestData[R](day: Int)(f: Source => R):R =
    using(relativeResource(s"day_${day}_input"))
        (_.close)
        (f)
