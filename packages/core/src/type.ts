export type TagNameMap = HTMLElementTagNameMap & SVGElementTagNameMap;

export type Primitive = string | number | boolean | bigint;

export type OrFunction<T> = T | (() => T);

export type Key = number | string | symbol;

export type Arrify<T> = T extends [] ? T : T[];

