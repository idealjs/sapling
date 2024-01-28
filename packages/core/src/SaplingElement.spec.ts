import { describe, expect, it } from "vitest";

import createElement from "./createElement";

describe("sapling element test", () => {
  it("children", () => {
    const App = () =>
      createElement("div", {
        children: createElement("p"),
      });

    const saplingElement = createElement(App);

    expect(saplingElement).toMatchInlineSnapshot(`
      Object {
        "staticContainer": true,
        "el": null,
        "children": Set {
          Object {
            "staticContainer": false,
            "el": HTMLDivElement {},
            "children": Set {
              Object {
                "staticContainer": false,
                "el": HTMLParagraphElement {},
                "children": Set {},
              },
            },
          },
        },
      }
    `);
  });

  it("children with string", () => {
    const App = () =>
      createElement("div", {
        children: createElement("p", { children: "hello world" }),
      });

    const saplingElement = createElement(App);

    expect(saplingElement).toMatchInlineSnapshot(`
      Object {
        "staticContainer": true,
        "el": null,
        "children": Set {
          Object {
            "staticContainer": false,
            "el": HTMLDivElement {},
            "children": Set {
              Object {
                "staticContainer": false,
                "el": HTMLParagraphElement {},
                "children": Set {
                  Object {
                    "staticContainer": false,
                    "el": Text {},
                    "children": Set {},
                  },
                },
              },
            },
          },
        },
      }
    `);
  });

  it("children with number", () => {
    const App = () =>
      createElement("div", {
        children: createElement("p", { children: 0 }),
      });

    const saplingElement = createElement(App);

    expect(saplingElement).toMatchInlineSnapshot(`
      Object {
        "staticContainer": true,
        "el": null,
        "children": Set {
          Object {
            "staticContainer": false,
            "el": HTMLDivElement {},
            "children": Set {
              Object {
                "staticContainer": false,
                "el": HTMLParagraphElement {},
                "children": Set {
                  Object {
                    "staticContainer": false,
                    "el": Text {},
                    "children": Set {},
                  },
                },
              },
            },
          },
        },
      }
    `);
  });

  it("children as function return string", () => {
    const App = () =>
      createElement("div", {
        children: () =>
          createElement("p", {
            children: () => "hello world",
          }),
      });

    const saplingElement = createElement(App);

    expect(saplingElement).toMatchInlineSnapshot(`
      Object {
        "staticContainer": true,
        "el": null,
        "children": Set {
          Object {
            "staticContainer": false,
            "el": HTMLDivElement {},
            "children": Set {
              Object {
                "staticContainer": false,
                "el": HTMLParagraphElement {},
                "children": Set {
                  Object {
                    "staticContainer": false,
                    "el": Text {},
                    "children": Set {},
                  },
                },
              },
            },
          },
        },
      }
    `);
  });

  it("children as array", () => {
    const App = () =>
      createElement("div", {
        children: [createElement("p"), createElement("p")],
      });

    const saplingElement = createElement(App);

    expect(saplingElement).toMatchInlineSnapshot(`
      Object {
        "staticContainer": true,
        "el": null,
        "children": Set {
          Object {
            "staticContainer": false,
            "el": HTMLDivElement {},
            "children": Set {
              Object {
                "staticContainer": true,
                "el": null,
                "children": Set {
                  Object {
                    "staticContainer": false,
                    "el": HTMLParagraphElement {},
                    "children": Set {},
                  },
                  Object {
                    "staticContainer": false,
                    "el": HTMLParagraphElement {},
                    "children": Set {},
                  },
                },
              },
            },
          },
        },
      }
    `);
  });

  it("children as array with string", () => {
    const App = () =>
      createElement("div", {
        children: [
          createElement("p", { children: "hello world" }),
          createElement("p", { children: "hello world" }),
        ],
      });

    const saplingElement = createElement(App);

    expect(saplingElement).toMatchInlineSnapshot(`
      Object {
        "staticContainer": true,
        "el": null,
        "children": Set {
          Object {
            "staticContainer": false,
            "el": HTMLDivElement {},
            "children": Set {
              Object {
                "staticContainer": true,
                "el": null,
                "children": Set {
                  Object {
                    "staticContainer": false,
                    "el": HTMLParagraphElement {},
                    "children": Set {
                      Object {
                        "staticContainer": false,
                        "el": Text {},
                        "children": Set {},
                      },
                    },
                  },
                  Object {
                    "staticContainer": false,
                    "el": HTMLParagraphElement {},
                    "children": Set {
                      Object {
                        "staticContainer": false,
                        "el": Text {},
                        "children": Set {},
                      },
                    },
                  },
                },
              },
            },
          },
        },
      }
    `);
  });

  it("children as array with number", () => {
    const App = () =>
      createElement("div", {
        children: [
          createElement("p", { children: 0 }),
          createElement("p", { children: 1 }),
        ],
      });

    const saplingElement = createElement(App);

    expect(saplingElement).toMatchInlineSnapshot(`
      Object {
        "staticContainer": true,
        "el": null,
        "children": Set {
          Object {
            "staticContainer": false,
            "el": HTMLDivElement {},
            "children": Set {
              Object {
                "staticContainer": true,
                "el": null,
                "children": Set {
                  Object {
                    "staticContainer": false,
                    "el": HTMLParagraphElement {},
                    "children": Set {
                      Object {
                        "staticContainer": false,
                        "el": Text {},
                        "children": Set {},
                      },
                    },
                  },
                  Object {
                    "staticContainer": false,
                    "el": HTMLParagraphElement {},
                    "children": Set {
                      Object {
                        "staticContainer": false,
                        "el": Text {},
                        "children": Set {},
                      },
                    },
                  },
                },
              },
            },
          },
        },
      }
    `);
  });

  it("children as function return array saplingElement", () => {
    const App = () =>
      createElement("div", {
        children: () => [
          createElement("div", { children: () => 0 }),
          createElement("div", { children: () => 1 }),
        ],
      });

    const saplingElement = createElement(App);

    expect(saplingElement).toMatchInlineSnapshot(`
      Object {
        "staticContainer": true,
        "el": null,
        "children": Set {
          Object {
            "staticContainer": false,
            "el": HTMLDivElement {},
            "children": Set {
              Object {
                "staticContainer": true,
                "el": null,
                "children": Set {
                  Object {
                    "staticContainer": false,
                    "el": HTMLDivElement {},
                    "children": Set {
                      Object {
                        "staticContainer": false,
                        "el": Text {},
                        "children": Set {},
                      },
                    },
                  },
                  Object {
                    "staticContainer": false,
                    "el": HTMLDivElement {},
                    "children": Set {
                      Object {
                        "staticContainer": false,
                        "el": Text {},
                        "children": Set {},
                      },
                    },
                  },
                },
              },
            },
          },
        },
      }
    `);
  });
});
