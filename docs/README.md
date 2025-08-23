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
class App {
  @State accessor count: number = 0;
  public render() {
    return (
      <div>
        <button
          onClick={() => {
            this.count++;
          }}
        >
          +
        </button>
        {this.count}
        <button
          onClick={() => {
            this.count--;
          }}
        >
          -
        </button>
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
import Todo, { type ITodo } from "./Todo.tsx";

class App {
  @State accessor todos: ITodo[] = [];
  public render() {
    return (
      <div>
        <For each={state.list} fallback={<div>Loading...</div>}>
          {(item) => <Todo todo={item} />}
        </For>
      </div>
    );
  }
}
```

Todo.tsx

```tsx
export interface ITodo {
  name: string;
  done: boolean;
}

interface IProps {
  todo: ITodo;
}

class Todo {
  constructor(props: IProps) {}
}
export default Todo;
```

### 带有初始化数据的代办

🚧

## State 定义

```tsx
class Example {
  @State accessor value: number = 0;
}
```

## Derive 定义

```tsx
class Example {
  @Derive get doubleValue() {
    return this.value * 2;
  }
}
```

## effect 使用

```ts
class Example {
  constructor() {
    effect(() => {
      getData(this.inputValue).then((value) => {
        this.value = value;
      });
    });
  }
}
```

## render 定义

```tsx
class Example {
  public render() {
    return <div>{this.value}</div>;
  }
}
```

## batch update

```tsx
class Example {
  public render() {
    return (
      <div
        onClick={() => {
          // 仅触发一次更新
          batch(() => {
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
