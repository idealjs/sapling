# 前言

在上一篇文章中，介绍了为什么我不再需要 React。并且开始从 0 到 1 编写响应式库。

这篇文章会介绍 sapling 响应式库的核心原理。

# 响应式库

响应式库是什么样的概念？这与事件哪些异同点呢？

**相同点**

| 行为                            | 事件 | 响应式 |
| ------------------------------- | ---- | ------ |
| 触发一段用户代码                | \*   | \*     |
| 可以触发其他事件\修改响应式数据 | \*   | \*     |
| 可以用逻辑代码实现              | \*   | \*     |

**不同点**

| 行为                            | 事件 | 响应式 |
| ------------------------------- | ---- | ------ |
| 浏览器/node 原生支持            | \*   | -      |


响应式库的目的在于当数据产生变更时，自动的执行一段`用户代码`。

虽然事件也可以做到，并且是浏览器原生支持。但是代码风格上并不相同。


```js
// 响应式
const a = createSignal(0);
const b = derive(() => a.val + 2);
const c = derive(() => b.val + 3);
```

```js
// 响应式
const a = createSignal(0);
const b = createSignal();
const c = createSignal();
effect(() => {
    b.val = a.val + 2
});
effect(() => {
    c.val = b.val + 3
});
```

```js
// 事件
const a = new EventEmitter();
const b = new EventEmitter();
a.addEventListener("change",()=>{
    b.val = a.val + 2;
    b.emit("change")
})
const c = new EventEmitter();
b.addEventListener("change",()=>{
    c.val = b.val + 2;
    c.emit("change")
})
```

对比以上的代码，可以发现。响应式代码相比于事件更具有`特定领域`的表现力。

1. 这里的 a b c 变量都是 EventEmitter 或者 响应式变量
2. 需要在执行赋值后，继续执行`用户代码`

## 实现响应式库

上面的**不同点**提到，并无原生的响应式支持。那么如何实现一个最基础的响应式库？

响应式的实现核心思路在于如何记录变量与函数之间的关系。

**实现思路**

1. createSignal 创建一个对象，在对象中保存与之相关的函数。

2. effect 执行时，将 callback 记录到全局。

3. signal 在 callback 中被读取意味着两者关系建立。在 signal 读取时，从全局获取 callback 并记录到对象中。


**实现方式**

1. 基于 EventEmitter，浏览器端可以使用 events 库。

2. 使用栈，维护订阅队列。

> 注意不要使用浏览器的 dispatchEvent。这会导致库的代码进入事件循环，降低用户代码的性能。

### 简单的实现

**确定 API 类型**

1. createSignal 用于创建响应式变量。

```ts
function createSignal<T>():{val:T}
```

2. effect 在响应式变量产生变化时重新执行 callback 内容。Dispose 函数用于清理 effect 中所产生的副作用。

```ts
function effect(callback:()=>Dispose):Dispose
```

derive 后续可以使用 effect + createSignal 的方式进行封装。估先不考虑 derive 的实现。

**实现**

```ts
type Dispose = () => void;

const fStack: (() => unknown)[] = [];

const createSignal = <T>(value: T) => {
  let val: T = value;
  const listeners = new Set<() => unknown>();
  return {
    get val() {
      fStack[fStack.length - 1] && listeners.add(fStack[fStack.length - 1]);
      return val;
    },
    set val(value: T) {
      fStack.forEach((f) => f());
      val = value;
      this._listeners.clear();
    },
  };
};

const effect = (callback: () => Dispose) => {
  fStack.push(callback);
  const val = callback();
  fStack.pop();
  return val;
};
```

在对 val 进行赋值时，记得清除 listener，因为有可能 effect 中 if 语句执行后，不再依赖此 val。

后续文章中，再细说 fStack 和 dispose 的作用。
