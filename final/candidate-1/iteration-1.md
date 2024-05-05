# Iteration 1 Report

The goal of Iteration 1 is:

1. Finish `crate::filesystem` and `crate::sandbox`.

## `crate::sandbox`

Sandbox provide abstration over process lifetime, here is some notable public interface from `crate::sandbox`:

**interfaces**

```rust
pub trait Context: Limit {
    type FS: Filesystem;
    fn create_fs(&mut self) -> Self::FS;
}

pub trait Limit {
    fn get_cpu(&mut self) -> Cpu;
    fn get_memory(&mut self) -> Memory;
    fn get_args(&mut self) -> impl Iterator<Item = &OsStr>;
    fn get_output_limit(&mut self) -> u64;
    fn get_walltime(&mut self) -> Duration {
        Duration::from_secs(60 * 30)
    }
}

pub trait Filesystem {
    fn mount(&mut self) -> impl AsRef<Path> + Send;
    fn get_size(&mut self) -> u64;
}
```

**classes**

```rust
pub struct Process<C: Context>;
impl<C: Context> Process<C> {
    pub fn new(context: C) -> Result<Self, Error>;
    pub async fn wait(mut self, input: Vec<u8>) -> Result<Corpse, Error>;
}

pub struct Corpse {
    pub code: Option<ExitStatus>,
    pub reason: Option<MonitorKind>,
    pub stdout: Vec<u8>,
    pub stat: Stat,
}
impl Corpse {
    pub fn status(&self) -> Result<ExitStatus, MonitorKind>;
    pub fn stdout(&self) -> &[u8];
    pub fn stat(&self) -> &Stat;
}

pub struct Stat {
    pub memory: Memory,
    pub cpu: Cpu,
    pub output: Output,
    pub walltime: Duration,
}
```

See backlog for detail implementation
```
sandbox(module)
├── monitor(module)
└── process(module)
```

### TODO

Due to time limitation, there are some feature/refactor waiting for completion:
1. use `int syscall(SYS_pidfd_open, pid_t pid, unsigned int flags);` to optimistically kill process.
2. use `composite` along with `factory for different kind of `monitor`
3. don't use `DuplexStream` for better performance.(it was initially used to decouple `OuputMonitor`)

## `crate::filesystem`

Sandbox provide abstration over a filesystem in userspace implementation, here is some notable public interface from `crate::filesystem`:

```rust
pub struct Filesystem<F>;

impl<F> Filesystem<F>{
    pub fn cloned(&self) -> Self;
    pub async fn mount(
        self,
        resource: Semaphore,
        path: impl AsRef<Path> + 'static
    ) -> Result<(), Error>;
}
impl Filesystem<std::io::File>{
    pub fn new(file: File) -> Result<Self, Error>;
}
```

### TODO

Due to time limitation, there are some feature/refactor waiting for completion:
1. Verify correctness not only by unit test.
2. use B plus tree instead of a deep copy of tree(that is full of reference counting and mixed of asynchronous/spin lock).
3. Add ability to perform link/unlink
4. provide common character special file(`/dev/null`, `/dev/random`).

## Reference

### Test result from CI

```text
running 24 tests
test filesystem::entry::rw::test::normal_read ... ok
test filesystem::entry::rw::test::end_of_file_read ... ok
test filesystem::entry::ro::test::normal_read ... ok
test filesystem::entry::ro::test::end_of_file_read ... ok
test filesystem::entry::rw::test::end_seek ... ok
test filesystem::entry::rw::test::rel_seek ... ok
test filesystem::entry::rw::test::normal_write ... ok
test filesystem::entry::rw::test::start_seek ... ok
test filesystem::entry::ro::test::multi_sequential_read ... ok
test filesystem::tree::test::deep_clone ... ok
test filesystem::entry::rw::test::test_take_short_read ... ok
test sandbox::monitor::stat::test::cpu_from_raw ... ok
test filesystem::tree::test::insert_parent_not_found ... ok
test filesystem::entry::template::test::nested_map ... ok
test filesystem::tree::test::insert_is_root ... ok
test semaphore::test::get_permit_max ... ok
test sandbox::monitor::output::test::monitor_output_limit ... ok
test filesystem::tree::test::insert ... ok
test filesystem::entry::rw::test::test_take_read - should panic ... ok
test filesystem::entry::ro::test::multi_reader_read ... ok
test semaphore::test::get_permit_max_wait ... ok
test filesystem::entry::rw::test::multi_read ... ok
test semaphore::test::get_permit_unorder ... ok
test filesystem::tree::test::multi_lookup ... ok

test result: ok. 24 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.05s
```