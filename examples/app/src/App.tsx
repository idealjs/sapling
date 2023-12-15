import "./App.css";
import Loading from "./components/Loading";

function App() {
  return (
    <div>
      {Array(1000)
        .fill(0)
        .map(() => {
          return <Loading />;
        })}
    </div>
  );
}

export default App;
