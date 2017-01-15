package interview;

import java.util.Arrays;
import java.util.Set;

/**
 * The classic solution.
 */
public class StringBuilderReplace implements Renderer {

  public CharSequence render(CharSequence text, Set<Entity> entities) {
    Entity[] array = entities.toArray(new Entity[entities.size()]);
    Arrays.sort(array, (o1, o2) -> o2.start - o1.start);
    StringBuilder sb = new StringBuilder(text.length() * 2).append(text);
    for (Entity entity : array) {
      sb.replace(entity.start, entity.end, entity.html.toString());
    }
    return sb;
  }
}
