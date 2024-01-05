import { describe, expect, it, vi } from "vitest";

import createElement, { useEffect } from "./createElement";
import { createState } from "./reactive";

describe("render test", () => {
  it("children", () => {
    const node = createElement("div", {
      children: [createElement("div")],
    });
    expect(node.el).toMatchInlineSnapshot(`
      <div>
        <div />
      </div>
    `);
  });

  it("children with string", () => {
    const node = createElement("div", {
      children: [
        createElement("div", { children: "hello world" }),
        createElement("div", { children: "hello world" }),
      ],
    });
    expect(node.el).toMatchInlineSnapshot(`
      <div>
        <div>
          hello world
        </div>
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
    expect(node.el).toMatchInlineSnapshot(`
      <div>
        <div>
          0
        </div>
      </div>
    `);
  });

  it("function child", () => {
    const node = createElement("div", {
      children: () =>
        createElement("div", {
          children: () => "hello world",
        }),
    });
    expect(node.el).toMatchInlineSnapshot(`
      <div>
        <div>
          hello world
        </div>
      </div>
    `);
  });

  it("function children", () => {
    const node = createElement("div", {
      children: () => [
        createElement("div", { children: () => 0 }),
        createElement("div", { children: () => 1 }),
      ],
    });
    expect(node.el).toMatchInlineSnapshot(`
      <div>
        <div>
          0
        </div>
        <div>
          1
        </div>
      </div>
    `);
  });
});

describe("reactive test", () => {
  it("function children reactive update", () => {
    const counter = createState(0);
    const node = createElement("div", {
      children: () => [
        createElement("div", {
          children: () => `counter ${counter.val}`,
        }),
      ],
    });
    expect(node.el).toMatchInlineSnapshot(`
      <div>
        <div>
          counter 0
        </div>
      </div>
    `);
    counter.val++;
    expect(node.el).toMatchInlineSnapshot(`
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
    expect(node.el).toMatchInlineSnapshot(`
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
    expect(node.el).toMatchInlineSnapshot(`
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
    expect(node.el).toMatchInlineSnapshot(`
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
            children: () => `item length ${items.val.length}`,
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
      vi.advanceTimersByTime(1000);
      items.val = [...(items.val ?? []), { id: items.val.length }];
    }
    expect(node.el).toMatchInlineSnapshot(`
      <div>
        <div>
          item length 10
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
      vi.advanceTimersByTime(1000);
      items.val = [...(items.val ?? []), { id: items.val.length }];
    }

    expect(node.el).toMatchInlineSnapshot(`
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

  it("Three Todo List with key", async () => {
    vi.useFakeTimers();

    const items = createState<{ id: number; hidden: boolean }[]>([
      { id: 1, hidden: false },
      { id: 2, hidden: false },
      { id: 3, hidden: true },
      { id: 4, hidden: false },
    ]);

    const updateList = () => {
      // const values = new Array(10).fill("").map((v, index) => {
      //   return {
      //     id: index,
      //     hidden: (Math.random() * 10) % 2 > 1,
      //   };
      // });
      // console.log("test test", (Math.random() * 10) % 2, values);
      // items.val = values;
      items.val = [
        { id: 1, hidden: false },
        { id: 2, hidden: false },
        { id: 3, hidden: false },
        { id: 4, hidden: false },
      ];
    };

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

    const TodoItem2 = (props: { name: number }) => {
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
          return `TodoItem2 ${name} counter ${state.val}`;
        },
      });
    };

    const TodoItem3 = (props: { name: number }) => {
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
          return `TodoItem3 ${name} counter ${state.val}`;
        },
      });
    };

    const TodoList = () => {
      return createElement("div", {
        children: [
          createElement("div", {
            children: () => `item length ${items.val.length}`,
          }),
          createElement("div", {
            children: [
              () =>
                items.val.map((item) => {
                  return item.hidden
                    ? null
                    : createElement(TodoItem, { name: item.id }, item.id);
                }),

              () =>
                items.val.map((item) => {
                  return item.hidden
                    ? null
                    : createElement(TodoItem2, { name: item.id }, item.id);
                }),
              () =>
                items.val.map((item) => {
                  return item.hidden
                    ? null
                    : createElement(TodoItem3, { name: item.id }, item.id);
                }),
            ],
          }),
        ],
      });
    };
    const node = createElement(TodoList);

    expect(node.el).toMatchInlineSnapshot(`
      <div>
        <div>
          item length 4
        </div>
        <div>
          <p>
            1 counter 0
          </p>
          <p>
            2 counter 0
          </p>
          <p>
            4 counter 0
          </p>
          <p>
            TodoItem2 1 counter 0
          </p>
          <p>
            TodoItem2 2 counter 0
          </p>
          <p>
            TodoItem2 4 counter 0
          </p>
          <p>
            TodoItem3 1 counter 0
          </p>
          <p>
            TodoItem3 2 counter 0
          </p>
          <p>
            TodoItem3 4 counter 0
          </p>
        </div>
      </div>
    `);

    vi.advanceTimersByTime(5000);
    updateList();
    vi.advanceTimersByTime(5000);

    expect(node.el).toMatchInlineSnapshot(`
      <div>
        <div>
          item length 4
        </div>
        <div>
          <p>
            1 counter 10
          </p>
          <p>
            2 counter 10
          </p>
          <p>
            3 counter 5
          </p>
          <p>
            4 counter 10
          </p>
          <p>
            TodoItem2 1 counter 10
          </p>
          <p>
            TodoItem2 2 counter 10
          </p>
          <p>
            TodoItem2 3 counter 5
          </p>
          <p>
            TodoItem2 4 counter 10
          </p>
          <p>
            TodoItem3 1 counter 10
          </p>
          <p>
            TodoItem3 2 counter 10
          </p>
          <p>
            TodoItem3 3 counter 5
          </p>
          <p>
            TodoItem3 4 counter 10
          </p>
        </div>
      </div>
    `);
    vi.useRealTimers();
  });
});

describe("dispose test", () => {
  it("Hidden Function Children, Should not dispose effect", async () => {
    const mockFn = vi.fn();
    const mockDispose = vi.fn();
    const hidden = createState(false);
    const counter = createState(0);
    const App = () => {
      useEffect(() => {
        mockFn(counter.val);
        return mockDispose;
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
    expect(node.el).toMatchInlineSnapshot(`
      <div>
        <div>
          hello
        </div>
      </div>
    `);
    expect(mockFn).toBeCalledTimes(1);
    counter.val++;
    expect(mockFn).toBeCalledTimes(2);
    hidden.val = true;
    counter.val++;
    expect(mockFn).toBeCalledTimes(3);
    hidden.val = false;
    counter.val++;
    expect(mockFn).toBeCalledTimes(4);
  });

  it("Hidden Counter", async () => {
    const count = createState(0);
    const hidden = createState(false);

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
    expect(node.el).toMatchInlineSnapshot(`
      <div>
        <div>
          0
        </div>
      </div>
    `);
    expect(mockFn).toBeCalledTimes(1);

    count.val++;
    expect(node.el).toMatchInlineSnapshot(`
      <div>
        <div>
          1
        </div>
      </div>
    `);
    expect(mockFn).toBeCalledTimes(2);

    hidden.val = !hidden.val;
    count.val++;
    expect(node.el).toMatchInlineSnapshot("<div />");
    expect(mockFn).toBeCalledTimes(2);
    expect(count.val).toBe(2);
  });

  it("Hidden Counter With Wrapper", async () => {
    const count = createState(0);
    const mockFn = vi.fn();
    const mockDispose = vi.fn();
    const Counter = () => {
      useEffect(() => {
        mockFn(count.val);
        return () => {
          mockDispose();
        };
      });
      return createElement("div", {
        children: () => count.val,
      });
    };
    const Wrapper = () => {
      return createElement(Counter);
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
    expect(node.el).toMatchInlineSnapshot(`
      <div>
        <div>
          0
        </div>
      </div>
    `);
    count.val++;
    expect(node.el).toMatchInlineSnapshot(`
      <div>
        <div>
          1
        </div>
      </div>
    `);
    count.val++;
    expect(node.el).toMatchInlineSnapshot(`
      <div>
        <div>
          2
        </div>
      </div>
    `);
    hidden.val = true;
    expect(mockDispose).toBeCalledTimes(1);
    expect(node.el).toMatchInlineSnapshot("<div />");
    expect(mockFn).toBeCalledTimes(3);
    count.val++;
    expect(mockDispose).toBeCalledTimes(1);
    expect(node.el).toMatchInlineSnapshot("<div />");
    expect(mockFn).toBeCalledTimes(3);
  });

  it("Hidden Counter With Nested Function Children", async () => {
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
    const hidden = createState(false);
    const InnterWrapper = () => {
      return createElement("div", {
        children: () => {
          return createElement(Counter);
        },
      });
    };
    const App = () => {
      useEffect(() => {
        mockFn2(hidden.val);
      });

      return createElement("div", {
        children: () => {
          return hidden.val ? null : createElement(InnterWrapper);
        },
      });
    };
    const node = createElement(App);
    expect(node.el).toMatchInlineSnapshot(`
      <div>
        <div>
          <div>
            0
          </div>
        </div>
      </div>
    `);
    expect(mockFn).toBeCalledTimes(1);

    count.val++;
    expect(node.el).toMatchInlineSnapshot(`
      <div>
        <div>
          <div>
            1
          </div>
        </div>
      </div>
    `);
    expect(mockFn).toBeCalledTimes(2);

    hidden.val = true;
    count.val++;
    expect(node.el).toMatchInlineSnapshot("<div />");
    expect(mockFn).toBeCalledTimes(2);
    expect(count.val).toBe(2);
  });
});
