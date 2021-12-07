package day3

import utils.*
import aoc.*

case class Result(occurences: Map[Int, Map[Char, Int]] = Map.empty):
    def +(text: String): Result = 
        val updated = text.zipWithIndex.foldLeft(occurences) {
            case (acc, (c, i)) =>
                val countsByIndex = acc.getOrElse(i, Map.empty)
                val currentValue = countsByIndex.getOrElse(c, 0) 
                acc.updated(i, countsByIndex.updated(c, currentValue+1))
        }
        copy(occurences=updated)

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

def gamma(input: Iterator[String]) =
    //most common bit for each index
    input.foldLeft(Result())(_ + _).gamma
    
def epsilon(input: Iterator[String]) =   
    input.foldLeft(Result())(_ + _).epsilon
    
def powerConsumption(input: Iterator[String]) =   
    input.foldLeft(Result())(_ + _).powerConsumption
