import { createProxy, createRef, createRoot, Ref } from "@idealjs/sapling";

let idCounter = 1;
const adjectives = [
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
  ],
  colours = [
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
  ],
  nouns = [
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

function _random(max: number) {
  return Math.round(Math.random() * 1000) % max;
}

function buildData(count: number) {
  let data = new Array<{
    id: number;
    label: Ref<string>;
  }>(count);
  for (let i = 0; i < count; i++) {
    data[i] = {
      id: idCounter++,
      label: createRef(
        `${adjectives[_random(adjectives.length)]} ${
          colours[_random(colours.length)]
        } ${nouns[_random(nouns.length)]}`,
      ),
    };
  }
  return data;
}

const Button = ({
  id,
  text,
  fn,
}: {
  id: string;
  text: string;
  fn: () => void;
}) => (
  <div className="col-sm-6 smallpad">
    <button
      id={id}
      className="btn btn-primary btn-block"
      type="button"
      onClick={fn}
    >
      {text}
    </button>
  </div>
);

const App = () => {
  const data = createRef<
    {
      id: number;
      label: Ref<string>;
    }[]
  >([]);
  const selected = createProxy<{ val: number }>();
  const run = () => (data.current = buildData(1000));
  const runLots = () => (data.current = buildData(10000));
  const add = () => (data.current = [...data.current, ...buildData(1000)]);
  const update = () => {
    for (let i = 0, len = data.current.length; i < len; i += 10) {
      data.current[i].label.current += " !!!";
    }
  };
  const swapRows = () => {
    if (data.current.length > 998) {
      let tmp = data.current[1];
      data.current[1] = data.current[998];
      data.current[998] = tmp;
    }
  };
  const clear = () => (data.current = []);
  const remove = (id: number) => {
    const idx = data.current.findIndex((d) => d.id === id);
    data.current.splice(idx, 1);
  };

  return (
    <div className="container">
      <div className="jumbotron">
        <div className="row">
          <div className="col-md-6">
            <h1>Sapling Keyed</h1>
          </div>
          <div className="col-md-6">
            <div className="row">
              <Button id="run" text="Create 1,000 rows" fn={run} />
              <Button id="runlots" text="Create 10,000 rows" fn={runLots} />
              <Button id="add" text="Append 1,000 rows" fn={add} />
              <Button id="update" text="Update every 10th row" fn={update} />
              <Button id="clear" text="Clear" fn={clear} />
              <Button id="swaprows" text="Swap Rows" fn={swapRows} />
            </div>
          </div>
        </div>
      </div>
      <table className="table table-hover table-striped test-data">
        <tbody>
          {() =>
            data.current.map((row) => {
              let rowId = row.id;
              return (
                <tr
                  key={rowId}
                  className={() => (selected.val === rowId ? "danger" : "")}
                >
                  <td className="col-md-1">{rowId}</td>
                  <td className="col-md-4">
                    <a
                      onClick={() => {
                        selected.val = rowId;
                      }}
                    >
                      {() => row.label.current}
                    </a>
                  </td>
                  <td className="col-md-1">
                    <a
                      onClick={() => {
                        remove(rowId);
                      }}
                    >
                      <span
                        className="glyphicon glyphicon-remove"
                        aria-hidden="true"
                      />
                    </a>
                  </td>
                  <td className="col-md-6" />
                </tr>
              );
            })
          }
        </tbody>
      </table>
      <span
        className="preloadicon glyphicon glyphicon-remove"
        aria-hidden="true"
      />
    </div>
  );
};

createRoot(document.getElementById("app")!).render(<App />);
