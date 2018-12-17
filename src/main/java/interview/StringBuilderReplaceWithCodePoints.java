package interview;

import java.util.Arrays;
import java.util.Set;

/**
 * The classic solution.
 */
public class StringBuilderReplaceWithCodePoints implements Renderer {

  public CharSequence render(CharSequence text, Set<Entity> entities) {
    Entity[] array = entities.toArray(new Entity[0]);
    Arrays.sort(array, (o1, o2) -> o2.start - o1.start);
    StringBuilder sb = new StringBuilder(text.length() * 2).append(text);
    String s = text.toString();
    int pos = 0;
    int codePointPosition = 0;
    for (Entity entity : array) {
      int start = s.offsetByCodePoints(pos, entity.start - codePointPosition);
      int end = s.offsetByCodePoints(pos, entity.end - codePointPosition);
      sb.replace(start, end, entity.html.toString());
      codePointPosition = entity.end;
      pos = end;
    }
    return sb;
  }
}
