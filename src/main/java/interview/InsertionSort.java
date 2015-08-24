package interview;

import java.util.Set;

/**
 * The classic solution.
 */
public class InsertionSort implements Renderer {
  public CharSequence render(CharSequence text, Set<Entity> entities) {
    int total = text.length();
    int size = entities.size();
    Entity[] array = new Entity[size];
    int i = 0;
    for (Entity entity : entities) {
      total += (entity.end - entity.start + entity.html.length());
      int j;
      for (j = 0; j < i; j++) {
        if (entity.start < array[j].start) {
          System.arraycopy(array, j, array, j + 1, size - j - 1);
          break;
        }
      }
      array[j] = entity;
      i++;
    }
    StringBuilder sb = new StringBuilder(total);
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
