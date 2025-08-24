import { State } from "@idealjs/sapling";

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
