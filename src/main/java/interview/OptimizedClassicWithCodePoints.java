package interview;

import java.util.ArrayList;
import java.util.Collections;
import java.util.Set;

/**
 * The classic solution.
 */
public class OptimizedClassicWithCodePoints implements Renderer {
  public CharSequence render(CharSequence text, Set<Entity> entities) {
    var list = new ArrayList<>(entities);
    Collections.sort(list);
    var sb = new StringBuilder(text.length() * 2);
    var s = text.toString();
    var pos = 0;
    var codePointPosition = 0;
    for (var entity : list) {
      var start = s.offsetByCodePoints(pos, entity.start - codePointPosition);
      sb.append(s, pos, start);
      sb.append(entity.html);
      codePointPosition = entity.end;
      pos = s.offsetByCodePoints(start, entity.end - entity.start);
    }
    sb.append(text, pos, text.length());
    return sb;
  }
}
