declare global {
  namespace NodeJS {
    interface ProcessEnv {
      HTTP2?: string;
    }
  }
}

export {};
