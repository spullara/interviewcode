package interview;

import java.util.Arrays;
import java.util.Comparator;
import java.util.Set;

/**
 * The classic solution.
 */
public class StringReplacement implements Renderer {
  private static Comparator<Entity> reverse = new Comparator<Entity>() {
    @Override
    public int compare(Entity o1, Entity o2) {
      return o2.start - o1.start;
    }
  };

  public CharSequence render(CharSequence text, Set<Entity> entities) {
    Entity[] array = entities.toArray(new Entity[entities.size()]);
    Arrays.sort(array, reverse);
    for (Entity entity : array) {
      text = text.subSequence(0, entity.start) + entity.html.toString() + text.subSequence(entity.end, text.length());
    }
    return text;
  }
}
