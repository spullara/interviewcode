package interview;

import java.util.Collections;
import java.util.LinkedList;
import java.util.List;
import java.util.Set;

/**
 * Inspired by the binary tree version. Left and right linearized.
 */
public class LinkedListEntities implements Renderer {

  @Override
  public CharSequence render(CharSequence text, Set<Entity> entities) {
    int size = 2 * entities.size() + 1;
    if (size == 1) return text;
    List<Entity> list = new LinkedList<Entity>();
    boolean first = true;
    int length = text.length();
    for (Entity e : entities) {
      length += e.html.length() - (e.end - e.start);
      if (first) {
        // First one is easy
        list.add(new Entity(0, e.start, null));
        list.add(e);
        list.add(new Entity(e.end, text.length(), null));
        first = false;
      } else {
        // Find the array index
        int i = Collections.binarySearch(list, e);
        if (i < 0) i = -i - 2;
        Entity current = list.get(i);
        list.add(i + 1, e);
        list.add(i + 2, new Entity(e.end, current.end, null));
        current.end = e.start;
      }
    }
    StringBuilder sb = new StringBuilder(length);
    for (Entity e : list) {
      if (e.html == null) {
        sb.append(text, e.start, e.end);
      } else {
        sb.append(e.html);
      }
    }
    return sb.toString();
  }
}
