const Demo = () => {
  const template = <h1>template</h1>;
  return (
    <div>
      {() => {
        return <span>demo</span>;
      }}
    </div>
  );
};

export default Demo;
