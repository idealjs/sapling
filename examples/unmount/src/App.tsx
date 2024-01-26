import Test1 from "./components/Test1";
import Test2 from "./components/Test2";
import Test3 from "./components/Test3";

const App = () => {
  return (
    <div style={{ display: "flex" }}>
      <Test1>
        <Test2>
          <Test3 />
        </Test2>
      </Test1>
    </div>
  );
};

export default App;
