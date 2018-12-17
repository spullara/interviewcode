package interview;

import org.junit.BeforeClass;
import org.junit.Test;

import java.lang.management.ManagementFactory;
import java.lang.management.MemoryMXBean;
import java.util.*;


public class RendererBenchmarkTest {
  private static List<Set<Entity>> entitiesList;
  private static String text = "Attend to hear 6 stellar #mobile #startups at #OF12 Entrepreneur Idol show 2day,  " +
          "http://t.co/HtzEMgAC @TiEcon @sv_entrepreneur @500!";


  @BeforeClass
  public static void setup() {
    entitiesList = createEntriesList();
  }

  @Test
  public void testClassic() throws Exception {
    Renderer renderer = new Classic();
    bench(renderer);
    memory(renderer);
  }

  @Test
  public void testOptimizedClassic() throws Exception {
    Renderer renderer = new OptimizedClassic();
    bench(renderer);
    memory(renderer);
  }

  @Test
  public void testOptimizedClassicWithCodePoints() throws Exception {
    Renderer renderer = new OptimizedClassicWithCodePoints();
    bench(renderer);
    memory(renderer);
  }

  @Test
  public void testBinaryTree() throws Exception {
    Renderer renderer = new BinaryTree();
    bench(renderer);
    memory(renderer);
  }

  @Test
  public void testLinearNodes() throws Exception {
    Renderer renderer = new LinkedListEntities();
    bench(renderer);
    memory(renderer);
  }

  @Test
  public void testInsertionSort() throws Exception {
    Renderer renderer = new InsertionSort();
    bench(renderer);
    memory(renderer);
  }

  @Test
  public void testBinarySearchSort() throws Exception {
    Renderer renderer = new BinarySearchSort();
    bench(renderer);
    memory(renderer);
  }

  @Test
  public void testTree() throws Exception {
    Renderer renderer = new Tree();
    bench(renderer);
    memory(renderer);
  }

  @Test
  public void testStringReplacement() throws Exception {
    Renderer renderer = new StringReplacement();
    bench(renderer);
    memory(renderer);
  }

  @Test
  public void testStringBuilderReplace() throws Exception {
    Renderer renderer = new StringBuilderReplace();
    bench(renderer);
    memory(renderer);
  }

  @Test
  public void testStringBuilderReplaceWithCodePoints() throws Exception {
    Renderer renderer = new StringBuilderReplaceWithCodePoints();
    bench(renderer);
    memory(renderer);
  }

  @Test
  public void testHashMapScan() throws Exception {
    Renderer renderer = new HashMapScan();
    bench(renderer);
    memory(renderer);
  }

  @Test
  public void testArrayScan() throws Exception {
    Renderer renderer = new ArrayScan();
    bench(renderer);
    memory(renderer);
  }

  private void bench(Renderer renderer) {
    String text = "Attend to hear 6 stellar #mobile #startups at #OF12 Entrepreneur Idol show 2day,  " +
            "http://t.co/HtzEMgAC @TiEcon @sv_entrepreneur @500!";
    List<Set<Entity>> entitiesList = createEntriesList();

    {
      for (int j = 0; j < 5; j++) {
        for (int i = 0; i < 10000; i++) {
          renderer.render(text, entitiesList.get(i % 1000));
        }
        long start = System.currentTimeMillis();
        for (int i = 0; i < 1000000; i++) {
          renderer.render(text, entitiesList.get(i % 1000));
        }
        System.out.println(renderer.getClass().getSimpleName() + ": " + (System.currentTimeMillis() - start) + " ns/op");
      }
    }
  }

  private void memory(Renderer renderer) {
    System.gc();
    MemoryMXBean mxbean = ManagementFactory.getMemoryMXBean();
    {
      long total = 0;
      for (int i = 0; i < 1000000; i++) {
        long start = mxbean.getHeapMemoryUsage().getUsed();
        renderer.render(text, entitiesList.get(i % 1000));
        long end = mxbean.getHeapMemoryUsage().getUsed();
        long diff = end - start;
        if (diff > 0) {
          total += diff;
        }
      }
      System.out.println("Memory: " + renderer.getClass().getSimpleName() + ": " + total / 1000000 + " bytes/op");
    }

  }

  private static List<Set<Entity>> createEntriesList() {
    Random r = new Random(938471093847L);
    List<Set<Entity>> entitiesList = new ArrayList<Set<Entity>>();
    for (int i = 0; i < 1000; i++) {
      Set<Entity> entities = new HashSet<Entity>();
      int total = r.nextInt(10);
      List<Integer> indices = new ArrayList<Integer>();
      for (int j = 0; j < total * 2; j++) {
        int next;
        while (indices.contains(next = r.nextInt(text.length()))) ;
        indices.add(next);
      }
      Collections.sort(indices);
      for (int j = 0; j < total * 2; j += 2) {
        int start = indices.get(j);
        int end = indices.get(j + 1);
        int length = end - start;
        StringBuilder sb = new StringBuilder(length * 2);
        for (int k = 0; k < length; k++) {
          sb.append("XX");
        }
        entities.add(new Entity(start, end, sb.toString()));
      }
      entitiesList.add(entities);
    }
    return entitiesList;
  }
}
