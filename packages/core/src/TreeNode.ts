import { DisposeStack } from "./type";

class TreeNode<E extends Node = Node> {
  protected parent: TreeNode<E> | null = null;
  public readonly el: E | null = null;
  public readonly children: Set<TreeNode<E>> = new Set();
  protected disposeStack: DisposeStack = [];
  public staticContainer: boolean = false;

  constructor(params?: {
    node?: E;
    disposeStack?: DisposeStack;
    children?: Set<TreeNode<E>>;
    staticContainer?: boolean;
  }) {
    if (params?.node != null) {
      this.el = params.node;
    }
    if (params?.disposeStack != null) {
      this.disposeStack = params.disposeStack;
    }
    if (params?.children != null) {
      this.children = params.children;
      Array.from(params.children).forEach(
        (child) => child.el != null && this.el?.appendChild(child.el),
      );
    }
    if (params?.staticContainer != null) {
      this.staticContainer = params.staticContainer;
    }
  }

  public findAncestor = (
    callback: (n: TreeNode<E>) => boolean,
  ): TreeNode<E> | null => {
    if (callback(this)) return this;
    if (this.parent == null) {
      return null;
    }
    return this.parent.findAncestor(callback);
  };

  public findDescendants = (
    callback: (n: TreeNode<E>) => boolean,
  ): TreeNode<E>[] | null => {
    if (callback(this)) return [this];
    if (this.parent == null) {
      return null;
    }
    return [...this.children]
      .flatMap((child) => {
        return child.findDescendants(callback);
      })
      .filter((v): v is TreeNode<E> => v != null);
  };

  public mount = (child: TreeNode<E>, prev: TreeNode<E> | null = null) => {
    this.children.add(child);
    child.parent = this;
    const targetNode = this.findAncestor((node) => node.el != null);
    return targetNode?.mountElement(child, prev);
  };

  public mountElement = (
    child: TreeNode<E>,
    prev: TreeNode<E> | null = null,
  ) => {
    const targetNode = this.findAncestor((node) => node.el != null);

    if (child.el != null && prev?.el != null) {
      if (child.el.parentElement != null) {
        // skip append for optimization
        return child;
      }
      targetNode?.el?.insertBefore(child.el, prev.el.nextSibling);
      return child;
    }
    if (child.el != null && prev?.el == null) {
      if (child.el.parentElement != null) {
        // skip append for optimization
        return child;
      }
      if (targetNode?.el?.firstChild != null) {
        targetNode?.el?.insertBefore(child.el, targetNode.el.firstChild);
      } else {
        targetNode?.el?.appendChild(child.el);
      }
      return child;
    }
    return Array.from(child.children).reduce(
      (p: TreeNode<E> | null, c): TreeNode<E> | null => {
        return targetNode?.mountElement(c, p) ?? p;
      },
      prev,
    );
  };

  public dispose = () => {
    this.el?.parentElement?.removeChild(this.el);
    this.disposeStack.forEach((dispose) => dispose.val?.());
    this.parent?.children.delete(this);
    Array.from(this.children).forEach((child) => {
      child.dispose();
    });
  };

  public hasChild = (childElement: TreeNode<E>): boolean => {
    return (
      this.children.has(childElement) ||
      Array.from(this.children).reduce((p, c) => {
        return p || c.hasChild(childElement);
      }, false)
    );
  };

  public expandStaticContainer = (): TreeNode<E>[] => {
    if (!this.staticContainer) {
      return [this];
    }
    return [...this.children].flatMap((child) => child.expandStaticContainer());
  };

  public migrate = (from: TreeNode<E> | null, to: TreeNode<E>) => {
    // if both from and to is fragment
    if (from?.staticContainer && to.staticContainer) {
      Array.from(from.expandStaticContainer()).forEach((child) => {
        if (!to.hasChild(child)) {
          child.dispose();
        }
      });
      this.mount(to);
    } else {
      from?.dispose();
      this.mount(to);
    }
  };
}

export default TreeNode;
