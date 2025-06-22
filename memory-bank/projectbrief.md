# Project Brief

## Overview
这个项目是一个 Rust 实现的 AST 转换器，用于处理和优化 JavaScript/TypeScript 代码。项目使用了 oxc 生态系统的多个组件，包括 AST 解析、遍历和内存管理功能。

## Core Requirements
1. 高效的内存管理
   - 使用 arena 分配策略处理 AST 节点
   - 优化内存分配和释放过程
   - 确保内存安全和生命周期正确性

2. AST 遍历和转换
   - 支持复杂的 AST 节点操作
   - 提供高效的遍历上下文管理
   - 确保 AST 转换的正确性和性能

## Technical Goals
1. 内存管理目标
   - 最小化内存分配开销
   - 避免内存碎片
   - 支持大规模 AST 处理

2. 代码质量目标
   - 保持代码清晰和可维护
   - 遵循 Rust 最佳实践
   - 提供良好的错误处理和类型安全
