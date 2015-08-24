package interview;

import java.util.Arrays;
import java.util.Set;

/**
 * The classic solution.
 */
public class OptimizedClassic implements Renderer {
  public CharSequence render(CharSequence text, Set<Entity> entities) {
    StringBuilder sb = new StringBuilder(256);
    Entity[] array = entities.toArray(new Entity[entities.size()]);
    Arrays.sort(array);
    int pos = 0;
    for (Entity entity : array) {
      sb.append(text, pos, entity.start);
      sb.append(entity.html);
      pos = entity.end;
    }
    sb.append(text, pos, text.length());
    return sb;
  }
}
