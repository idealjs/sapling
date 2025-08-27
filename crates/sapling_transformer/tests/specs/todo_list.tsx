import { State } from "@idealjs/sapling-reactive";

import For from "./for";

class App {
  @State accessor todos: ITodo[] = [];
  render() {
    return (
      <div>
        <For each={this.todos} fallback={<div>Loading...</div>}>
          {(item) => <Todo todo={item} />}
        </For>
      </div>
    );
  }
}

export interface ITodo {
  name: string;
  done: boolean;
}

interface IProps {
  todo: ITodo;
}

class Todo {
  @State accessor props: IProps;
  constructor(_props: IProps) {
    this.props = _props;
  }
  render() {
    const { todo } = this.props;
    return <div>{todo.name}</div>;
  }
}

export default App;
