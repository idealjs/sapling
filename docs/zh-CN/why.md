# 前言

2018年，我开始接触并深入学习React。从React16到React18，从类组件到函数式组件，我逐渐从新手变成了熟悉各种API特性的高级前端工程师。

尽管React解决了大量的问题，减少了工程复杂度，缩短了从JS到HTML的编码距离，但仍有一些不足之处。

## 响应式（Reactive）

当前，前端编程库普遍具备响应式功能，那么这种设计模式是如何解决工程复杂度问题的呢？

### 使用响应式库前 

代码主要围绕 DOM 编写。随着工程规模的扩大，DOM API 代码量也随之增加。自然地，我们开始按照 DOM API 修改的部分来划分模块。 此时，数据应该放在哪里？

### 使用响应式库后

代码围绕`数据 => 视图`的关系来编写。即 `View = f(Data)` 的表示。

`数据与视图`操作不再耦合，代替的是`关系与视图`的耦合。

数据可以根据场景，自由组织且不受`设计模式与原则`限制。

响应式库根据`关系与视图`的耦合代码操作 Dom，100% 代替了以操作 Dom 为目的的代码。

## 小结

代码量减少，编码自由度提高，减少`设计模式`的限制。必然会降低工程复杂度。

## JSX 与 HTML 模板

### HTML 模板

JSX 与 HTML 模板是减少 Dom Api 代码的有效手段。

HTML 模板在响应式编程以前就已经存在，如 PUG EJS 等。

然而只使用模板并不能解决工程上的问题，仍然需要 Dom Api 来辅助完成工程师的意图。

在引入响应式编程后，有效的降低了工程复杂度问题，使 HTML 模板变为一种强力工具。

### JSX
JSX 在此方面更加激进，它允许 js 与 tag 混合的方式来描述`数据与视图`的关系。

因此，在编码时（人机交互的一种）工程师可以更加轻松的看到`数据与视图`的关系。

### 小结
JSX 与 HTML 模板是两种不同的编程风格，但是他们解决问题相同。

> HTML 模板并没有形成与库无关的语法定义。
> 
> JSX 已经被内置到了各种编辑器的默认支持中。

# React 的问题

## 提取用户代码意图的思路

```js
const [state, setState] = useState([1, 2, 3]);
const A = () => createElement("p");
const Root = () =>
  createElement("div", {
    children: state.map((id) => {
      createElement(A, {}, id);
    }),
  });
render(createElement(Root));
```

在 JS 脚本中，Root 的 children 始终会在 `createElement("div")` 执行之前被计算出结果。

此处 `数据与视图` 关系的被 `createElement(Root)` 执行时所创建出的作用域保存。

所以 Root 函数会在 state 产生变化时重新执行。也因此带来了一些问题。

1. 函数重新执行可能带有副作用（[side effect](https://zh.wikipedia.org/wiki/%E5%89%AF%E4%BD%9C%E7%94%A8_(%E8%AE%A1%E7%AE%97%E6%9C%BA%E7%A7%91%E5%AD%A6))）。
2. `数据与视图` 关系没有发生变化，但是函数重新执行。

## 过低的下限

在不使用经过特殊优化的函数时 （如 [solidjs flow_for](https://www.solidjs.com/tutorial/flow_for)），响应式库无法依据代码意图精准操作 Dom。

为了解决上面的问题，React 使用了以下的解决方案。
1. 使用 Virtual Dom 的来计算发生变化的 Dom，以此来精确操作Dom。同时避免 `数据与视图` 关系未发生变化的部分产生 Dom 变化。
2. 要求包含副作用的用户代码放置于 effect 中，并使用 deps 对比的方案来决定是否要重新执行 effect

也因为这些可选API，导致了低代码质量的 React 工程诞生。其根本原因是 React `提取用户代码意图的思路`

# 替代品

https://github.com/idealjs/sapling 项目仍在施工中。

sapling 源自于2022年时看到的 [solidjs 文章](https://dev.to/ryansolid/introducing-the-solidjs-ui-library-4mck)。

2023 年再一次受到世界上最小的响应式 UI 库 [vanjs](https://vanjs.org/) 的启发。

期望新的 `提取用户代码意图的思路` 可以带来更好的性能。
