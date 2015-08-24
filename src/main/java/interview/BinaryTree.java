package interview;

import java.util.Set;

/**
 * Rongrong Zhong
 */
public class BinaryTree implements Renderer {

  @Override
  public CharSequence render(CharSequence text, Set<Entity> entities) {
    int size = text.length();
    // Root is the full string initially
    Node root = new Node();
    root.start = 0;
    root.end = size;

    // Loop over the entities, finding them in the tree, and inserting them
    for (Entity entity : entities) {
      insert(root, entity);
      size += (entity.end - entity.start + entity.html.length());
    }

    // In-order traverse the tree to generate the output
    StringBuilder sb = new StringBuilder(size);
    traverse(text, root, sb);
    return sb;
  }

  void insert(Node node, Entity entity) {
    // Is the node on the left?
    if (entity.start <= node.start) {
      if (node.left != null) {
        // More nodes to the left
        insert(node.left, entity);
      } else {
        // Create the new node
        Node newnode = createNode(entity);
        // Split the current node
        if (node.text != null) {
          throw new AssertionError("Overlapping entities");
        }
        node.start = newnode.end;
        node.left = newnode;
      }
      // Is the node on the right?
    } else if (entity.start <= node.end) {
      // Create the new node
      Node newnode = createNode(entity);
      // Split the current node
      if (node.text != null) {
        throw new AssertionError("Overlapping entities");
      }
      if (entity.end <= node.end) {
        if (entity.end < node.end) {
          // Make a new right node
          Node right = new Node();
          newnode.right = right;
          right.start = newnode.end;
          right.end = node.end;
        }
        // Make the left node
        Node left = new Node();
        newnode.left = left;
        left.start = node.start;
        left.end = newnode.start;
        // Replace the parent
        node.text = newnode.text;
        node.start = newnode.start;
        node.end = newnode.end;
        node.left = newnode.left;
        node.right = newnode.right;
      } else {
        throw new AssertionError("Overlapping nodes");
      }
    } else if (node.right != null) {
      insert(node.right, entity);
    } else {
      throw new AssertionError("Invalid tree");
    }
  }

  private Node createNode(Entity entity) {
    Node newnode = new Node();
    newnode.text = entity.html;
    newnode.start = entity.start;
    newnode.end = entity.end;
    return newnode;
  }

  void traverse(CharSequence text, Node node, StringBuilder sb) {
    if (node.left != null) {
      traverse(text, node.left, sb);
    }
    if (node.text == null) {
      sb.append(text.subSequence(node.start, node.end));
    } else {
      sb.append(node.text);
    }
    if (node.right != null) {
      traverse(text, node.right, sb);
    }
  }
}

class Node {
  // Either the text or a reference into the string
  CharSequence text;
  int start;
  int end;
  Node left;
  Node right;
}