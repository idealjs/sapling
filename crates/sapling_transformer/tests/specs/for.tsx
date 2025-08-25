import { JSXElementType, SaplingElement } from "@idealjs/sapling/createElement";
import { JSX } from "@idealjs/sapling/jsx-runtime";
import { State } from "@idealjs/sapling-reactive";

interface IProps<T> {
  each: T[];
  fallback: JSX.Element;
  children: (item: T) => JSX.Element;
}

class For<T> {
  @State accessor props: IProps<T>;
  constructor(_props: IProps<T>) {
    this.props = _props;
  }
  render() {
    const { children } = this.props;
    return <div />;
  }
}

export default For;
