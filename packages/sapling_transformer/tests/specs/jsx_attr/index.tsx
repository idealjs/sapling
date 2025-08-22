function Attr() {
  return <button type="button">attr</button>;
}

let someStyle = { color: "red", fontSize: "16px" };

function Attr2() {
  return <div style={someStyle}>attr</div>;
}

let getSomeStyle = () => someStyle;

function Attr3() {
  return <div style={getSomeStyle()}>attr</div>;
}

function Attr4() {
  return <div style={getSomeStyle()} />;
}
