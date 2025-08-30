# Sapling Jsx 框架

Sapling 的理念是，在尽可能减少非用户代码与心智负担的情况下，由框架提供响应式编程的能力。

## 快速入门

🚧

### 计数器

main.tsx

```ts
import App from "./App.tsx"

const domNode = document.getElementById('root');
const root = createRoot(domNode);

root.render(<App />);
```

App.tsx

```tsx
import { Component } from "@idealjs/sapling";

class App extends Component {
  @State accessor count: number = 0;
  @Mutate plusOne() {
    return this.value++;
  }
  @Mutate minusOne() {
    return this.value++;
  }
  public render() {
    return (
      <div>
        <button onClick={this.plusOne}>+</button>
        {this.count}
        <button onClick={this.minusOne}>-</button>
      </div>
    );
  }
}

export default App;
```

### 代办列表

main.tsx

```ts
import App from "./App.tsx"

const domNode = document.getElementById('root');
const root = createRoot(domNode);

root.render(<App />);
```

App.tsx

```tsx
import { Component } from "@idealjs/sapling";
import Todo, { type ITodo } from "./Todo.tsx";

class App extends Component {
  @State accessor todos: ITodo[] = [];
  public render() {
    return (
      <div>
        <For each={state.list}>{(item) => <Todo todo={item} />}</For>
      </div>
    );
  }
}
```

Todo.tsx

```tsx
import { Component } from "@idealjs/sapling";

export interface ITodo {
  name: string;
  done: boolean;
}

interface IProps {
  todo: ITodo;
}

class Todo extends Component {
  constructor(props: IProps) {
    super(props);
  }
}
export default Todo;
```

### 带有初始化数据的代办

🚧

## State 定义

```tsx
import { Component } from "@idealjs/sapling";

class Example extends Component {
  @State accessor value: number = 0;
}
```

## Derive 定义

```tsx
import { Component } from "@idealjs/sapling";

class Example extends Component {
  @Derive get doubleValue() {
    return this.value * 2;
  }
}
```

## Mutate 定义

```tsx
import { Component } from "@idealjs/sapling";

class Example extends Component {
  @Mutate plusOne() {
    return this.value++;
  }
}
```

## effect 使用

```ts
import { Component } from "@idealjs/sapling";

class Example extends Component {
  @State accessor inputValue = "";
  @Mutate updateValue(value) {}
  constructor() {
    super();
    this.effect(() => {
      getData(this.inputValue).then(updateValue);
      // AUTO_INFER will be compiled and replaced by [this.inputValue]
    }, AUTO_INFER);

    this.effect(() => {
      console.log("log if component trigger any mutate");
    }, EACH_TIME);

    this.effect(
      () => {
        console.log(
          "Although the this.inputValue variable is not used, the log will be printed after this.inputValue is updated",
        );
      },
      () => [this.inputValue],
    );
  }
}
```

## dispose 使用

🚧

```ts
import { Component } from "@idealjs/sapling";

class Example extends Component {
  constructor() {
    super();
    this.addEventListener("dispose", () => {
      console.log("component dispose");
    });
  }
}
```

## render 定义

```tsx
import { Component } from "@idealjs/sapling";

class Example extends Component {
  @State accessor value: number = 0;
  public render() {
    // 将会被编译为
    // let el = createElement("div");
    // effect(() => {
    //   insertOrUpdate(el, this.value);
    // }, [this.value]);
    return <div>{this.value}</div>;
  }
}
```

## batch update

```tsx
import { Component } from "@idealjs/sapling";

class Example extends Component {
  public render() {
    return (
      <div
        onClick={() => {
          // 仅触发一次更新
          this.batch(() => {
            this.value++;
            this.value++;
            this.value++;
          });
        }}
      />
    );
  }
}
```

## 基于用户代码的追踪

```tsx
import { State, Component } from "@idealjs/sapling";

class App extends Component {
  @State
  accessor obj: { count: number } = { count: 0 };
  @Mutate plusOne() {
    let { count } = this.obj;
    return count++;
  }
  public render() {
    return _$createJsxTagElement(() => {
      let _el$ = _$createElement("div");
      let _el$1 = _$createElement("button");
      _$setProp(_el$1, "onClick", plusOne);
      _$insertNode(_el$1, _$createTextNode("+"));
      _$insertNode(_el$, _el$1);
      // 追踪 count, 如果是 State 或者 Derive
      effect(
        () => {
          _$insert(_el$, count);
        },
        () => [count],
      );
      return _el$;
    });
  }
}

export default App;
```

# 非用户代码

如，为提供响应式能力而添加的非用户代码 ———— get 函数。

```tsx
<Counter value={value()}>
```

```ts
createComponent(Counter, {
  get value() {
    return value();
  },
});
```

# 心智负担

如，为了响应式能力要求用户使用特定语法 ———— props.xxx

```tsx
interface IProps {
  value: number;
}
const Counter = (props: IProps) => {
  // 必须使用此语法才能获得响应式能力
  return <div>{props.value}</div>;
};

const Counter = (props: IProps) => {
  // 此类写法无法支持响应式能力
  const { value } = props;
  return <div>{value}</div>;
};
```
