package interview;

import java.util.*;

/**
 * The classic solution.
 */
public class OptimizedClassicWithCodePoints implements Renderer {
  public CharSequence render(CharSequence text, Set<Entity> entities) {
    StringBuilder sb = new StringBuilder();
    List<Entity> list = new ArrayList<>(entities);
    Collections.sort(list);
    String s = text.toString();
    int pos = 0;
    int codePointPosition = 0;
    for (Entity entity : list) {
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
