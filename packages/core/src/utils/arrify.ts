const arrify = <T>(v: T | T[]) => {
  if (Array.isArray(v)) {
    return v;
  }
  return [v];
};

export default arrify;
