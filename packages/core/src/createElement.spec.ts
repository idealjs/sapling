import { describe, expect, it, vi } from "vitest";

import createElement, { useEffect } from "./createElement";
import { createState, State } from "./reactive";
import { sleep } from "./utils/sleep";

describe("unit test", () => {
  it("children", () => {
    const node = createElement("div", {
      children: [createElement("div")],
    });
    expect(node.childNode).toMatchInlineSnapshot(`
      <div>
        <div />
      </div>
    `);
  });

  it("children with string", () => {
    const node = createElement("div", {
      children: [createElement("div", { children: "hello world" })],
    });
    expect(node.childNode).toMatchInlineSnapshot(`
      <div>
        <div>
          hello world
        </div>
      </div>
    `);
  });

  it("children with number", () => {
    const node = createElement("div", {
      children: [createElement("div", { children: 0 })],
    });
    expect(node.childNode).toMatchInlineSnapshot(`
      <div>
        <div>
          0
        </div>
      </div>
    `);
  });

  it("function child", () => {
    const node = createElement("div", {
      children: () => createElement("div"),
    });
    expect(node.childNode).toMatchInlineSnapshot(`
      <div>
        <div />
      </div>
    `);
  });

  it("function children", () => {
    const node = createElement("div", {
      children: () => [createElement("div"), createElement("div")],
    });
    expect(node.childNode).toMatchInlineSnapshot(`
      <div>
        <div />
        <div />
      </div>
    `);
  });

  it("function children reactive update", () => {
    const counter = createState(0);
    const node = createElement("div", {
      children: () => [
        createElement("div", {
          children: () => `counter ${counter.val}`,
        }),
      ],
    });
    expect(node.childNode).toMatchInlineSnapshot(`
      <div>
        <div>
          counter 0
        </div>
      </div>
    `);
    counter.val++;
    expect(node.childNode).toMatchInlineSnapshot(`
      <div>
        <div>
          counter 1
        </div>
      </div>
    `);
  });

  it("function children reactive update list", () => {
    const list = createState([
      {
        key: 1,
        hidden: true,
      },
      {
        key: 2,
        hidden: false,
      },
      {
        key: 3,
        hidden: true,
      },
      {
        key: 4,
        hidden: false,
      },
    ]);
    const node = createElement("div", {
      children: () =>
        list.val.map((v) => {
          return v.hidden
            ? null
            : createElement("div", {
                key: v.key,
                children: v.key,
              });
        }),
    });
    expect(node.childNode).toMatchInlineSnapshot(`
      <div>
        <div>
          2
        </div>
        <div>
          4
        </div>
      </div>
    `);
    list.val = [
      {
        key: 1,
        hidden: false,
      },
      {
        key: 2,
        hidden: true,
      },
      {
        key: 3,
        hidden: false,
      },
      {
        key: 4,
        hidden: true,
      },
    ];
    expect(node.childNode).toMatchInlineSnapshot(`
      <div>
        <div>
          1
        </div>
        <div>
          3
        </div>
      </div>
    `);
    list.val = [
      {
        key: 1,
        hidden: false,
      },
      {
        key: 5,
        hidden: false,
      },
      {
        key: 6,
        hidden: false,
      },
      {
        key: 4,
        hidden: true,
      },
    ];
    expect(node.childNode).toMatchInlineSnapshot(`
      <div>
        <div>
          1
        </div>
        <div>
          5
        </div>
        <div>
          6
        </div>
      </div>
    `);
  });

  it("Todo List with key", async () => {
    vi.useFakeTimers();
    const TodoItem = (props: { name: number }) => {
      const { name } = props;
      const state = createState(0);
      useEffect(() => {
        const handler = setInterval(() => {
          state.val++;
        }, 1000);
        return () => {
          clearInterval(handler);
        };
      });
      return createElement("p", {
        children: () => {
          return `${name} counter ${state.val}`;
        },
      });
    };

    const items = createState<{ id: number }[]>([]);
    const TodoList = () => {
      return createElement("div", {
        children: [
          createElement("div", {
            children: () => `add item ${items.val.length}`,
          }),
          createElement("div", {
            children: () =>
              items.val.map((item) => {
                return createElement(TodoItem, { name: item.id }, item.id);
              }),
          }),
        ],
      });
    };
    const node = createElement(TodoList);
    for (let index = 0; index < 10; index++) {
      await sleep(1000);
      items.val = [...(items.val ?? []), { id: items.val.length }];
    }
    expect(node.childNode).toMatchInlineSnapshot(`
      <div>
        <div>
          add item 10
        </div>
        <div>
          <p>
            0 counter 9
          </p>
          <p>
            1 counter 8
          </p>
          <p>
            2 counter 7
          </p>
          <p>
            3 counter 6
          </p>
          <p>
            4 counter 5
          </p>
          <p>
            5 counter 4
          </p>
          <p>
            6 counter 3
          </p>
          <p>
            7 counter 2
          </p>
          <p>
            8 counter 1
          </p>
          <p>
            9 counter 0
          </p>
        </div>
      </div>
    `);
    vi.useRealTimers();
  });

  it("Todo List Without key", async () => {
    vi.useFakeTimers();
    const TodoItem = (props: { name: number }) => {
      const { name } = props;
      const state = createState(0);
      useEffect(() => {
        const handler = setInterval(() => {
          state.val++;
        }, 1000);
        return () => {
          clearInterval(handler);
        };
      });
      return createElement("p", {
        children: () => {
          return `${name} counter ${state.val}`;
        },
      });
    };

    const items = createState<{ id: number }[]>([]);
    const TodoList = () => {
      return createElement("div", {
        children: [
          createElement("div", {
            children: () => `add item ${items.val.length}`,
          }),
          createElement("div", {
            children: () =>
              items.val.map((item) => {
                return createElement(TodoItem, { name: item.id });
              }),
          }),
        ],
      });
    };
    const node = createElement(TodoList);
    for (let index = 0; index < 10; index++) {
      await sleep(1000);
      items.val = [...(items.val ?? []), { id: items.val.length }];
    }

    expect(node.childNode).toMatchInlineSnapshot(`
      <div>
        <div>
          add item 10
        </div>
        <div>
          <p>
            0 counter 0
          </p>
          <p>
            1 counter 0
          </p>
          <p>
            2 counter 0
          </p>
          <p>
            3 counter 0
          </p>
          <p>
            4 counter 0
          </p>
          <p>
            5 counter 0
          </p>
          <p>
            6 counter 0
          </p>
          <p>
            7 counter 0
          </p>
          <p>
            8 counter 0
          </p>
          <p>
            9 counter 0
          </p>
        </div>
      </div>
    `);
    vi.useRealTimers();
  });

  it("Hidden Counter", async () => {
    vi.useFakeTimers();
    const count = createState(0);
    const mockFn = vi.fn();
    const Counter = () => {
      useEffect(() => {
        mockFn(count.val);
      });
      return createElement("div", {
        children: () => count.val,
      });
    };
    const mockFn2 = vi.fn();

    const App = () => {
      const hidden = createState(false);
      useEffect(() => {
        setTimeout(() => {
          hidden.val = true;
        }, 1000);
      });
      useEffect(() => {
        mockFn2(hidden.val);
      });

      return createElement("div", {
        children: () => {
          return hidden.val ? null : createElement(Counter);
        },
      });
    };
    const node = createElement(App);
    expect(node.childNode).toMatchInlineSnapshot(`
      <div>
        <div>
          0
        </div>
      </div>
    `);
    expect(mockFn).toBeCalledTimes(1);

    count.val++;
    expect(node.childNode).toMatchInlineSnapshot(`
      <div>
        <div>
          1
        </div>
      </div>
    `);
    expect(mockFn).toBeCalledTimes(2);

    vi.advanceTimersToNextTimer();
    count.val++;
    expect(node.childNode).toMatchInlineSnapshot("<div />");
    expect(mockFn).toBeCalledTimes(2);
    expect(count.val).toBe(2);
    vi.useRealTimers();
  });

  it("Hidden CountDown With Wrapper", async () => {
    vi.useFakeTimers();
    const mockFn = vi.fn((state: State<number>) => {
      state.val--;
    });
    const mockClearInterval = vi.fn((handler: NodeJS.Timeout) => {
      clearInterval(handler);
    });
    const CountDown = () => {
      const count = createState(10);
      useEffect(() => {
        const handler = setInterval(() => {
          mockFn(count);
        }, 1000);
        return () => {
          mockClearInterval(handler);
        };
      });
      return createElement("div", {
        children: () => count.val,
      });
    };
    const Wrapper = () => {
      return createElement(CountDown);
    };
    const hidden = createState(false);

    const App = () => {
      return createElement("div", {
        children: () => {
          return hidden.val ? null : createElement(Wrapper);
        },
      });
    };

    const node = createElement(App);
    expect(node.childNode).toMatchInlineSnapshot(`
      <div>
        <div>
          10
        </div>
      </div>
    `);
    vi.advanceTimersByTime(1000);
    expect(node.childNode).toMatchInlineSnapshot(`
      <div>
        <div>
          9
        </div>
      </div>
    `);
    vi.advanceTimersByTime(1000);
    expect(node.childNode).toMatchInlineSnapshot(`
      <div>
        <div>
          8
        </div>
      </div>
    `);
    hidden.val = true;
    expect(mockClearInterval).toBeCalledTimes(1);
    expect(node.childNode).toMatchInlineSnapshot("<div />");
    expect(mockFn).toBeCalledTimes(2);
    vi.advanceTimersByTime(1000);
    expect(mockClearInterval).toBeCalledTimes(1);
    expect(node.childNode).toMatchInlineSnapshot("<div />");
    expect(mockFn).toBeCalledTimes(2);
  });

  it("Should not dispose effect", async () => {
    const mockFn = vi.fn();
    const hidden = createState(false);

    const App = () => {
      useEffect(() => {
        const handler = setInterval(mockFn, 1000);
        return () => clearInterval(handler);
      });

      return createElement("div", {
        children: () => {
          return hidden.val
            ? null
            : createElement("div", { children: "hello" });
        },
      });
    };
    const node = createElement(App);
    expect(node.childNode).toMatchInlineSnapshot(`
      <div>
        <div>
          hello
        </div>
      </div>
    `);
    expect(mockFn).toBeCalledTimes(0);
    await sleep(1000);
    expect(mockFn).toBeCalledTimes(1);
    hidden.val = true;
    await sleep(1000);
    expect(mockFn).toBeCalledTimes(2);
    hidden.val = false;
    await sleep(1000);
    expect(mockFn).toBeCalledTimes(3);
  });
  it("Should not dispose effect with state", async () => {
    const hidden = createState(false);

    const App = () => {
      const state = createState(0);

      useEffect(() => {
        const handler = setInterval(() => {
          state.val++;
        }, 1000);
        return () => clearInterval(handler);
      });

      return createElement("div", {
        children: () => {
          return hidden.val
            ? null
            : createElement("div", { children: () => state.val });
        },
      });
    };
    const node = createElement(App);
    expect(node.childNode).toMatchInlineSnapshot(`
      <div>
        <div>
          0
        </div>
      </div>
    `);
    await sleep(1000);
    expect(node.childNode).toMatchInlineSnapshot(`
      <div>
        <div>
          1
        </div>
      </div>
    `);
    hidden.val = true;
    await sleep(1000);
    expect(node.childNode).toMatchInlineSnapshot("<div />");
    hidden.val = false;
    await sleep(1000);
    expect(node.childNode).toMatchInlineSnapshot(`
      <div>
        <div>
          3
        </div>
      </div>
    `);
  });
});
