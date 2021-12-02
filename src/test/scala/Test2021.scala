package y2021

  import org.junit.Test
  import org.junit.Assert.*

  class Test2021:
    val data = List(
        199,
        200,
        208,
        210,
        200,
        207,
        240,
        269,
        260,
        263
      )
    @Test def test_day1_part1(): Unit = 

      assertEquals(7, day1.largeThanPreviousCount(data.iterator, (p, v) => v > p))

      assertEquals(1832, day1.run())

    @Test def test_day1_part2(): Unit = 
      assertEquals(5, day1.largeThanPreviousCount(data.sliding(3), _.sum < _.sum))
      assertEquals(1858, day1.run2())
