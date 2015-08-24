package interview;

import java.util.Comparator;

public class Entity implements Comparable<Entity>, Comparator<Entity> {
  Entity(int start, int end, CharSequence html) {
    this.start = start;
    this.end = end;
    this.html = html;
  }
  public int start;
  public int end;
  public CharSequence html;

  @Override
  public int compareTo(Entity o) {
    return start - o.start;
  }

  @Override
  public int compare(Entity o1, Entity o2) {
    return o1.start - o2.start;
  }
}
