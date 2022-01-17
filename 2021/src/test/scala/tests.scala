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
  import day1.*
  val simpleData = sampleData(0)
  @Test 
  def part1SampleData() = 
    assertEquals(7, largeThanPreviousCount(simpleData.iterator))
    
  @Test
  def part1TestData() = 
    assertEquals(1832, utils.withTestData(id) { source =>
      largeThanPreviousCount(source.getLines.map(_.trim.toInt))
    })

  @Test 
  def part2() = 
    import day1.intListOrdering
    val value = utils.withTestData(id) { source => 
      largeThanPreviousCount[List[Int]](
                source
                    .getLines
                    .map(_.trim.toInt).sliding(3).map(_.toList))
    }
    assertEquals(1858, value)

class Day2Tests:
  import day2.*
  val commandText = sampleData(1)

  @Test 
  def part1SampleData() = 
    assertEquals(150, location(Command(commandText.iterator)))

  @Test
  def part1TestData2() =
    val value = utils.withTestData(id) { source => location(Command(source.getLines))}
    assertEquals(1524750, value)

  @Test 
  def part2_simple() = 
    assertEquals(900, location(Command(commandText.iterator), LocationWithAim()))
      
  @Test 
  def part2_real_data() = 
    assertEquals(1592426537, utils.withTestData(id) { source => 
      location(Command(source.getLines), LocationWithAim())
    })

class Day3Tests:
  import day3.*
  @Test
  def sample() =
    assertEquals(22, gamma(sampleData(2).iterator))
    assertEquals(9, epsilon(sampleData(2).iterator)) 
    assertEquals(198, powerConsumption(sampleData(2).iterator)) 
    utils.withTestData(id) {
      source =>
        //part1
        assertEquals(3895776, powerConsumption(source.getLines))
    }
    //part2 sample
    assertEquals(23, oxygenGenRating(sampleData(2).iterator)) 
    assertEquals(10, co2Rating(sampleData(2).iterator))
    utils.withTestData(id) {
      source =>
        //part2
        assertEquals(7928162, lifeSupportRating(source.getLines.toList))
    }

    
    