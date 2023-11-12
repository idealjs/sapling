import fs from "fs";
import chunk from "lodash.chunk";

export type HeapSnapshot = {
  snapshot: {
    meta: {
      node_fields: [
        "type",
        "name",
        "id",
        "self_size",
        "edge_count",
        "trace_node_id",
        "detachedness",
      ];
      node_types: [
        [
          "hidden",
          "array",
          "string",
          "object",
          "code",
          "closure",
          "regexp",
          "number",
          "native",
          "synthetic",
          "concatenated string",
          "sliced string",
          "symbol",
          "bigint",
        ],
        "string",
        "number",
        "number",
        "number",
        "number",
        "number",
      ];
      edge_fields: ["type", "name_or_index", "to_node"];
      edge_types: [
        [
          "context",
          "element",
          "property",
          "internal",
          "hidden",
          "shortcut",
          "weak",
        ],
        "string_or_number",
        "node",
      ];
      trace_function_info_fields: [
        "function_id",
        "name",
        "script_name",
        "script_id",
        "line",
        "column",
      ];
      trace_node_fields: [
        "id",
        "function_info_index",
        "count",
        "size",
        "children",
      ];
      sample_fields: ["timestamp_us", "last_assigned_id"];
      location_fields: ["object_index", "script_id", "line", "column"];
    };
    node_count: number;
    edge_count: number;
    trace_function_count: number;
  };
  nodes: number[];
  edges: number[];
  trace_function_infos: [];
  trace_tree: [];
  samples: [];
  locations: number[];
  strings: string[];
};

export type HeapNode = {
  type: string;
  name: string;
  id: number;
  selfSize: number;
  edgeCount: number;
  traceNodeId: number;
  detachedness: number;
};

export const readSnapshotFile = (filename: string) => {
  return JSON.parse(fs.readFileSync(filename).toString());
};

export const getNodes = (heapSnapshot: HeapSnapshot) => {
  const nodes = (
    chunk(heapSnapshot.nodes, 7) as [
      number,
      number,
      number,
      number,
      number,
      number,
      number,
    ][]
  ).map((node) => {
    const type = heapSnapshot.snapshot.meta.node_types[0][node[0]];
    const name = heapSnapshot.strings[node[1]];
    const id = node[2];
    const size = node[3];
    const edgeCount = node[4];
    const traceNodeId = node[5];
    const detachedness = node[6];
    return {
      type,
      name,
      id,
      size,
      edgeCount,
      traceNodeId,
      detachedness,
    };
  });
  return nodes;
};

export const getState = (nodes: ReturnType<typeof getNodes>) => {
  return nodes.filter(
    (node) =>
      (node.name === "Observable" || node.name === "Subscriber") &&
      node.type === "object",
  );
};
