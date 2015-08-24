package interview;

import org.junit.Test;

import java.util.HashSet;
import java.util.Set;

import static junit.framework.Assert.assertEquals;


public class RendererTest {
  @Test
  public void testClassic() throws Exception {
    Renderer renderer = new Classic();
    test(renderer);
  }

  @Test
  public void testBinaryTree() throws Exception {
    Renderer renderer = new BinaryTree();
    test(renderer);
  }

  @Test
  public void testInsertionSort() throws Exception {
    Renderer renderer = new InsertionSort();
    test(renderer);
  }

  @Test
  public void testBinarySearchSort() throws Exception {
    Renderer renderer = new BinarySearchSort();
    test(renderer);
  }

  @Test
  public void testTree() throws Exception {
    Renderer renderer = new Tree();
    test(renderer);
  }

  @Test
  public void testStringReplacement() throws Exception {
    Renderer renderer = new StringReplacement();
    test(renderer);
  }

  @Test
  public void testStringBuilderReplace() throws Exception {
    Renderer renderer = new StringBuilderReplace();
    test(renderer);
  }

  @Test
  public void testLinearNodes() throws Exception {
    Renderer renderer = new LinkedListEntities();
    test(renderer);
  }


  private void test(Renderer renderer) {
    String text = "Attend to hear 6 stellar #mobile #startups at #OF12 Entrepreneur Idol show 2day,  " +
            "http://t.co/HtzEMgAC @TiEcon @sv_entrepreneur @500!";
    assertEquals(text, renderer.render(text, new HashSet<Entity>()).toString());

    String result = "Attend to hear 6 stellar <#mobile> <#startups> at <#OF12> Entrepreneur Idol show 2day,  " +
            "<http://t.co/HtzEMgAC> <@TiEcon> <@sv_entrepreneur> <@500>!";
    Set<Entity> entities = new HashSet<Entity>() {{
      add(new Entity(25, 32, "<#mobile>"));
      add(new Entity(33, 42, "<#startups>"));
      add(new Entity(46, 51, "<#OF12>"));
      add(new Entity(82, 102, "<http://t.co/HtzEMgAC>"));
      add(new Entity(103, 110, "<@TiEcon>"));
      add(new Entity(111, 127, "<@sv_entrepreneur>"));
      add(new Entity(128, 132, "<@500>"));
    }};
    assertEquals(result, renderer.render(text, entities).toString());
  }
}
