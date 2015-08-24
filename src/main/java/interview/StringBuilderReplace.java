package interview;

import java.util.Arrays;
import java.util.Comparator;
import java.util.Set;

/**
 * The classic solution.
 */
public class StringBuilderReplace implements Renderer {
  private static Comparator<Entity> reverse = new Comparator<Entity>() {
    @Override
    public int compare(Entity o1, Entity o2) {
      return o2.start - o1.start;
    }
  };

  public CharSequence render(CharSequence text, Set<Entity> entities) {
    Entity[] array = entities.toArray(new Entity[entities.size()]);
    Arrays.sort(array, reverse);
    StringBuilder sb = new StringBuilder(text);
    for (Entity entity : array) {
      sb.replace(entity.start, entity.end, entity.html.toString());
    }
    return sb;
  }
}
