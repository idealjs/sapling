import { useCallback, useSyncExternalStore } from "react";
import { createProxy } from "./createProxy";
import type { PathKey } from "./types";

type Selector<R> = () => R;

type SelectorEntry<R> = {
  selector: Selector<R>;
  snapshot: R | undefined;
  hasSnapshot: boolean;
  listeners: Set<() => void>;
  dependencies: Set<string>;
  dirty: boolean;
};

type TrieNode = {
  selectors: Set<Selector<unknown>>;
  children: Map<string, TrieNode>;
};

const PATH_SEPARATOR = "\u0001";

const serializePath = (paths: PathKey[]) => paths.join(PATH_SEPARATOR);

const createTrieNode = (): TrieNode => ({
  selectors: new Set(),
  children: new Map(),
});

const isPrefixPath = (parent: string, child: string) => {
  return parent === child || child.startsWith(`${parent}${PATH_SEPARATOR}`);
};

const compressDependencies = (dependencies: Set<string>) => {
  const sortedDependencies = [...dependencies].sort(
    (left, right) =>
      right.split(PATH_SEPARATOR).length - left.split(PATH_SEPARATOR).length,
  );
  const compressed = new Set<string>();

  for (const dependency of sortedDependencies) {
    let shouldKeep = true;
    for (const existingDependency of compressed) {
      if (isPrefixPath(dependency, existingDependency)) {
        shouldKeep = false;
        break;
      }
    }

    if (shouldKeep) {
      compressed.add(dependency);
    }
  }

  return compressed;
};

export const createStore = <T extends object>(value: T) => {
  const selectorEntries = new Map<Selector<unknown>, SelectorEntry<unknown>>();
  const rootTrie = createTrieNode();

  let trackingPaths: Set<string> | null = null;

  const getOrCreateEntry = <R,>(selector: Selector<R>) => {
    const existing = selectorEntries.get(selector) as
      | SelectorEntry<R>
      | undefined;
    if (existing) {
      return existing;
    }

    const entry: SelectorEntry<R> = {
      selector,
      snapshot: undefined,
      hasSnapshot: false,
      listeners: new Set(),
      dependencies: new Set(),
      dirty: true,
    };

    selectorEntries.set(selector, entry as SelectorEntry<unknown>);
    return entry;
  };

  const addDependencyToTrie = (
    selector: Selector<unknown>,
    dependency: string,
  ) => {
    const segments = dependency.split(PATH_SEPARATOR).filter(Boolean);
    let node = rootTrie;

    for (const segment of segments) {
      let nextNode = node.children.get(segment);
      if (!nextNode) {
        nextNode = createTrieNode();
        node.children.set(segment, nextNode);
      }

      node = nextNode;
    }

    node.selectors.add(selector);
  };

  const removeDependencyFromTrie = (
    selector: Selector<unknown>,
    dependency: string,
  ) => {
    const segments = dependency.split(PATH_SEPARATOR).filter(Boolean);
    const stack: TrieNode[] = [rootTrie];
    let node = rootTrie;

    for (const segment of segments) {
      const nextNode = node.children.get(segment);
      if (!nextNode) {
        return;
      }

      node = nextNode;
      stack.push(node);
    }

    node.selectors.delete(selector);

    for (let index = stack.length - 1; index > 0; index -= 1) {
      const currentNode = stack[index];
      if (currentNode.selectors.size > 0 || currentNode.children.size > 0) {
        break;
      }

      const parentNode = stack[index - 1];
      const segment = segments[index - 1];
      parentNode.children.delete(segment);
    }
  };

  const replaceDependencies = (
    entry: SelectorEntry<unknown>,
    nextDependencies: Set<string>,
  ) => {
    for (const dependency of entry.dependencies) {
      removeDependencyFromTrie(entry.selector, dependency);
    }

    entry.dependencies = nextDependencies;
    for (const dependency of nextDependencies) {
      addDependencyToTrie(entry.selector, dependency);
    }
  };

  const evaluateSelector = <R,>(entry: SelectorEntry<R>) => {
    const previousTrackingPaths = trackingPaths;

    trackingPaths = new Set();

    try {
      const nextSnapshot = entry.selector();
      const nextDependencies = compressDependencies(trackingPaths);
      if (!nextDependencies) {
        throw new Error(
          "Selector tracking context was lost during evaluation.",
        );
      }

      replaceDependencies(entry as SelectorEntry<unknown>, nextDependencies);
      entry.snapshot = nextSnapshot;
      entry.hasSnapshot = true;
      entry.dirty = false;
      return nextSnapshot;
    } finally {
      trackingPaths = previousTrackingPaths;
    }
  };

  const collectAffectedSelectors = (paths: PathKey[]) => {
    const affected = new Set<Selector<unknown>>();

    let prefixNode: TrieNode | undefined = rootTrie;
    for (const segment of paths) {
      prefixNode = prefixNode.children.get(segment);
      if (!prefixNode) {
        return affected;
      }

      for (const selector of prefixNode.selectors) {
        affected.add(selector);
      }
    }

    const collectSubtree = (node: TrieNode) => {
      for (const selector of node.selectors) {
        affected.add(selector);
      }

      for (const childNode of node.children.values()) {
        collectSubtree(childNode);
      }
    };

    collectSubtree(prefixNode);
    return affected;
  };

  const store = createProxy(value, {
    notifyGet(paths, operation) {
      if (operation === "get") {
        if (!trackingPaths) {
          return;
        }

        trackingPaths.add(serializePath(paths));
        return;
      }

      const affectedSelectors = collectAffectedSelectors(paths);

      for (const selector of affectedSelectors) {
        const entry = selectorEntries.get(selector);
        if (!entry) {
          continue;
        }

        entry.dirty = true;
        for (const listener of entry.listeners) {
          listener();
        }
      }
    },
  });

  const useSelector = <R,>(selector: Selector<R>) => {
    const subscribe = useCallback(
      (onStoreChange: () => void) => {
        const entry = getOrCreateEntry(selector);
        entry.listeners.add(onStoreChange);

        return () => {
          const currentEntry = selectorEntries.get(selector);
          if (!currentEntry) {
            return;
          }

          currentEntry.listeners.delete(onStoreChange);
          if (currentEntry.listeners.size > 0) {
            return;
          }

          for (const dependency of currentEntry.dependencies) {
            removeDependencyFromTrie(currentEntry.selector, dependency);
          }

          selectorEntries.delete(selector);
        };
      },
      [selector],
    );

    const getSnapshot = useCallback(() => {
      const entry = getOrCreateEntry(selector);
      if (entry.hasSnapshot && !entry.dirty) {
        return entry.snapshot as R;
      }

      return evaluateSelector(entry);
    }, [selector]);

    return useSyncExternalStore(subscribe, getSnapshot);
  };

  return {
    store,
    useSelector,
  };
};
