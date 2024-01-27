import {
  createRoot,
  createSignal,
  derive,
  Getter,
  Setter,
  Suspense,
} from "@idealjs/sapling";
// import debounce from "lodash.debounce";

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
    label: Getter<string>;
    setLabel: Setter<string>;
  }>(count);
  for (let i = 0; i < count; i++) {
    const [label, setLabel] = createSignal(
      `${adjectives[_random(adjectives.length)]} ${
        colours[_random(colours.length)]
      } ${nouns[_random(nouns.length)]}`,
    );
    data[i] = {
      id: idCounter++,
      label,
      setLabel,
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
  const [data, setData] = createSignal<
    {
      id: number;
      label: Getter<string>;
      setLabel: Setter<string>;
    }[]
  >([]);
  const [selected, setSelected] = createSignal<number | null>(null);
  const run = () => setData(buildData(1000));
  const runLots = () => setData(buildData(10000));
  const add = () => setData((d) => [...d, ...buildData(1000)]);
  const update = () => {
    for (let i = 0, d = data(), len = d.length; i < len; i += 10)
      d[i].setLabel((l) => l + " !!!");
  };
  const swapRows = () => {
    const d = data().slice();
    if (d.length > 998) {
      let tmp = d[1];
      d[1] = d[998];
      d[998] = tmp;
      setData(d);
    }
  };
  const clear = () => setData([]);
  const remove = (id: number) =>
    setData((d) => {
      const idx = d.findIndex((d) => d.id === id);
      return [...d.slice(0, idx), ...d.slice(idx + 1)];
    });
  const isSelected = derive(() => selected != null);

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
          <Suspense>
            {() =>
              data().map((row) => {
                let rowId = row.id;
                return (
                  <tr
                    key={rowId}
                    className={() => (isSelected.val ? "danger" : "")}
                  >
                    <td className="col-md-1">{rowId}</td>
                    <td className="col-md-4">
                      <a
                        onClick={() => {
                          setSelected(rowId);
                        }}
                      />
                      {() => row.label()}
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
          </Suspense>
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
