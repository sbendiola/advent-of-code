package y2021

  import org.junit.Test
  import org.junit.Assert.*

  class Test2021:

    @Test def test_day1(): Unit = 

      assertEquals(7, day1.largeThanPreviousCount(List(
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
      ).iterator))

      assertEquals(1832, day1.run())
