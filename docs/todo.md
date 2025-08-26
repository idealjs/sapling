# 待实现

1. createComponent
2. effect insertNode updateNode
3. effect setProps updateProps
4. 数据标记，由于对象解构会使 get 在非预期位置调用，需要使用编译时数据标记来处理响应式
    - 记录 State、Derive
    - 处理 render 中的展开式。使用变量 name，表达式，binding 三种方式在编译时追踪。

# bug 修复

1. 存在 render 没有被 transform
