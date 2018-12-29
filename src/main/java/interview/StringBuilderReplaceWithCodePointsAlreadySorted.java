package interview;

import java.util.Arrays;
import java.util.Set;

/**
 * The classic solution.
 */
public class StringBuilderReplaceWithCodePointsAlreadySorted implements Renderer {

  public CharSequence render(CharSequence text, Set<Entity> entities) {
    StringBuilder sb = new StringBuilder(text.length() * 2).append(text);
    String s = text.toString();
    int pos = 0;
    int codePointPosition = 0;
    for (Entity entity : entities) {
      int start = s.offsetByCodePoints(pos, entity.start - codePointPosition);
      int end = s.offsetByCodePoints(pos, entity.end - codePointPosition);
      sb.replace(start, end, entity.html.toString());
      codePointPosition = entity.end;
      pos = end;
    }
    return sb;
  }
}
