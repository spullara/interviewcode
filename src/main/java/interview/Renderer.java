package interview;

import java.util.Set;

public interface Renderer {
  CharSequence render(CharSequence text, Set<Entity> entities);
}
