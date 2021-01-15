// Copyright 2018-2021 the Deno authors. All rights reserved. MIT license.

((window) => {
  const core = window.Deno.core;
  const { errors } = window.__bootstrap.errors;
  const { read, write } = window.__bootstrap.io;

  function opBindEndpoint(args) {
    return core.jsonOpSync("op_bind_endpoint", args);
  }

  function createQuicEndpoint( hostname, port ) {
    const res = opBindEndpoint({
      hostname: hostname,
      port: port,
    });

    return new QuicEndpoint(res.rid, res.hostname, res.port);
  }

  class QuicEndpoint {
    #rid = 0;
    #hostname = null;
    #port = null;

    constructor(rid, hostname, port) {
      this.#rid = rid;
      this.#hostname = hostname;
      this.#port = port;
    }

    get rid() {
      return this.#rid;
    }

    // hostname
    get hostname() {
      return this.#hostname;
    }

    get port() {
      return this.#port;
    }

    // listen() {}

    // connect() {}

    close() {
      core.close(this.rid);
    }
  }

  window.__bootstrap.quic = {
    QuicEndpoint,
    createQuicEndpoint,
    opBindEndpoint,
  };
})(this);
