package interview;

import java.util.ArrayList;
import java.util.Collections;
import java.util.List;
import java.util.Set;

/**
 * The classic solution.
 */
public class BinarySearchSort implements Renderer {
  public CharSequence render(CharSequence text, Set<Entity> entities) {
    int total = text.length();
    int size = entities.size();
    List<Entity> list = new ArrayList<Entity>(size);
    for (Entity e : entities) {
      int current = list.size();
      if (current == 0) {
        list.add(e);
      } else if (current == 1) {
        if (e.start < list.get(0).start) {
          list.add(0, e);
        } else {
          list.add(e);
        }
      } else {
        list.add(-(Collections.binarySearch(list, e) + 1), e);
      }
    }
    StringBuilder sb = new StringBuilder(total);
    int pos = 0;
    for (int i = 0; i < size; i++) {
      Entity entity = list.get(i);
      sb.append(text.subSequence(pos, entity.start));
      sb.append(entity.html);
      pos = entity.end;
    }
    sb.append(text.subSequence(pos, text.length()));
    return sb;
  }
}
