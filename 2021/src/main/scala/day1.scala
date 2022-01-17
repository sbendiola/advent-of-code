import PartialFunction.condOpt

object day1 extends BaseDay(1):

    given intListOrdering: Ordering[List[Int]] with
        def compare(as: List[Int], bs: List[Int]): Int = 
            as.sum.compare(bs.sum)

    case class Result[T](prev: Option[T] = None, count: Int = 0)(using ordered: Ordering[T]):
        def +(value: T): Result[T] = 
            condOpt(prev) {
                case Some(p) if ordered.compare(p, value) < 0 =>
                    copy(Option(value), count + 1)
            }.getOrElse(copy(prev=Option(value)))


    def largeThanPreviousCount[T](iter: Iterator[T])(using ordered: Ordering[T]): Int =
        iter.foldLeft(Result())(_ + _).count
