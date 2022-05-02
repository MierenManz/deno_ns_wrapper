interface WasmInternalExports {
  __alloc(size: number): number;
  memory: WebAssembly.Memory;
}

const SIGNALS = [
  "SIGABRT",
  "SIGALRM",
  "SIGBUS",
  "SIGCHLD",
  "SIGCONT",
  "SIGEMT",
  "SIGFPE",
  "SIGHUP",
  "SIGILL",
  "SIGINFO",
  "SIGINT",
  "SIGIO",
  "SIGKILL",
  "SIGPIPE",
  "SIGPROF",
  "SIGPWR",
  "SIGQUIT",
  "SIGSEGV",
  "SIGSTKFLT",
  "SIGSTOP",
  "SIGSYS",
  "SIGTERM",
  "SIGTRAP",
  "SIGTSTP",
  "SIGTTIN",
  "SIGTTOU",
  "SIGURG",
  "SIGUSR1",
  "SIGUSR2",
  "SIGVTALRM",
  "SIGWINCH",
  "SIGXCPU",
  "SIGXFSZ",
];

export async function wasmInstanciateStreaming<T extends WebAssembly.Exports>(
  prom: Response | PromiseLike<Response>,
): Promise<T> {
  const mod = await WebAssembly.instantiateStreaming(prom, {
    env: {
      deno_read_sync(rid: number, ptr: number, length: number): number {
        const view = new Uint8Array(exports.memory.buffer, ptr, length);
        const read = Deno.readSync(rid, view);
        return read ?? -1;
      },
      deno_write_sync(rid: number, ptr: number, length: number): number {
        const view = new Uint8Array(exports.memory.buffer, ptr, length);
        return Deno.writeSync(rid, view);
      },

      deno_exit(code: number): never {
        Deno.exit(code !== -1 ? code : undefined);
      },
      deno_kill(pid: number, signal_id: number) {
        Deno.kill(pid, SIGNALS[signal_id] as Deno.Signal);
      },
      deno_memory_usage(): number {
        const ptr = exports.__alloc(8 * 4);
        const view = new BigUint64Array(exports.memory.buffer);

        const bigintValues = Object
          .values(Deno.memoryUsage())
          .map((x) => BigInt(x));
        view.set(bigintValues, ptr);

        return ptr;
      },
      deno_metrics(): number {
        const ptr = exports.__alloc(8 * 11);
        const view = new BigUint64Array(exports.memory.buffer);
        const metrics = Deno.metrics() as unknown as Record<string, number>;
        delete metrics.ops;

        const bigintValues = Object
          .values(metrics)
          .map((x) => BigInt(x));
        view.set(bigintValues, ptr);

        return ptr;
      },
    },
  });
  const exports = mod.instance.exports as unknown as WasmInternalExports;
  return mod.instance.exports as T;
}
