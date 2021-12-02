package y2021

  import org.junit.Test
  import org.junit.Assert.*

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
        263), ())
    
    @Test 
    def day1_part1(): Unit = 
      assertEquals(7, day1.largeThanPreviousCount(data(0).iterator))
      assertEquals(1832, day1.part1())

    @Test 
    def day1_part2(): Unit = 
      import y2021.day1.intListOrdering
      assertEquals(5, day1.largeThanPreviousCount(data(0).sliding(3)))
      assertEquals(1858, day1.part2())
