package aoc

import org.junit.Test
import org.junit.Assert.*
import aoc.*

class Test2021:
  val data = (List(
      199,
      200,
      208,
      210,
      200,
      207,
      240,
      269,
      260,
      263), 
      List(
        "forward 5",
        "down 5",
        "forward 8",
        "up 3",
        "down 8",
        "forward 2"
      ))
  
  @Test 
  def day1_part1(): Unit = 
    assertEquals(7, day1.largeThanPreviousCount(data(0).iterator))
    assertEquals(1832, day1.part1())

  @Test 
  def day1_part2(): Unit = 
    import day1.intListOrdering
    assertEquals(5, day1.largeThanPreviousCount(data(0).sliding(3)))
    assertEquals(1858, day1.part2())

  @Test 
  def day2_part1(): Unit = 
    val commandText = data(1)
    assertEquals(150, day2.location(Command(commandText.iterator)))
    assertEquals(1524750, day2.part1())


  @Test 
  def day2_part2_simple(): Unit = 
    val commandText = data(1)
    assertEquals(900, day2.location(Command(commandText.iterator), day2.LocationWithAim()))
    
  @Test 
  def day2_part2_real_data(): Unit = 
    val commandText = data(1)
    assertEquals(1592426537, day2.part2())
