import Fastify from "fastify";
import fs from "fs";
import path from "path";

const createServer = () => {
  const options = {
    key: fs.readFileSync(path.resolve(__dirname, "../server.key")),
    cert: fs.readFileSync(path.resolve(__dirname, "../server.crt")),
  };
  const fastify = Fastify({
    http2: true,
    https: options,
  });

  return fastify;
};

export default createServer;
