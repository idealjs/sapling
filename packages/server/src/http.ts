import Fastify from "fastify";

const createServer = () => {
  const fastify = Fastify({
    logger: true,
  });

  return fastify;
};

export default createServer;
