object day3 extends BaseDay(3):
    case class Result(occurences: Map[Int, Map[Char, Int]] = Map.empty, inputs: List[String] = Nil):
        def +(text: String): Result = 
            val updated = text.zipWithIndex.foldLeft(occurences) {
                case (acc, (c, i)) =>
                    val countsByIndex = acc.getOrElse(i, Map.empty)
                    val currentValue = countsByIndex.getOrElse(c, 0) 
                    acc.updated(i, countsByIndex.updated(c, currentValue+1))
            }
            copy(occurences=updated, inputs = text :: inputs)

        def gamma =
            val digits = occurences.toList.sortBy(_._1)
                            .map(entry => entry._2.maxBy(_._2)._1)
                            .mkString("")
            Integer.parseInt(digits, 2)
        
        def epsilon =
            val digits = occurences.toList.sortBy(_._1)
                            .map(entry => entry._2.minBy(_._2)._1)
                            .mkString("")
            Integer.parseInt(digits, 2)
        
        def powerConsumption = 
            gamma * epsilon

        private
        def filterAtIndex(index: Int, candidates: List[String], f: Map[Char, Int] => (Char, Int), default: Char) = {
            if (candidates.size == 1) {
                candidates
            } else {
                val counts = candidates.foldLeft(Result()) (_ + _)
                val countsByChar = counts.occurences(index)
                val (char: Char, count) = f(countsByChar)
                val validChar = char match        
                    case _ if countsByChar.values.filter(_ == count).size > 1 =>
                        default
                    case c =>
                        c
                candidates.filter(_.charAt(index) == validChar)                
            }
        }
    
        def oxygenGenRating = 

            val remaining = (0 until inputs.head.length).foldLeft(inputs) {
                case (remaining, index) =>
                    filterAtIndex(index, remaining, default='1', f =_.toList.sortBy(_._2).reverse.head)
            }

            remaining.ensuring(_.size == 1, remaining)
            Integer.valueOf(remaining.head, 2)
        
        def co2Rating = 

            val remaining = (0 until inputs.head.length).foldLeft(inputs) {
                case (remaining, index) =>
                    filterAtIndex(index, remaining, default = '0', f =_.toList.sortBy(_._2).head)
            }

            remaining.ensuring(_.size == 1, remaining)
            Integer.valueOf(remaining.head, 2)

    def gamma(input: Iterator[String]) =
        //most common bit for each index
        input.foldLeft(Result())(_ + _).gamma
        
    def epsilon(input: Iterator[String]) =   
        input.foldLeft(Result())(_ + _).epsilon
        
    def powerConsumption(input: Iterator[String]) =   
        input.foldLeft(Result())(_ + _).powerConsumption

    def oxygenGenRating(input: Iterator[String]) =
        input.foldLeft(Result())(_ + _).oxygenGenRating

    def co2Rating(input: Iterator[String]) =
        input.foldLeft(Result())(_ + _).co2Rating
        
    def lifeSupportRating(input: List[String]) =
        val result = input.foldLeft(Result())(_ + _)
        result.co2Rating * result.oxygenGenRating

