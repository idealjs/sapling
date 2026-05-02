import { describe, expect, it } from "vitest";
import { createProxy } from "./createProxy";
import type { PathKey } from "./types";

describe("createProxy - path tracing with state mutations", () => {
  it("should trace multi-field object paths and notify property mutations", () => {
    const target = {
      user: {
        profile: {
          name: "John",
          email: "john@example.com",
          age: 30,
        },
        preferences: {
          theme: "dark",
          notifications: true,
        },
      },
      settings: {
        locale: "en-US",
        timezone: "UTC",
      },
      metadata: {
        createdAt: "2024-01-01",
        version: 1,
      },
    };
    const operations: Array<{ paths: PathKey[]; operation: string }> = [];
    const notify = (paths: PathKey[], operation: string) => {
      operations.push({ paths, operation });
    };

    const proxy = createProxy(target, { notify });

    const profileProxy = proxy.user.profile;

    expect(profileProxy.name).toBe("John");
    expect(profileProxy.email).toBe("john@example.com");

    profileProxy.name = "Jane";
    profileProxy.email = "jane@example.com";

    expect(target.user.profile.name).toBe("Jane");
    expect(target.user.profile.email).toBe("jane@example.com");

    expect(operations).toEqual([
      {
        operation: "get",
        paths: ["user"],
      },
      {
        operation: "get",
        paths: ["user", "profile"],
      },
      {
        operation: "get",
        paths: ["user", "profile", "name"],
      },
      {
        operation: "get",
        paths: ["user", "profile", "email"],
      },
      { paths: ["user", "profile", "name"], operation: "set" },
      { paths: ["user", "profile", "email"], operation: "set" },
    ]);
  });

  it("should trace nested array structures and notify item mutations and deletions", () => {
    const target = {
      users: [
        {
          id: 1,
          name: "Alice",
          tags: ["admin", "user"],
          metadata: { active: true, role: "admin" },
        },
        {
          id: 2,
          name: "Bob",
          tags: ["user", "moderator"],
          metadata: { active: true, role: "moderator" },
        },
        {
          id: 3,
          name: "Charlie",
          tags: ["guest"],
          metadata: { active: false, role: "guest" },
        },
      ],
      metadata: {
        total: 3,
        lastUpdated: "2024-01-01",
      },
    };
    const operations: Array<{ paths: PathKey[]; operation: string }> = [];
    const notify = (paths: PathKey[], operation: string) => {
      operations.push({ paths, operation });
    };

    const proxy = createProxy(target, { notify });

    const firstUserProxy = proxy.users[0];
    const readOps = operations.filter((op) => op.operation === "get");
    expect(readOps.length).toBeGreaterThan(0);

    expect(firstUserProxy.name).toBe("Alice");
    expect(firstUserProxy.tags[0]).toBe("admin");
    expect(firstUserProxy.metadata.role).toBe("admin");

    operations.length = 0;

    firstUserProxy.name = "Alicia";
    delete firstUserProxy.tags[0];
    firstUserProxy.metadata.role = "super-admin";

    expect(target.users[0].name).toBe("Alicia");
    expect(0 in target.users[0].tags).toBe(false);
    expect(target.users[0].metadata.role).toBe("super-admin");

    expect(operations).toEqual(
      expect.arrayContaining([
        { paths: ["users", "0", "name"], operation: "set" },
        { paths: ["users", "0", "tags", "0"], operation: "delete" },
        { paths: ["users", "0", "metadata", "role"], operation: "set" },
      ]),
    );
  });

  it("should track array push operations with correct paths", () => {
    const target = {
      items: [
        { id: 1, name: "item1" },
        { id: 2, name: "item2" },
      ],
    };
    const operations: Array<{ paths: PathKey[]; operation: string }> = [];
    const notify = (paths: PathKey[], operation: string) => {
      operations.push({ paths, operation });
    };

    const proxy = createProxy(target, { notify });

    // Read items array
    const itemsProxy = proxy.items;
    const readOps = operations.filter((op) => op.operation === "get");
    expect(readOps.length).toBeGreaterThan(0);

    operations.length = 0;

    // Push a new item to the array
    itemsProxy.push({ id: 3, name: "item3" });

    expect(target.items.length).toBe(3);
    expect(target.items[2]).toEqual({ id: 3, name: "item3" });

    expect(operations).toEqual(
      expect.arrayContaining([
        { paths: ["items", "2"], operation: "set" },
        { paths: ["items", "length"], operation: "set" },
      ]),
    );
  });

  it("should track array splice insert operations with correct paths", () => {
    const target = {
      items: [
        { id: 1, name: "item1" },
        { id: 2, name: "item2" },
        { id: 3, name: "item3" },
      ],
    };
    const operations: Array<{ paths: PathKey[]; operation: string }> = [];
    const notify = (paths: PathKey[], operation: string) => {
      operations.push({ paths, operation });
    };

    const proxy = createProxy(target, { notify });

    const itemsProxy = proxy.items;

    itemsProxy.splice(1, 0, { id: 1.5, name: "item1.5" });

    expect(target.items).toEqual([
      { id: 1, name: "item1" },
      { id: 1.5, name: "item1.5" },
      { id: 2, name: "item2" },
      { id: 3, name: "item3" },
    ]);

    expect(operations).toMatchInlineSnapshot([
      {
        operation: "get",
        paths: ["items"],
      },
      {
        operation: "get",
        paths: ["items", "splice"],
      },
      {
        operation: "get",
        paths: ["items", "length"],
      },
      {
        operation: "get",
        paths: ["items", "constructor"],
      },
      {
        operation: "get",
        paths: ["items", "2"],
      },
      {
        operation: "set",
        paths: ["items", "3"],
      },
      {
        operation: "get",
        paths: ["items", "1"],
      },
      {
        operation: "set",
        paths: ["items", "2"],
      },
      {
        operation: "set",
        paths: ["items", "1"],
      },
      {
        operation: "set",
        paths: ["items", "length"],
      },
      {
        operation: "get",
        paths: ["items", "2", "symbol:Symbol.iterator"],
      },
      {
        operation: "get",
        paths: ["items", "2", "symbol:Symbol.toStringTag"],
      },
      {
        operation: "get",
        paths: ["items", "2", "id"],
      },
      {
        operation: "get",
        paths: ["items", "2", "name"],
      },
      {
        operation: "get",
        paths: ["items", "2", "name"],
      },
      {
        operation: "get",
        paths: ["items", "2", "id"],
      },
      {
        operation: "get",
        paths: ["items", "1", "symbol:Symbol.iterator"],
      },
      {
        operation: "get",
        paths: ["items", "1", "symbol:Symbol.toStringTag"],
      },
      {
        operation: "get",
        paths: ["items", "1", "id"],
      },
      {
        operation: "get",
        paths: ["items", "1", "name"],
      },
      {
        operation: "get",
        paths: ["items", "1", "name"],
      },
      {
        operation: "get",
        paths: ["items", "1", "id"],
      },
    ]);
  });
});
