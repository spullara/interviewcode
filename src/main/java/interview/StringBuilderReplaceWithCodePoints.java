package interview;

import java.util.Arrays;
import java.util.Set;

public class StringBuilderReplaceWithCodePoints implements Renderer {

  public CharSequence render(CharSequence text, Set<Entity> entities) {
    var array = entities.toArray(new Entity[0]);
    Arrays.sort(array, (o1, o2) -> o2.start - o1.start);
    var sb = new StringBuilder(text.length() * 2).append(text);
    var s = text.toString();
    var pos = 0;
    var codePointPosition = 0;
    for (Entity entity : array) {
      var start = s.offsetByCodePoints(pos, entity.start - codePointPosition);
      var end = s.offsetByCodePoints(pos, entity.end - codePointPosition);
      sb.replace(start, end, entity.html.toString());
      codePointPosition = entity.end;
      pos = end;
    }
    return sb;
  }
}
