let some = () => {
  return 1;
};

function Expr() {
  const value = some();

  return <div>{value}</div>;
}

const Expr2 = () => {
  return <div>{some()}</div>;
};

const Expr3 = () => {
  const some = () => {
    return 2;
  };
  return <div>{some()}</div>;
};
