# Active Context

## 当前工作重点

### JSX Transformer 实现
1. 基于 Biome 的实现方案
   - 使用 biome_js_parser 解析 JSX
   - 使用 biome_js_syntax 处理语法节点
   - 使用 biome_rowan 处理 AST

2. 转换规则设计
   ```jsx
   // 输入
   <div>Hello</div>

   // 输出 (参考 SolidJS 格式)
   _$template(`<div>Hello</div>`, 2)
   ```

3. 需要处理的场景
   - 普通 JSX 元素 (`<div>Hello</div>` -> `_$template`)
   - 自闭合元素 (`<br/>` -> `_$template`)
   - 动态属性 (`<div class={x}>` -> 使用 `_$spread` 或 `_$setAttribute`)
   - 事件绑定 (`onClick={...}` -> `_$addEventListener`)
   - 条件渲染 (使用 `_$createComponent` 和 `Show`)
   - 列表渲染 (使用 `_$createComponent` 和 `For`) 

## 关键决策

1. 技术选择
   - 使用 Rust 实现转换器
   - 采用 Biome 作为解析工具
   - 参考 SolidJS 的转换策略

2. 转换策略
   - 模板字符串优化
   - 静态内容提升
   - 动态内容处理
   - 支持 source map

3. 性能考虑
   - 缓存静态模板
   - 增量编译支持
   - 并行处理能力

## 当前挑战

1. 技术挑战
   - JSX 语法完整性支持
   - 错误处理和恢复
   - 源码映射维护

2. 待解决问题
   - 复杂 JSX 表达式处理
   - TypeScript 类型保持
   - 构建工具集成

## 下一步计划

1. 核心功能实现
   - [x] 基础 JSX 转换
   - [ ] 静态优化
   - [ ] 动态属性处理
   - [ ] 事件绑定
   - [ ] 控制流组件

2. 工具链集成
   - [ ] Vite 插件
   - [ ] Rollup 插件
   - [ ] ESLint 规则

3. 文档和测试
   - [ ] API 文档
   - [ ] 使用示例
   - [ ] 测试用例