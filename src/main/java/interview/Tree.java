package interview;

import java.util.Set;
import java.util.TreeSet;

/**
 * The classic solution.
 */
public class Tree implements Renderer {
  public CharSequence render(CharSequence text, Set<Entity> entities) {
    StringBuilder sb = new StringBuilder(256);
    TreeSet<Entity> tree = new TreeSet<Entity>(entities);
    int pos = 0;
    for (Entity entity : tree) {
      sb.append(text.subSequence(pos, entity.start));
      sb.append(entity.html);
      pos = entity.end;
    }
    sb.append(text.subSequence(pos, text.length()));
    return sb;
  }
}
