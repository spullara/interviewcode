package interview;

import java.util.Set;

public class ArrayScan implements Renderer {
  @Override
  public CharSequence render(CharSequence text, Set<Entity> entities) {
    int length = text.length();
    Entity[] map = new Entity[length];
    int total = length;
    for (Entity entity : entities) {
      map[entity.start] = entity;
      total += entity.html.length() - (entity.end - entity.start);
    }
    StringBuilder sb = new StringBuilder(total);
    for (int i = 0; i < length; ) {
      Entity entity = map[i];
      if (entity == null) {
        sb.append(text.charAt(i++));
      } else {
        sb.append(entity.html);
        i = entity.end;
      }
    }
    return sb;
  }
}
