# Technical Context

## 核心技术选择

### 1. JSX 转换实现 (sapling_transformer)
- 基于 Biome 的 Rust crate 实现
- 使用 biome_js_parser 解析 JSX 代码
- 使用 biome_js_syntax 处理语法节点
- 支持将 JSX 转换为 createElement 调用

### 2. 响应式系统 (reactive)
- 使用 Proxy 进行数据劫持
- 支持 effect 和 derive 
- 基于 fStack 实现依赖收集
- 提供精确的更新调度

### 3. 核心运行时 (core)
- 实现 JSX runtime
- 提供 createElement 和 createRoot API
- 支持 Suspense 功能
- 实现 TreeNode 结构管理组件树

### 4. 构建工具链
- 使用 Vite 作为开发服务器和构建工具
- 支持 TypeScript 
- 提供 ESLint 插件进行代码检查

## 开发环境设置

1. 项目配置:
```json
{
  "compilerOptions": {
    "jsx": "react-jsx",
    "jsxImportSource": "@idealjs/sapling"
  }
}
```

2. 依赖安装:
```bash
yarn add @idealjs/sapling
```

## 技术约束

1. 浏览器兼容性要求:
   - 支持现代浏览器
   - 需要原生 Proxy 支持

2. 构建输出:
   - ESM 格式
   - 支持 Tree-shaking