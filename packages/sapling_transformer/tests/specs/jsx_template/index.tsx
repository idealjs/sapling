let some = () => {
  return 1;
};

function Counter() {
  const value = some();

  return <button type="button">{value}</button>;
}
