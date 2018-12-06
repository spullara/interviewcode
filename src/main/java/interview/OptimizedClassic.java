package interview;

import java.util.Arrays;
import java.util.Set;

/**
 * The classic solution.
 */
public class OptimizedClassic implements Renderer {
  public CharSequence render(CharSequence text, Set<Entity> entities) {
    Entity[] array = new Entity[entities.size()];
    int capacity = text.length();
    int i = 0;
    for (Entity entity : entities) {
      capacity += entity.html.length() - (entity.end - entity.start);
      array[i++] = entity;
    }
    StringBuilder sb = new StringBuilder(capacity);
    Arrays.sort(array);
    int pos = 0;
    for (Entity entity : array) {
      sb.append(text, pos, entity.start).append(entity.html);
      pos = entity.end;
    }
    sb.append(text, pos, text.length());
    return sb;
  }
}
