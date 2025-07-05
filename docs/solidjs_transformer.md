# SolidJS Transformer 流程图

## 概述

SolidJS Transformer 是一个基于 Rust 和 Biome 的 JSX 转换器，将 JSX 语法转换为 SolidJS 运行时调用。本文档详细描述了转换器的工作流程和决策路径。

## 整体架构流程

```mermaid
flowchart TD
    A[JSX 源码输入] --> B[Biome 解析器]
    B --> C[生成 AST]
    C --> D{检测是否包含 JSX}
    D -->|否| E[直接返回原代码]
    D -->|是| F[创建 TransformState]
    F --> G[遍历模块项]
    G --> H[transform_module_item_with_tracker]
    H --> I[收集 Helper 使用统计]
    I --> J[generate_solid_imports]
    J --> K[重构整个模块]
    K --> L[输出转换后的代码]
```

## 核心转换决策流程

```mermaid
flowchart TD
    A[JSX 节点] --> B{判断节点类型}
    
    B -->|JSX Element| C[原生 HTML 元素]
    B -->|JSX Component| D[React 组件]
    B -->|JSX Fragment| E[Fragment 片段]
    B -->|JSX Expression| F[表达式插值]
    
    C --> C1{是否自闭合}
    C1 -->|是| C2[create_solidjs_call_self_closing]
    C1 -->|否| C3[create_solidjs_call_with_tracker]
    
    D --> D1[create_component_call]
    D1 --> D2[生成 createComponent 调用]
    
    E --> E1{Fragment 子元素数量}
    E1 -->|单个| E2[直接返回子元素]
    E1 -->|多个| E3[包装成数组]
    
    F --> F1{表达式类型}
    F1 -->|静态值| F2[直接保留]
    F1 -->|动态调用| F3[包装 memo 函数]
    
    C2 --> G[生成 IIFE]
    C3 --> G
    D2 --> H[组件实例化]
    E2 --> I[递归转换子元素]
    E3 --> I
    F2 --> J[表达式处理完成]
    F3 --> J
    
    G --> K[原生元素处理完成]
    H --> K
    I --> K
    J --> K
    K --> L[更新 HelperUsageTracker]
    L --> M[继续处理其他节点]
```

## JSX 元素转换详细流程

### 1. 原生元素转换 (如 `<div>`)

```mermaid
flowchart TD
    A[JSX 原生元素] --> B{判断是否自闭合}
    
    B -->|自闭合| C[create_solidjs_call_self_closing]
    B -->|非自闭合| D[create_solidjs_call_with_tracker]
    
    C --> C1[创建 createElement 调用]
    C1 --> C2[处理属性]
    C2 --> C3[包装 IIFE]
    C3 --> C4[返回 IIFE 表达式]
    
    D --> D1[创建 createElement 调用]
    D1 --> D2[处理子元素]
    D2 --> D3[处理属性]
    D3 --> D4[包装 IIFE]
    D4 --> D5[返回完整 IIFE 表达式]
```

### 2. 组件转换 (如 `<Component>`)

```mermaid
flowchart TD
    A[JSX 组件] --> B[判断是否自定义组件]
    B -->|是| C[create_component_call]
    C --> D[处理 props]
    D --> E[收集属性]
    E --> F[生成 props 对象]
    F --> G[创建 createComponent 调用]
    G --> H[返回组件调用表达式]
```

### 3. Fragment 转换 (如 `<>...</>`)

```mermaid
flowchart TD
    A[JSX Fragment] --> B[遍历子元素]
    B --> C{子元素类型}
    
    C -->|JSX Element| D[递归转换元素]
    C -->|JSX Expression| E[处理表达式]
    C -->|JSX Text| F[创建文本节点]
    
    D --> G[收集转换结果]
    E --> G
    F --> G
    
    G --> H{子元素数量}
    H -->|单个| I[直接返回元素]
    H -->|多个| J[包装成数组]
    
    I --> K[单个表达式]
    J --> L[返回数组表达式]
```

### 4. 表达式转换 (如 `{value}`)

```mermaid
flowchart TD
    A[JSX 表达式] --> B{表达式类型判断}
    
    B -->|标识符| C[静态引用]
    B -->|调用表达式| D[动态调用]
    B -->|字面量| E[静态值]
    B -->|复杂表达式| F[动态表达式]
    
    C --> G[直接保留]
    D --> H[包装 memo 函数]
    E --> G
    F --> H
    
    G --> I[插入到父元素]
    H --> J[生成 memo 调用]
    J --> I
```

## 属性处理流程

