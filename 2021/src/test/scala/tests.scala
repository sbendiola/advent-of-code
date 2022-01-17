import org.junit.Test
import org.junit.Assert.*

val sampleData = (List(
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
      ), 
      List(
        "00100",
        "11110",
        "10110",
        "10111",
        "10101",
        "01111",
        "00111",
        "11100",
        "10000",
        "11001",
        "00010",
        "01010",
      ))


class TestDay1:

  @Test 
  def part1(): Unit = 
    assertEquals(7, day1.largeThanPreviousCount(sampleData(0).iterator))
    assertEquals(1832, day1.part1())

  @Test 
  def part2(): Unit = 
    import day1.intListOrdering
    assertEquals(5, day1.largeThanPreviousCount(sampleData(0).sliding(3)))
    assertEquals(1858, day1.part2())

class Day2Tests:
  @Test 
  def part1(): Unit = 
    val commandText = sampleData(1)
    assertEquals(150, day2.location(day2.Command(commandText.iterator)))
    assertEquals(1524750, day2.part1())


  @Test 
  def part2_simple(): Unit = 
    val commandText = sampleData(1)
    assertEquals(900, day2.location(day2.Command(commandText.iterator), day2.LocationWithAim()))
    
  @Test 
  def part2_real_data(): Unit = 
    val commandText = sampleData(1)
    assertEquals(1592426537, day2.part2())

class Day3Tests:
  @Test
  def sample(): Unit =
    val id = day3.id
    assertEquals(22, day3.gamma(sampleData(2).iterator))
    assertEquals(9, day3.epsilon(sampleData(2).iterator)) 
    assertEquals(198, day3.powerConsumption(sampleData(2).iterator)) 
    utils.withTestData(id) {
      source =>
        //part1
        assertEquals(3895776, day3.powerConsumption(source.getLines))
    }
    //part2 sample
    assertEquals(23, day3.oxygenGenRating(sampleData(2).iterator)) 
    assertEquals(10, day3.co2Rating(sampleData(2).iterator))
    utils.withTestData(id) {
      source =>
        //part2
        assertEquals(7928162, day3.lifeSupportRating(source.getLines.toList))
    }

    
    