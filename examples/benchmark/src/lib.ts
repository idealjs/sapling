export abstract class Component<
  Props extends object = Record<string, unknown>,
> {
  public props: Props;
  constructor(_props?: Props) {
    this.props = _props ?? ({} as Props);
  }
  abstract render(): unknown;
}
