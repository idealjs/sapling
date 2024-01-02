const arrify = <T>(v: T | T[]) => {
  if (v == null) {
    return [];
  }
  if (Array.isArray(v)) {
    return v;
  }
  return [v];
};

export default arrify;
