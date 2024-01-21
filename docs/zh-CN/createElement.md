上一节提到了如何创建一个简单的 JSX 解析器，但是并没有涉及到响应式编程。
让我们来定义一些新的语法，这样可以更优雅地使用 JSX。

也让我们的库拥有更简单的数据结构与算法。

```tsx
const state = createProxy({val:[1,2,3]})

<div>
  {() => {
    return state.val.map((v) => {
      return <CustomComponent key={v} />;
    });
  }}
</div>
```

<font color="green"></font> 
- <font color="green">SaplingElement // div</font> 
  - <font color="green">children</font> 
    - <font color="red">SaplingElement</font>  // the arrow function
      - <font color="red">children</font> 
        - <font color="orange">SaplingElement // key 1</font> 
        - <font color="orange">SaplingElement // key 2</font> 
        - <font color="orange">SaplingElement // key 3</font> 

上述例子中，JSX 代码运行后，会转换成 SaplingElement 对象。

SaplingElement 有两个功能

1. 描述用户代码的结构
2. 缓存已经生成的 HTMLElement/SVGElement

我们可以做一些定义：

- 绿色部分不需要生成新的 SaplingElement。
- 红色部分在依赖的响应式变量更新时，一定会重新生成 SaplingElement
- 橙色部分会根据缓存来决定是否重新生成 SaplingElement

根据上面的定义，我们可以给出一个更加复杂的 createElement 函数

```ts
const createElement = (jsxTag,options,key)=>{
  if(typeof jsxTag === "function"){
    return jsxTag(options);
  }
  const currentElement = new SaplingElement({
    node: hyper(jsxTag,options)
  })
  if(children!=null){
    let nodeCaches: Map<Key, SaplingElement>[] = [];
    effect(()=>{
      // element 对应了红色部分的 saplingElement，响应式变量更新时这里的函数都会重新执行
      const saplingElement = prepareSaplingElement(children, nodeCaches);
      currentElement.upsert(saplingElement);
    })
  }
  return currentElement;
}
```