### 静态属性处理

```mermaid
flowchart TD
    A[静态属性] --> B[属性值解析]
    B --> C[生成 setProp 调用]
    C --> D[返回属性设置表达式]
```

### 动态属性处理

```mermaid
flowchart TD
    A[动态属性] --> B[动态值解析]
    B --> C[生成 effect 调用]
    C --> D[返回响应式属性设置]
```

### 事件处理

```mermaid
flowchart TD
    A[事件属性] --> B[事件处理器解析]
    B --> C[生成事件绑定]
    C --> D[返回事件监听器设置]
```

## Helper 函数统计

转换过程中会统计使用的 helper 函数：

```mermaid
flowchart TD
    A[HelperUsageTracker] --> B[create_element: bool]
    A --> C[create_component: bool]
    A --> D[insert_node: bool]
    A --> E[create_text_node: bool]
    A --> F[memo: bool]
    A --> G[effect: bool]
    A --> H[set_prop: bool]
    
    B --> I[createElement 导入]
    C --> J[createComponent 导入]
    D --> K[insertNode 导入]
    E --> L[createTextNode 导入]
    F --> M[memo 导入]
    G --> N[effect 导入]
    H --> O[setProp 导入]
```

## 转换示例

### 简单元素转换

**输入:**
```jsx
<div>Hello</div>
```

**输出:**
```javascript
(() => {
  var _el$ = _$createElement("div");
  _$insertNode(_el$, _$createTextNode("Hello"));
  return _el$;
})()
```

### Fragment 转换

**输入:**
```jsx
<>
  <div>First</div>
  <div>Second</div>
</>
```

**输出:**
```javascript
[
  (() => {
    var _el$ = _$createElement("div");
    _$insertNode(_el$, _$createTextNode("First"));
    return _el$;
  })(),
  (() => {
    var _el$2 = _$createElement("div");
    _$insertNode(_el$2, _$createTextNode("Second"));
    return _el$2;
  })()
]
```

### 组件转换

**输入:**
```jsx
<Component prop={value} />
```

**输出:**
```javascript
_$createComponent(Component, { prop: value })
```

### 动态表达式转换

**输入:**
```jsx
<div>{count()}</div>
```

**输出:**
```javascript
(() => {
  var _el$ = _$createElement("div");
  _$insertNode(_el$, _$memo(() => count()));
  return _el$;
})()
```

## 当前实现状态

### 已实现功能 ✅
- [x] 基础 JSX 元素转换
- [x] 自闭合元素处理
- [x] Fragment 转换
- [x] 组件调用生成
- [x] 静态属性处理
- [x] Helper 函数统计
- [x] Import 语句生成

### 部分实现功能 ⚠️
- [x] JSX 表达式处理 (基础版本)
- [x] 动态属性处理 (需要完善)
- [x] 事件绑定 (需要实现)
- [x] 文本节点处理 (需要优化)

### 待实现功能 ❌
- [ ] 条件渲染优化
- [ ] 列表渲染优化
- [ ] 复杂表达式处理
- [ ] 错误处理机制
- [ ] 源码映射支持
- [ ] 性能优化

## 技术架构

### 依赖关系

```mermaid
graph TD
    A[sapling_transformer] --> B[sapling_transformation]
    B --> C[biome_js_parser]
    B --> D[biome_js_syntax]
    B --> E[biome_js_factory]
    B --> F[biome_analyze]
    
    A --> G[测试用例]
    G --> H[Fragment 测试]
    G --> I[组件测试] 
    G --> J[属性测试]
    G --> K[表达式测试]
```

### 模块结构

```
src/transformations/jsx_template/
├── transform_module.rs          # 模块级转换
├── transform_expression.rs      # 表达式转换
├── transform_statement.rs       # 语句转换
├── create_solidjs_call_*.rs     # SolidJS 调用生成
├── create_component_call.rs     # 组件调用生成
├── handle_jsx_attributes.rs     # 属性处理
├── generate_solid_imports.rs    # Import 生成
└── helpers/                     # 辅助函数
```

## 总结

SolidJS Transformer 采用了模块化的设计，通过 Biome 解析器处理 JSX 语法，然后根据不同的 JSX 节点类型选择相应的转换策略。核心思路是将 JSX 转换为 SolidJS 的运行时调用，通过 IIFE 模式确保每个元素都有独立的作用域，同时通过 Helper 函数统计来优化 import 语句的生成。

当前实现已经覆盖了基础的转换功能，但在复杂表达式处理、性能优化和错误处理方面还有改进空间。