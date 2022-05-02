mod external;

pub struct MemoryUsage {
    pub rss: u64,
    pub heap_total: u64,
    pub heap_used: u64,
    pub external: u64,
}

pub struct Metrics {
    pub ops_dispatched: u64,
    pub ops_dispatched_sync: u64,
    pub ops_dispatched_async: u64,
    pub ops_dispatched_async_unref: u64,
    pub ops_completed: u64,
    pub ops_completed_sync: u64,
    pub ops_completed_async: u64,
    pub ops_completed_async_unref: u64,
    pub bytes_sent_control: u64,
    pub bytes_sent_data: u64,
    pub bytes_received: u64,
  }

#[derive(PartialEq)]
pub enum Signal {
    SIGABRT = 0,
    SIGALRM = 1,
    SIGBUS = 2,
    SIGCHLD = 3,
    SIGCONT = 4,
    SIGEMT = 5,
    SIGFPE = 6,
    SIGHUP = 7,
    SIGILL = 8,
    SIGINFO = 9,
    SIGINT = 10,
    SIGIO = 11,
    SIGKILL = 12,
    SIGPIPE = 13,
    SIGPROF = 14,
    SIGPWR = 15,
    SIGQUIT = 16,
    SIGSEGV = 17,
    SIGSTKFLT = 18,
    SIGSTOP = 19,
    SIGSYS = 20,
    SIGTERM = 21,
    SIGTRAP = 22,
    SIGTSTP = 23,
    SIGTTIN = 24,
    SIGTTOU = 25,
    SIGURG = 26,
    SIGUSR1 = 27,
    SIGUSR2 = 28,
    SIGVTALRM = 29,
    SIGWINCH = 30,
    SIGXCPU = 31,
    SIGXFSZ = 32,
}

pub struct Deno {}

impl Deno {
    pub fn read_sync(rid: u32, buffer: &mut [u8]) -> usize {
        unsafe { external::deno_read_sync(rid, buffer.as_mut_ptr() as *mut u8, buffer.len()) }
    }

    pub fn write_sync(rid: u32, buffer: &[u8]) -> usize {
        unsafe { external::deno_write_sync(rid, buffer.as_ptr() as *const u8, buffer.len()) }
    }
    pub fn exit(code: Option<u32>) {
        let exit_code = match code {
            Some(v) => v.try_into().unwrap(),
            None => -1,
        };

        unsafe {
            external::deno_exit(exit_code);
        };
    }

    pub fn deno_kill(pid: u32, signal: Signal) {
        unsafe { external::deno_kill(pid, signal as u8) }
    }

    pub fn memory_usage() -> MemoryUsage {
        let values = unsafe {
            let ptr = external::deno_memory_usage();
            let data = std::ptr::read(ptr);
            external::__dealloc(ptr as *mut u8, data.len() * std::mem::size_of::<u32>());

            data
        };

        MemoryUsage {
            rss: values[0],
            heap_total: values[1],
            heap_used: values[2],
            external: values[3],
        }
    }

    pub fn deno_metrics() -> Metrics {
        let values = unsafe {
            let ptr = external::deno_metrics();
            let data = std::ptr::read(ptr);
            external::__dealloc(ptr as *mut u8, data.len() * std::mem::size_of::<u64>());
        
            data
        };

        Metrics {
            ops_dispatched: values[0],
            ops_dispatched_sync: values[1],
            ops_dispatched_async: values[2],
            ops_dispatched_async_unref: values[3],
            ops_completed: values[4],
            ops_completed_sync: values[5],
            ops_completed_async: values[6],
            ops_completed_async_unref: values[7],
            bytes_sent_control: values[8],
            bytes_sent_data: values[9],
            bytes_received: values[10]
        }
    }
}
