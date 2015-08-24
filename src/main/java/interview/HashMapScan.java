package interview;

import java.util.HashMap;
import java.util.Map;
import java.util.Set;

public class HashMapScan implements Renderer {
  @Override
  public CharSequence render(CharSequence text, Set<Entity> entities) {
    Map<Integer, Entity> map = new HashMap<Integer, Entity>();
    int length = text.length();
    int total = length;
    for (Entity entity : entities) {
      map.put(entity.start, entity);
      total += entity.html.length() - (entity.end - entity.start);
    }
    StringBuilder sb = new StringBuilder(total);
    for (int i = 0; i < length; ) {
      Entity entity = map.get(i);
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
