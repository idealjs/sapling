# Technical Context

## Memory Management in AST Traversal

### Core Components

1. **Arena Allocator (`Allocator`)**
   - From `oxc_allocator` crate
   - 基础的内存分配器
   - 采用 arena 分配策略：所有内存在同一个大块中分配，统一释放
   - 特别适合 AST 这种整体生命周期的场景
   - 主要功能：
     - 基础内存分配
     - 创建集合类型（如 `AstVec::new_in(allocator)`）
     - 处理 `CloneIn` trait 的克隆操作

2. **Traversal Context (`TraverseCtx`)**
   - From `oxc_traverse` crate
   - 高级抽象层，内部封装了 arena allocator
   - 在内存分配基础上提供 AST 遍历功能
   - 主要功能：
     - AST 节点的分配和管理（通过 `ctx.alloc()`）
     - 维护遍历状态
     - 提供遍历过程中的上下文信息

### 关系和使用模式

1. **互补关系**
   - ctx 和 allocator 是互补而非替代关系
   - ctx 内部包含 allocator，在其基础上增加了 AST 特定功能
   - 某些操作只能用 allocator（如创建基础集合）
   - 某些操作只能用 ctx（如分配 AST 节点）

2. **代码示例**
```rust
// 基础内存分配，使用 allocator
let mut declarators = AstVec::new_in(allocator);

// AST 节点分配，使用 ctx
let declaration = ctx.alloc(VariableDeclaration { ... });
```

### 最佳实践

1. **使用 Allocator 当：**
   - 需要直接的内存分配操作
   - 创建基础集合类型
   - 处理 CloneIn trait 的克隆操作

2. **使用 TraverseCtx 当：**
   - 需要分配和管理 AST 节点
   - 需要维护遍历状态
   - 需要遍历上下文信息

### Working with Template Items

1. **Template Structure**
   - TemplateItem requires references to Expression
   - When using `ctx.alloc()`, which returns boxed Expressions, you need to dereference:
```rust
let id = ctx.alloc(Expression::Identifier(...));
let template = ctx.alloc(Expression::StringLiteral(...));

let template_item = TemplateItem {
    id: &*id,         // Dereference Box<Expression> to get &Expression
    template: &*template,
};
```

2. **StringLiteral Structure**
   - Required fields:
     - span: Span
     - value: Atom
     - raw: Option<Atom>
     - lone_surrogates: bool
