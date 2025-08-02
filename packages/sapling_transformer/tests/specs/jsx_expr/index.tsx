let some = () => {
  return 1;
};

function Expr() {
  const value = some();

  return <div>{value}</div>;
}
