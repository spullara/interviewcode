package interview;

import java.util.Arrays;
import java.util.Set;

/**
 * The classic solution.
 */
public class OptimizedClassicWithCodePoints implements Renderer {
  public CharSequence render(CharSequence text, Set<Entity> entities) {
    StringBuilder sb = new StringBuilder(256);
    Entity[] array = entities.toArray(new Entity[entities.size()]);
    Arrays.sort(array);
    String s = text.toString();
    int pos = 0;
    int codePointPosition = 0;
    for (Entity entity : array) {
      int start = s.offsetByCodePoints(pos, entity.start - codePointPosition);
      sb.append(s, pos, start);
      sb.append(entity.html);
      codePointPosition = entity.end;
      pos = s.offsetByCodePoints(start, entity.end - entity.start);
    }
    sb.append(text, pos, text.length());
    return sb;
  }
}
