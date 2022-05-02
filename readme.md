Why do I do this?

Welp use this however you like. Deno namespace wrapper for rust compiled to
wasm.

## Compatible api's

### Signal api

- [ ] addSignalListener
- [ ] removeSignalListener

### Filesystem operations

- [ ] chdir
- [ ] chmodSync
- [ ] chownSync
- [ ] copyFileSync
- [ ] createSync
- [ ] fdatasyncSync
- [ ] fstatSync
- [ ] fsyncSync
- [ ] ftruncateSync
- [ ] makeTempDirSync
- [ ] makeTempFileSync
- [ ] mkdirSync
- [ ] openSync
- [ ] readAllSync
- [ ] readDirSync
- [ ] readFileSync
- [ ] readLinkSync
- [ ] readTextFileSync
- [ ] realPathsSync
- [ ] removeSync
- [ ] renameSync
- [ ] statSync
- [ ] symlinkSync
- [ ] truncateSync
- [ ] writeFileSync
- [ ] writeTextFileSync

### Resource based operations

All of these operations require a RID

- [ ] close
- [ ] isatty
- [ ] linkSync
- [ ] lstatSync
- [x] readSync
- [ ] seekSync
- [ ] writeSync

### Process based operations

- [x] exit
- [x] kill
- [x] memoryUsage
- [x] metrics
- [ ] resources
