# Biome CST 遍历和转换

## CST 遍历机制

Biome 通过访问者模式实现 CST 的遍历，核心包括：

1. WalkEvent 遍历事件:
   - Enter(T) - 进入节点时触发
   - Leave(T) - 离开节点后触发

2. 遍历器类型:
   - Preorder - 只遍历节点
   - PreorderWithTokens - 遍历节点和 Token
   - PreorderTokens - 只遍历 Token

3. 访问者模式：
   - Visitor trait - 全局访问者，可访问任意节点
   - NodeVisitor trait - 节点特定访问者，生命周期与节点访问绑定
   - merge_node_visitors! 宏用于组合多个 NodeVisitor

## 代码转换实现

### 基本转换结构

1. 定义转换规则:
```rust
declare_transformation! {
    pub(crate) MyTransform {
        version: "1.0.0",
        name: "transformExample",
        language: "js",
    }
}
```

2. 实现 Rule trait:
```rust
impl Rule for MyTransform {
    type Query = Ast<TargetNode>;
    type State = TransformState;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        // 收集转换所需信息
    }

    fn transform(ctx: &RuleContext<Self>, state: &Self::State) -> ption<JsBatchMutation> {
        // 执行转换
    }
}
```

### 实际转换案例

1. TypeScript enum 转换
```typescript
// 输入
enum Foo {
    A,
    B
}

// 输出
var Foo;
(function (Foo) {
    Foo[Foo["A"] = 0] = "A";
    Foo[Foo["B"] = 1] = "B";
})(Foo || (Foo = {}));
```

2. Solid.js JSX 转换
```jsx
// 输入
function Counter() {
  const [count, setCount] = createSignal(1);
  return (
    <div>
      <For each={items}>
        {(item) => (
          <div>
            <span>{item.name}</span>
            <span>{item.value}</span>
          </div>
        )}
      </For>
    </div>
  );
}

// 输出 (客户端渲染)
var _el$ = _tmpl$();
_$insert(_el$, _$createComponent(For, {
  each: items,
  children: item => (() => {
    var _el$2 = _tmpl$2(),
      _el$3 = _el$2.firstChild,
      _el$4 = _el$3.nextSibling;
    _$insert(_el$3, () => item.name);
    _$insert(_el$4, () => item.value);
    return _el$2;
  })()
}));
return _el$;

// 输出 (服务端渲染)
import { ssr as _$ssr } from "solid-js/web";
import { escape as _$escape } from "solid-js/web";
var _tmpl$ = ["<div", ">", "</div>"],
    _tmpl$2 = ["<div", "><span>", "</span><span>", "</span></div>"];

function Counter() {
  const [count, setCount] = createSignal(1);
  return _$ssr(_tmpl$, _$ssrHydrationKey(), 
    _$escape(_$createComponent(For, {
      each: items,
      children: item => _$ssr(_tmpl$2, _$ssrHydrationKey(), 
        _$escape(item.name), 
        _$escape(item.value)
      )
    }))
  );
}
```

### 转换实现要点

1. 状态管理:
   - 追踪模板和变量
   - 维护作用域信息
   - 管理导入声明

2. 节点转换:
   - 使用 factory 函数创建新节点
   - 通过 mutation API 替换节点
   - 处理子节点递归转换

3. 关键考虑:
   - 类型安全
   - 作用域处理
   - 正确的错误处理
   - 源码映射支持

这种基于访问者模式的转换机制可以实现从简单的语法转换到复杂的框架编译，同时保证类型安全和可维护性。

# Biome 相关内容

### 转换器实现

1. 收集模板和状态:
```rust
struct SolidTransformState {
    // 追踪模板变量
    template_vars: Vec<String>,
    // 追踪作用域内的变量
    scope_vars: Vec<String>,
    // 收集的模板定义
    templates: HashMap<String, JsxElement>,
}
```

2. 实现转换:
```rust 
impl Rule for SolidTransform {
    type Query = Ast<JsxElement>;
    type State = SolidTransformState;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let node = ctx.query();
        // 收集静态模板和状态
        Some(collect_transform_info(node))
    }

    fn transform(ctx: &RuleContext<Self>, state: &Self::State) -> Option<JsBatchMutation> {
        let node = ctx.query();
        let mut mutation = node.clone().begin();

        match state.mode {
            // 客户端渲染转换
            ClientSide => transform_client_side(node, state, &mut mutation),
            // 服务端渲染转换 
            ServerSide => transform_server_side(node, state, &mut mutation),
        }

        Some(mutation)
    }
}
```

3. 辅助函数:
```rust
// 生成模板引用
fn make_template_ref(name: &str) -> JsCallExpression {
    js_call_expression(
        js_identifier("_tmpl$"),
        []
    )
}

// 生成插入调用
fn make_insert_call(target: &str, value: JsExpression) -> JsCallExpression {
    js_call_expression(
        js_identifier("_$insert"),
        [js_identifier(target), value]
    )
}

// SSR 相关转换
fn make_ssr_call(template: &str, key: JsExpression, children: JsExpression) -> JsCallExpression {
    js_call_expression(
        js_identifier("_$ssr"),
        [
            js_identifier(template),
            make_hydration_key_call(),
            make_escape_call(children)
        ]
    )
}
```

这种转换设计可以:
1. 处理复杂的 JSX 结构
2. 支持客户端和服务端两种模式
3. 正确处理作用域和模板
4. 维护代码的可读性和可调试性

