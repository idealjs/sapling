# Project Brief

@idealjs/sapling 是一个轻量级的 JSX runtime 实现，旨在解决 React 中的一些性能问题。

## 核心目标

1. 提供更高效的响应式 UI 库实现
2. 使用更简单的数据结构和算法
3. 规避 React 的树状更新问题

## 主要特性

1. 内置缓存机制，用于解决性能问题
2. 响应式编程范式
3. JSX 转换与渲染支持
4. 提供更精准的 DOM 更新控制

## 项目结构

使用 monorepo 方式组织，主要包含以下包:

- core: 核心运行时实现
- reactive: 响应式系统实现  
- sapling_transformer: JSX 转换器
- sapling_macros: 宏相关功能
- sapling_shared: 共享工具和类型
- eslint-plugin-sapling: ESLint 插件

## 技术栈

- 前端: TypeScript + JSX
- 转换器: Rust + Biome
- 构建: Vite