import { createRoot, Mutate, State } from "@idealjs/sapling";

import For from "./for";

const A = [
  "pretty",
  "large",
  "big",
  "small",
  "tall",
  "short",
  "long",
  "handsome",
  "plain",
  "quaint",
  "clean",
  "elegant",
  "easy",
  "angry",
  "crazy",
  "helpful",
  "mushy",
  "odd",
  "unsightly",
  "adorable",
  "important",
  "inexpensive",
  "cheap",
  "expensive",
  "fancy",
];

const C = [
  "red",
  "yellow",
  "blue",
  "green",
  "pink",
  "brown",
  "purple",
  "brown",
  "white",
  "black",
  "orange",
];

const N = [
  "table",
  "chair",
  "house",
  "bbq",
  "desk",
  "car",
  "pony",
  "cookie",
  "sandwich",
  "burger",
  "pizza",
  "mouse",
  "keyboard",
];

const random = (max: number) => Math.round(Math.random() * 1000) % max;

let nextId = 1;

function assignData(data: { id: number; label: string }[], offset = 0) {
  const count = data.length;
  for (let i = offset; i < count; i++) {
    data[i] = {
      id: nextId++,
      label: `${A[random(A.length)]} ${C[random(C.length)]} ${N[random(N.length)]}`,
    };
  }
  return data;
}

interface IData {
  id: number;
  label: string;
}

class Row {
  @State accessor props: {
    item: IData;
    selected: number;
    onSelect: (id: number) => void;
    onRemove: (id: number) => void;
  };
  constructor(p: {
    item: IData;
    selected: number;
    onSelect: (id: number) => void;
    onRemove: (id: number) => void;
  }) {
    this.props = p;
  }
  render() {
    const { item, selected, onSelect, onRemove } = this.props;
    const isSelected = selected === item.id;
    return (
      <tr className={isSelected ? "danger" : ""}>
        <td className="col-md-1">{item.id}</td>
        <td className="col-md-4">
          <a onClick={() => onSelect(item.id)}>{item.label}</a>
        </td>
        <td className="col-md-1">
          <a onClick={() => onRemove(item.id)}>
            <span
              className="glyphicon glyphicon-remove"
              aria-hidden="true"
            ></span>
          </a>
        </td>
        <td className="col-md-6"></td>
      </tr>
    );
  }
}

class App {
  @State accessor data: IData[] = [];
  @State accessor selected: number = 0;

  @Mutate
  run() {
    this.data = new Array(1000);
    assignData(this.data);
    this.selected = 0;
  }

  @Mutate
  runLots() {
    this.data = new Array(10000);
    assignData(this.data);
    this.selected = 0;
  }

  @Mutate
  add() {
    const offset = this.data.length;
    this.data.length += 1000;
    assignData(this.data, offset);
    this.selected = 0;
  }

  @Mutate
  update() {
    for (let i = 0; i < this.data.length; i += 10) {
      const r = this.data[i];
      this.data[i] = { id: r.id, label: r.label + " !!!" };
    }
  }

  @Mutate
  remove(id: number) {
    const idx = this.data.findIndex((d) => d.id === id);
    if (idx >= 0) this.data.splice(idx, 1);
  }

  @Mutate
  select(id: number) {
    this.selected = id;
  }

  @Mutate
  clear() {
    this.data = [];
    this.selected = 0;
  }

  @Mutate
  swapRows() {
    if (this.data.length > 998) {
      const tmp = this.data[1];
      this.data[1] = this.data[998];
      this.data[998] = tmp;
    }
  }

  render() {
    const { data, selected } = this;
    return (
      <div className="container">
        <div className="jumbotron">
          <div className="row">
            <div className="col-md-6">
              <h1>Sapling</h1>
            </div>
            <div className="col-md-6">
              <div className="row">
                <div className="col-sm-6 smallpad">
                  <button
                    type="button"
                    className="btn btn-primary btn-block"
                    id="run"
                    onClick={() => this.run()}
                  >
                    Create 1,000 rows
                  </button>
                </div>
                <div className="col-sm-6 smallpad">
                  <button
                    type="button"
                    className="btn btn-primary btn-block"
                    id="runlots"
                    onClick={() => this.runLots()}
                  >
                    Create 10,000 rows
                  </button>
                </div>
                <div className="col-sm-6 smallpad">
                  <button
                    type="button"
                    className="btn btn-primary btn-block"
                    id="add"
                    onClick={() => this.add()}
                  >
                    Append 1,000 rows
                  </button>
                </div>
                <div className="col-sm-6 smallpad">
                  <button
                    type="button"
                    className="btn btn-primary btn-block"
                    id="update"
                    onClick={() => this.update()}
                  >
                    Update every 10th row
                  </button>
                </div>
                <div className="col-sm-6 smallpad">
                  <button
                    type="button"
                    className="btn btn-primary btn-block"
                    id="clear"
                    onClick={() => this.clear()}
                  >
                    Clear
                  </button>
                </div>
                <div className="col-sm-6 smallpad">
                  <button
                    type="button"
                    className="btn btn-primary btn-block"
                    id="swaprows"
                    onClick={() => this.swapRows()}
                  >
                    Swap Rows
                  </button>
                </div>
              </div>
            </div>
          </div>
        </div>

        <table className="table table-hover table-striped test-data">
          <tbody>
            <For each={data} fallback={<tr />}>
              {(item) => (
                <Row
                  item={item}
                  selected={selected}
                  onSelect={(id) => this.select(id)}
                  onRemove={(id) => this.remove(id)}
                />
              )}
            </For>
          </tbody>
        </table>
        <span
          className="preloadicon glyphicon glyphicon-remove"
          aria-hidden="true"
        ></span>
      </div>
    );
  }
}

const el = document.getElementById("app");

if (el) {
  const root = createRoot(el);
  root.render(<App />);
}
