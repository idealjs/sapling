import { createProxy, createRef } from "@idealjs/sapling-reactive";
import { describe, expect, it, vi } from "vitest";

import createElement, { useEffect } from "./createElement";
import createRoot from "./createRoot";

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

  it("ref test", () => {
    const ref = createRef<HTMLDivElement>(null);
    const node = createElement("div", {
      ref: ref,
      children: [createElement("div")],
    });

    expect(node.el).toMatchInlineSnapshot(`
      <div>
        <div />
      </div>
    `);
    expect(ref.current).toMatchInlineSnapshot(`
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

  it("function children update", () => {
    const counter = createProxy({ val: 0 });
    const App = () => {
      return createElement("div", {
        children: [
          () => counter.val,
          createElement("div", { children: "hello" }),
        ],
      });
    };
    const body = document.createElement("body");
    createRoot(body).render(createElement(App));

    expect(body).toMatchInlineSnapshot(`
      <body>
        <div>
          0
          <div>
            hello
          </div>
        </div>
      </body>
    `);
    counter.val++;
    expect(body).toMatchInlineSnapshot(`
      <body>
        <div>
          1
          <div>
            hello
          </div>
        </div>
      </body>
    `);
  });

  it("function children update at start", () => {
    const counter = createProxy({ val: 0 });
    const App = () => {
      return createElement("div", {
        children: [
          createElement("div", { children: "hello" }),
          () => counter.val,
          createElement("div", { children: "hello" }),
        ],
      });
    };
    const body = document.createElement("body");
    createRoot(body).render(createElement(App));

    expect(body).toMatchInlineSnapshot(`
      <body>
        <div>
          <div>
            hello
          </div>
          0
          <div>
            hello
          </div>
        </div>
      </body>
    `);
    counter.val++;
    expect(body).toMatchInlineSnapshot(`
      <body>
        <div>
          <div>
            hello
          </div>
          1
          <div>
            hello
          </div>
        </div>
      </body>
    `);
  });

  it("component return array", () => {
    const Component = () => {
      return [
        createElement("div", { children: "A" }),
        createElement("div", { children: "B" }),
        createElement("div", { children: "C" }),
      ];
    };
    const node = createElement("div", {
      children: [
        createElement("div", {
          children: () => createElement(Component),
        }),
        createElement("div", {
          children: () => [createElement(Component)],
        }),
        createElement(Component),
      ],
    });
    expect(node.el).toMatchInlineSnapshot(`
      <div>
        <div>
          <div>
            A
          </div>
          <div>
            B
          </div>
          <div>
            C
          </div>
        </div>
        <div>
          <div>
            A
          </div>
          <div>
            B
          </div>
          <div>
            C
          </div>
        </div>
        <div>
          A
        </div>
        <div>
          B
        </div>
        <div>
          C
        </div>
      </div>
    `);
  });
});

describe("reactive test", () => {
  it("function children reactive update", () => {
    const counter = createProxy({ val: 0 });
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
    const list = createProxy([
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
        list.map((v) => {
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
    list[0].hidden = false;
    list[1].hidden = true;
    list[2].hidden = false;
    list[3].hidden = true;
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
    list[0].hidden = false;
    list[1] = {
      key: 5,
      hidden: false,
    };
    list[2] = {
      key: 6,
      hidden: false,
    };

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
      const state = createProxy({ val: 0 });
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

    const items = createProxy<{ id: number }[]>([]);
    const App = () => {
      return createElement("div", {
        children: [
          createElement("div", {
            children: () => `item length ${items.length}`,
          }),
          createElement("div", {
            children: () =>
              items.map((item) => {
                return createElement(TodoItem, { name: item.id }, item.id);
              }),
          }),
        ],
      });
    };

    const body = document.createElement("body");
    createRoot(body).render(createElement(App));

    for (let index = 0; index < 10; index++) {
      vi.advanceTimersByTime(1000);
      items.push({ id: items.length });
    }
    expect(body).toMatchInlineSnapshot(`
      <body>
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
      </body>
    `);
    vi.useRealTimers();
  });

  it("Todo List Without key", async () => {
    vi.useFakeTimers();
    const TodoItem = (props: { name: number }) => {
      const { name } = props;
      const state = createProxy({ val: 0 });
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

    const items = createProxy<{ id: number }[]>([]);
    const App = () => {
      return createElement("div", {
        children: [
          createElement("div", {
            children: () => `add item ${items.length}`,
          }),
          createElement("div", {
            children: () =>
              items.map((item) => {
                return createElement(TodoItem, { name: item.id });
              }),
          }),
        ],
      });
    };

    const body = document.createElement("body");
    createRoot(body).render(createElement(App));

    for (let index = 0; index < 10; index++) {
      vi.advanceTimersByTime(1000);
      items.push({ id: items.length });
    }

    expect(body).toMatchInlineSnapshot(`
      <body>
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
      </body>
    `);
    vi.useRealTimers();
  });

  it("Three Todo List with key", async () => {
    vi.useFakeTimers();

    const items = createProxy<{ id: number; hidden: boolean }[]>([
      { id: 1, hidden: false },
      { id: 2, hidden: false },
      { id: 3, hidden: true },
      { id: 4, hidden: false },
    ]);

    const updateList = () => {
      items[2].hidden = false;
    };

    const TodoItem = (props: { name: number }) => {
      const { name } = props;
      const state = createProxy({ val: 0 });
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
          return `TodoItem1 ${name} counter ${state.val}`;
        },
      });
    };

    const TodoItem2 = (props: { name: number }) => {
      const { name } = props;
      const state = createProxy({ val: 0 });
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

    const App = () => {
      return createElement("div", {
        children: [
          createElement("div", {
            children: () => `item length ${items.length}`,
          }),
          createElement("div", {
            children: [
              () =>
                items.map((item) => {
                  return item.hidden
                    ? null
                    : createElement(TodoItem, { name: item.id }, item.id);
                }),

              () =>
                items.map((item) => {
                  return item.hidden
                    ? null
                    : createElement(TodoItem2, { name: item.id }, item.id);
                }),
            ],
          }),
        ],
      });
    };

    const body = document.createElement("body");
    createRoot(body).render(createElement(App));

    expect(body).toMatchInlineSnapshot(`
      <body>
        <div>
          <div>
            item length 4
          </div>
          <div>
            <p>
              TodoItem1 1 counter 0
            </p>
            <p>
              TodoItem1 2 counter 0
            </p>
            <p>
              TodoItem1 4 counter 0
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
          </div>
        </div>
      </body>
    `);

    vi.advanceTimersByTime(5000);
    updateList();
    vi.advanceTimersByTime(5000);

    expect(body).toMatchInlineSnapshot(`
      <body>
        <div>
          <div>
            item length 4
          </div>
          <div>
            <p>
              TodoItem1 1 counter 10
            </p>
            <p>
              TodoItem1 2 counter 10
            </p>
            <p>
              TodoItem1 3 counter 5
            </p>
            <p>
              TodoItem1 4 counter 10
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
          </div>
        </div>
      </body>
    `);
    vi.useRealTimers();
  });
});

describe("dispose test", () => {
  it("Hidden Function Children, Should not dispose effect", async () => {
    const mockFn = vi.fn();
    const mockDispose = vi.fn();
    const hidden = createProxy({ val: false });
    const counter = createProxy({ val: 0 });
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
    const body = document.createElement("body");
    createRoot(body).render(createElement(App));
    expect(body).toMatchInlineSnapshot(`
      <body>
        <div>
          <div>
            hello
          </div>
        </div>
      </body>
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
    const count = createProxy({ val: 0 });
    const hidden = createProxy({ val: false });

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
    const body = document.createElement("body");
    createRoot(body).render(createElement(App));

    expect(body).toMatchInlineSnapshot(`
      <body>
        <div>
          <div>
            0
          </div>
        </div>
      </body>
    `);
    expect(mockFn).toBeCalledTimes(1);

    count.val++;
    expect(body).toMatchInlineSnapshot(`
      <body>
        <div>
          <div>
            1
          </div>
        </div>
      </body>
    `);
    expect(mockFn).toBeCalledTimes(2);

    hidden.val = !hidden.val;
    count.val++;
    expect(body).toMatchInlineSnapshot(`
      <body>
        <div />
      </body>
    `);
    expect(mockFn).toBeCalledTimes(2);
    expect(count.val).toBe(2);
  });

  it("Hidden Counter With Wrapper", async () => {
    const count = createProxy({ val: 0 });
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
    const hidden = createProxy({ val: false });

    const App = () => {
      return createElement("div", {
        children: () => {
          return hidden.val ? null : createElement(Wrapper);
        },
      });
    };

    const body = document.createElement("body");
    createRoot(body).render(createElement(App));

    expect(body).toMatchInlineSnapshot(`
      <body>
        <div>
          <div>
            0
          </div>
        </div>
      </body>
    `);
    count.val++;
    expect(body).toMatchInlineSnapshot(`
      <body>
        <div>
          <div>
            1
          </div>
        </div>
      </body>
    `);
    count.val++;
    expect(body).toMatchInlineSnapshot(`
      <body>
        <div>
          <div>
            2
          </div>
        </div>
      </body>
    `);
    hidden.val = true;
    expect(mockDispose).toBeCalledTimes(1);
    expect(body).toMatchInlineSnapshot(`
      <body>
        <div />
      </body>
    `);
    expect(mockFn).toBeCalledTimes(3);
    count.val++;
    expect(mockDispose).toBeCalledTimes(1);
    expect(body).toMatchInlineSnapshot(`
      <body>
        <div />
      </body>
    `);
    expect(mockFn).toBeCalledTimes(3);
  });

  it("Hidden Counter With Nested Function Children", async () => {
    const count = createProxy({ val: 0 });
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
    const hidden = createProxy({ val: false });
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
    const body = document.createElement("body");
    createRoot(body).render(createElement(App));

    expect(body).toMatchInlineSnapshot(`
      <body>
        <div>
          <div>
            <div>
              0
            </div>
          </div>
        </div>
      </body>
    `);
    expect(mockFn).toBeCalledTimes(1);

    count.val++;
    expect(body).toMatchInlineSnapshot(`
      <body>
        <div>
          <div>
            <div>
              1
            </div>
          </div>
        </div>
      </body>
    `);
    expect(mockFn).toBeCalledTimes(2);

    hidden.val = true;
    count.val++;
    expect(body).toMatchInlineSnapshot(`
      <body>
        <div />
      </body>
    `);
    expect(mockFn).toBeCalledTimes(2);
    expect(count.val).toBe(2);
  });
});
