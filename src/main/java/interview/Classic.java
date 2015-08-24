package interview;

import java.util.ArrayList;
import java.util.Arrays;
import java.util.Collections;
import java.util.List;
import java.util.Set;

/**
 * The classic solution.
 */
public class Classic implements Renderer {
  public CharSequence render(CharSequence text, Set<Entity> entities) {
    StringBuilder sb = new StringBuilder();
    List<Entity> array = new ArrayList<Entity>(entities);
    Collections.sort(array);
    int pos = 0;
    for (Entity entity : array) {
      sb.append(text.subSequence(pos, entity.start));
      sb.append(entity.html);
      pos = entity.end;
    }
    sb.append(text.subSequence(pos, text.length()));
    return sb;
  }
}
