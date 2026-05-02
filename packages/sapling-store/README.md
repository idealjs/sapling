# sapling-store

一个针对“可变（mutable）数据修改”的外部 store 库。

## 背景与问题

在 JavaScript 环境下，使用可变更新（直接修改对象/数组）可以显著减少短期对象分配，从而降低 GC 压力并提升性能。但这样会带来与 React 等框架集成的兼容问题：

- React 的 `useSyncExternalStore`（或类似的 external store + selector 模式）期望当外部状态变化时能返回一个新的 snapshot（值引用发生变化），以触发组件重新渲染。
- 如果外部 store 使用了可变更新，直接返回被内部重用的对象引用会导致 selector 返回的仍为“旧值”（即引用未变），从而无法触发 React 的更新或导致组件读取到过期数据。

为了解决这个矛盾，本库采用了基于“selector 依赖路径追踪 + 路径级缓存与按需展开（expand）”的策略：既保留可变更新带来的低 GC 优势，又在需要向外部暴露 snapshot 时保证对外返回新的值。

## 设计理念（概要）

- Selector 运行时会被“跟踪”——收集 selector 访问到的状态路径（比如 `state.a.b.c`）。
- 我们把 selector 依赖的最深路径及其值缓存起来（path -> cachedValue）。
- 当 store 发生更新时，我们会根据更新的路径判断哪些缓存的依赖被影响了；仅对受影响的依赖重新计算并展开（生成新的快照值），并通知对应订阅者。
- 这样在 React 的 `useSyncExternalStore` 调用中，受影响的 selector 能拿到新的 snapshot（新的引用），触发正确的渲染；未受影响的 selector 则能继续复用旧引用，减少不必要的分配。

## 主要特性

- 支持可变风格的高性能更新（低 GC）。
- 自动追踪 selector 的字段路径依赖。
- 路径级缓存与最小化重新计算，按需生成新的 snapshot 引用以兼容 React。
- 提供与 React 友好的 Hook（例如类似 `useStoreSelector` 的接口），兼容 `useSyncExternalStore` 模型。

## 简单示例

（下面示例基于库导出的约定 API，实际命名请参照 `src` 中的导出）

```ts
import { createStore, useStoreSelector } from 'sapling-store';

const store = createStore({ a: { b: { c: 0 } } });

// 在组件中按 selector 订阅
function MyComponent() {
  const c = useStoreSelector(store, s => s.a.b.c);
  return <div>{c}</div>;
}

// 可变更新（例如在性能敏感路径）
store.mutate(state => {
  state.a.b.c += 1; // 直接修改，减少临时对象分配
});
```

在上面的模型中，当 `mutate` 修改了 `a.b.c` 时，我们会检测到依赖路径 `a.b.c` 被改变，重新展开并为该 selector 返回新的 snapshot，从而触发 `MyComponent` 的更新。

## 注意与限制

- 依赖路径追踪依赖 selector 的读取行为；对动态路径或反射式访问需要小心设计 selector。
- 本库在选择性地生成新引用以驱动渲染时力求最小化分配，但在大量不同 selector 被频繁触发的场景下仍会产生一些短期对象。
- 若需要对外提供完整不可变快照（deep clone），请明确调用相应的 API；本库默认只按 selector 需要做最小展开。

## 贡献与扩展方向

- 增强对复杂 selector（如数组索引、Map/Set 等）路径追踪的支持。 
- 提供更多与 React 工具链（像 Concurrent 模式、server components 等）的兼容性测试。
- 增加诊断工具，用于可视化 selector 的依赖路径与更新频率。

---

更多细节请参见同目录下的设计说明文档：

[packages/sapling-store/DESIGN.md](packages/sapling-store/DESIGN.md)
