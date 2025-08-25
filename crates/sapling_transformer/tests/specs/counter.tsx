import { State } from "@idealjs/sapling";

class App {
  @State
  accessor obj: { count: number } = { count: 0 };
  public render() {
    let { count } = this.obj;
    return (
      <div>
        <button
          onClick={() => {
            count++;
          }}
        >
          +
        </button>
        {count}
        <button
          onClick={() => {
            count--;
          }}
        >
          -
        </button>
      </div>
    );
  }
}

export default App;
