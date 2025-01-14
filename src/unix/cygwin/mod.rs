pub type c_char = i8;
pub type c_long = i64;
pub type c_ulong = u64;

cfg_if! {
    if #[cfg(doc)] {
        pub(crate) type Ioctl = ::c_int;
    } else {
        #[doc(hidden)]
        pub type Ioctl = ::c_int;
    }
}

// stddef.h
pub type wchar_t = ::c_ushort;

s_no_extra_traits! {
    #[allow(missing_debug_implementations)]
    #[repr(align(16))]
    pub struct max_align_t {
        priv_: [f64; 4]
    }
}

// machine/_types.h
pub type blkcnt_t = i64;
pub type blksize_t = i32;
pub type dev_t = u32;
pub type fsblkcnt_t = c_ulong;
pub type fsfilcnt_t = c_ulong;
pub type ino_t = u64;
pub type key_t = ::c_longlong;
pub type sa_family_t = u16;
pub type socklen_t = ::c_int;

// sys/_types.h
pub type off_t = c_long;
pub type id_t = u32;
pub type mode_t = u32;
pub type _off64_t = ::c_longlong;
pub type loff_t = _off64_t;
pub type iconv_t = *mut ::c_void;
pub type clock_t = c_ulong;
pub type time_t = c_long;
pub type clockid_t = c_ulong;
pub type timer_t = c_ulong;
pub type nl_item = ::c_int;
pub type nlink_t = ::c_ushort;
pub type suseconds_t = c_long;
pub type useconds_t = c_ulong;

// sys/_sigset.h
pub type sigset_t = c_ulong;

// sys/timespec.h
s! {
    pub struct itimerspec {
        pub it_interval: ::timespec,
        pub it_value: ::timespec,
    }
}

// sys/select.h
pub const FD_SETSIZE: usize = 1024;
pub type fd_mask = c_ulong;

// intentionally not public, only used for fd_set
cfg_if! {
    if #[cfg(target_pointer_width = "32")] {
        const ULONG_SIZE: usize = 32;
    } else if #[cfg(target_pointer_width = "64")] {
        const ULONG_SIZE: usize = 64;
    } else {
        // Unknown target_pointer_width
    }
}

s_no_extra_traits! {
    pub struct fd_set {
        fds_bits: [fd_mask; FD_SETSIZE / ULONG_SIZE],
    }
}

cfg_if! {
    if #[cfg(feature = "extra_traits")] {
        impl PartialEq for fd_set {
            fn eq(&self, other: &fd_set) -> bool {
                self.fds_bits
                    .iter()
                    .zip(other.fds_bits.iter())
                    .all(|(a,b)| a == b)
            }
        }

        impl Eq for fd_set {}

        impl ::fmt::Debug for fd_set {
            fn fmt(&self, f: &mut ::fmt::Formatter) -> ::fmt::Result {
                f.debug_struct("fd_set")
                    // FIXME: .field("fds_bits", &self.fds_bits)
                    .finish()
            }
        }

        impl ::hash::Hash for fd_set {
            fn hash<H: ::hash::Hasher>(&self, state: &mut H) {
                self.fds_bits.hash(state);
            }
        }
    }
}

f! {
    pub fn FD_CLR(fd: ::c_int, set: *mut fd_set) -> () {
        let fd = fd as usize;
        let size = ::mem::size_of_val(&(*set).fds_bits[0]) * 8;
        (*set).fds_bits[fd / size] &= !(1 << (fd % size));
        return
    }

    pub fn FD_ISSET(fd: ::c_int, set: *const fd_set) -> bool {
        let fd = fd as usize;
        let size = ::mem::size_of_val(&(*set).fds_bits[0]) * 8;
        return ((*set).fds_bits[fd / size] & (1 << (fd % size))) != 0
    }

    pub fn FD_SET(fd: ::c_int, set: *mut fd_set) -> () {
        let fd = fd as usize;
        let size = ::mem::size_of_val(&(*set).fds_bits[0]) * 8;
        (*set).fds_bits[fd / size] |= 1 << (fd % size);
        return
    }

    pub fn FD_ZERO(set: *mut fd_set) -> () {
        for slot in (*set).fds_bits.iter_mut() {
            *slot = 0;
        }
    }
}

// sys/cpuset.h
pub const CPU_SETSIZE: ::c_int = 0x400;

s! {
    pub struct cpu_set_t {
        bits: [u64; 16],
    }
}

f! {
    pub fn CPU_ALLOC_SIZE(count: ::c_int) -> ::size_t {
        let _dummy: cpu_set_t = ::mem::zeroed();
        let size_in_bits = 8 * ::mem::size_of_val(&_dummy.bits[0]);
        ((count as ::size_t + size_in_bits - 1) / 8) as ::size_t
    }

    pub fn CPU_COUNT_S(size: usize, cpuset: &cpu_set_t) -> ::c_int {
        let mut s: u32 = 0;
        let size_of_mask = ::mem::size_of_val(&cpuset.bits[0]);
        for i in cpuset.bits[..(size / size_of_mask)].iter() {
            s += i.count_ones();
        };
        s as ::c_int
    }

    pub fn CPU_ZERO(cpuset: &mut cpu_set_t) -> () {
        for slot in cpuset.bits.iter_mut() {
            *slot = 0;
        }
    }
    pub fn CPU_SET(cpu: usize, cpuset: &mut cpu_set_t) -> () {
        let size_in_bits
            = 8 * ::mem::size_of_val(&cpuset.bits[0]);
        if cpu < size_in_bits {
            let (idx, offset) = (cpu / size_in_bits, cpu % size_in_bits);
            cpuset.bits[idx] |= 1 << offset;
            ()
        }
    }

    pub fn CPU_CLR(cpu: usize, cpuset: &mut cpu_set_t) -> () {
        let size_in_bits
            = 8 * ::mem::size_of_val(&cpuset.bits[0]);
        if cpu < size_in_bits {
            let (idx, offset) = (cpu / size_in_bits, cpu % size_in_bits);
            cpuset.bits[idx] &= !(1 << offset);
            ()
        }
    }

    pub fn CPU_ISSET(cpu: usize, cpuset: &cpu_set_t) -> bool {
        let size_in_bits = 8 * ::mem::size_of_val(&cpuset.bits[0]);
        if cpu < size_in_bits {
            let (idx, offset) = (cpu / size_in_bits, cpu % size_in_bits);
            0 != (cpuset.bits[idx] & (1 << offset))
        } else {
            false
        }
    }

    pub fn CPU_COUNT(cpuset: &cpu_set_t) -> ::c_int {
        CPU_COUNT_S(::mem::size_of::<cpu_set_t>(), cpuset)
    }

    pub fn CPU_EQUAL(set1: &cpu_set_t, set2: &cpu_set_t) -> bool {
        set1.bits == set2.bits
    }
}

// sys/_pthreadtypes.h
pub type pthread_t = *mut ::c_void;
pub type pthread_mutex_t = *mut ::c_void;

// Must be usize due to libstd/sys_common/thread_local.rs,
// should technically be *mut ::c_void
pub type pthread_key_t = usize;

pub type pthread_attr_t = *mut ::c_void;
pub type pthread_mutexattr_t = *mut ::c_void;
pub type pthread_condattr_t = *mut ::c_void;
pub type pthread_cond_t = *mut ::c_void;

// The following ones should be *mut ::c_void
pub type pthread_barrierattr_t = usize;
pub type pthread_barrier_t = usize;
pub type pthread_spinlock_t = usize;

pub type pthread_rwlock_t = *mut ::c_void;
pub type pthread_rwlockattr_t = *mut ::c_void;

// sys/sysmacros.h
f! {
    pub fn major(dev: dev_t) -> ::c_uint {
        ((dev >> 16) & 0xffff) as ::c_uint
    }

    pub fn minor(dev: dev_t) -> ::c_uint {
        (dev & 0xffff) as ::c_uint
    }
}

safe_f! {
    pub {const} fn makedev(ma: ::c_uint, mi: ::c_uint) -> dev_t {
        let ma = ma as dev_t;
        let mi = mi as dev_t;
        (ma << 16) | (mi & 0xffff)
    }
}

// sys/types.h
pub type register_t = ::intptr_t;
pub type u_char = ::c_uchar;
pub type u_short = ::c_ushort;
pub type u_long = c_ulong;
pub type u_int = ::c_uint;
pub type caddr_t = *mut c_char;
pub type vm_size_t = c_ulong;

// cygwin/signal.h
s_no_extra_traits! {
    pub struct _uc_fpxreg {
        pub significand: [u16; 4],
        pub exponent: u16,
        pub padding: [u16; 3],
    }

    pub struct _uc_xmmreg {
        pub element: [u32; 4],
    }

    pub struct _fpstate {
        pub cwd: u16,
        pub swd: u16,
        pub ftw: u16,
        pub fop: u16,
        pub rip: u64,
        pub rdp: u64,
        pub mxcsr: u32,
        pub mxcr_mask: u32,
        pub st: [_uc_fpxreg; 8],
        pub xmm: [_uc_xmmreg; 16],
        pub padding: [u32; 24],
    }

    #[repr(align(16))]
    pub struct mcontext_t {
        pub p1home: u64,
        pub p2home: u64,
        pub p3home: u64,
        pub p4home: u64,
        pub p5home: u64,
        pub p6home: u64,
        pub ctxflags: u32,
        pub mxcsr: u32,
        pub cs: u16,
        pub ds: u16,
        pub es: u16,
        pub fs: u16,
        pub gs: u16,
        pub ss: u16,
        pub eflags: u32,
        pub dr0: u64,
        pub dr1: u64,
        pub dr2: u64,
        pub dr3: u64,
        pub dr6: u64,
        pub dr7: u64,
        pub rax: u64,
        pub rcx: u64,
        pub rdx: u64,
        pub rbx: u64,
        pub rsp: u64,
        pub rbp: u64,
        pub rsi: u64,
        pub rdi: u64,
        pub r8: u64,
        pub r9: u64,
        pub r10: u64,
        pub r11: u64,
        pub r12: u64,
        pub r13: u64,
        pub r14: u64,
        pub r15: u64,
        pub rip: u64,
        pub fpregs: _fpstate,
        pub vregs: [u64; 52],
        pub vcx: u64,
        pub dbc: u64,
        pub btr: u64,
        pub bfr: u64,
        pub etr: u64,
        pub efr: u64,
        pub oldmask: u64,
        pub cr2: u64,
    }
}

cfg_if! {
    if #[cfg(feature = "extra_traits")] {
        impl PartialEq for _uc_fpxreg {
            fn eq(&self, other: &_uc_fpxreg) -> bool {
                self.significand == other.significand &&
                self.exponent == other.exponent &&
                self.padding == other.padding
            }
        }

        impl Eq for _uc_fpxreg {}

        impl ::fmt::Debug for _uc_fpxreg {
            fn fmt(&self, f: &mut ::fmt::Formatter) -> ::fmt::Result {
                f.debug_struct("_uc_fpxreg")
                    .field("significand", &self.significand)
                    .field("exponent", &self.exponent)
                    .finish()
            }
        }

        impl ::hash::Hash for _uc_fpxreg {
            fn hash<H: ::hash::Hasher>(&self, state: &mut H) {
                self.significand.hash(state);
                self.exponent.hash(state);
                self.padding.hash(state);
            }
        }

        impl PartialEq for _uc_xmmreg {
            fn eq(&self, other: &_uc_xmmreg) -> bool {
                self.element == other.element
            }
        }

        impl Eq for _uc_xmmreg {}

        impl ::fmt::Debug for _uc_xmmreg {
            fn fmt(&self, f: &mut ::fmt::Formatter) -> ::fmt::Result {
                f.debug_struct("_uc_xmmreg")
                    .field("element", &self.element)
                    .finish()
            }
        }

        impl ::hash::Hash for _uc_xmmreg {
            fn hash<H: ::hash::Hasher>(&self, state: &mut H) {
                self.element.hash(state);
            }
        }

        impl PartialEq for _fpstate {
            fn eq(&self, other: &_fpstate) -> bool {
                self.cwd == other.cwd &&
                self.swd == other.swd &&
                self.ftw == other.ftw &&
                self.fop == other.fop &&
                self.rip == other.rip &&
                self.rdp == other.rdp &&
                self.mxcsr == other.mxcsr &&
                self.mxcr_mask == other.mxcr_mask &&
                self.st == other.st &&
                self.xmm == other.xmm &&
                self.padding == other.padding
            }
        }

        impl Eq for _fpstate {}

        impl ::fmt::Debug for _fpstate {
            fn fmt(&self, f: &mut ::fmt::Formatter) -> ::fmt::Result {
                f.debug_struct("_fpstate")
                    .field("cwd", &self.cwd)
                    .field("swd", &self.swd)
                    .field("ftw", &self.ftw)
                    .field("fop", &self.fop)
                    .field("rip", &self.rip)
                    .field("rdp", &self.rdp)
                    .field("mxcsr", &self.mxcsr)
                    .field("mxcr_mask", &self.mxcr_mask)
                    .field("st", &self.st)
                    .field("xmm", &self.xmm)
                    .finish()
            }
        }

        impl ::hash::Hash for _fpstate {
            fn hash<H: ::hash::Hasher>(&self, state: &mut H) {
                self.cwd.hash(state);
                self.swd.hash(state);
                self.ftw.hash(state);
                self.fop.hash(state);
                self.rip.hash(state);
                self.rdp.hash(state);
                self.mxcsr.hash(state);
                self.mxcr_mask.hash(state);
                self.st.hash(state);
                self.xmm.hash(state);
                self.padding.hash(state);
            }
        }

        impl PartialEq for mcontext_t {
            fn eq(&self, other: &mcontext_t) -> bool {
                self.p1home == other.p1home &&
                self.p2home == other.p2home &&
                self.p3home == other.p3home &&
                self.p4home == other.p4home &&
                self.p5home == other.p5home &&
                self.p6home == other.p6home &&
                self.ctxflags == other.ctxflags &&
                self.mxcsr == other.mxcsr &&
                self.cs == other.cs &&
                self.ds == other.ds &&
                self.es == other.es &&
                self.fs == other.fs &&
                self.gs == other.gs &&
                self.ss == other.ss &&
                self.eflags == other.eflags &&
                self.dr0 == other.dr0 &&
                self.dr1 == other.dr1 &&
                self.dr2 == other.dr2 &&
                self.dr3 == other.dr3 &&
                self.dr6 == other.dr6 &&
                self.dr7 == other.dr7 &&
                self.rax == other.rax &&
                self.rcx == other.rcx &&
                self.rdx == other.rdx &&
                self.rbx == other.rbx &&
                self.rsp == other.rsp &&
                self.rbp == other.rbp &&
                self.rsi == other.rsi &&
                self.rdi == other.rdi &&
                self.r8 == other.r8 &&
                self.r9 == other.r9 &&
                self.r10 == other.r10 &&
                self.r11 == other.r11 &&
                self.r12 == other.r12 &&
                self.r13 == other.r13 &&
                self.r14 == other.r14 &&
                self.r15 == other.r15 &&
                self.fpregs == other.fpregs &&
                self.vregs.iter().zip(other.vregs.iter())
                .all(|(a, b)| a == b) &&
                self.vcx == other.vcx &&
                self.dbc == other.dbc &&
                self.btr == other.btr &&
                self.bfr == other.bfr &&
                self.etr == other.etr &&
                self.efr == other.efr &&
                self.oldmask == other.oldmask &&
                self.cr2 == other.cr2
            }
        }

        impl Eq for mcontext_t {}

        impl ::fmt::Debug for mcontext_t {
            fn fmt(&self, f: &mut ::fmt::Formatter) -> ::fmt::Result {
                f.debug_struct("mcontext_t")
                    .field("p1home", &self.p1home)
                    .field("p2home", &self.p2home)
                    .field("p3home", &self.p3home)
                    .field("p4home", &self.p4home)
                    .field("p5home", &self.p5home)
                    .field("p6home", &self.p6home)
                    .field("ctxflags", &self.ctxflags)
                    .field("mxcsr", &self.mxcsr)
                    .field("cs", &self.cs)
                    .field("ds", &self.ds)
                    .field("es", &self.es)
                    .field("fs", &self.fs)
                    .field("gs", &self.gs)
                    .field("ss", &self.ss)
                    .field("eflags", &self.eflags)
                    .field("dr0", &self.dr0)
                    .field("dr1", &self.dr1)
                    .field("dr2", &self.dr2)
                    .field("dr3", &self.dr3)
                    .field("dr6", &self.dr6)
                    .field("dr7", &self.dr7)
                    .field("rax", &self.rax)
                    .field("rcx", &self.rcx)
                    .field("rdx", &self.rdx)
                    .field("rbx", &self.rbx)
                    .field("rsp", &self.rsp)
                    .field("rbp", &self.rbp)
                    .field("rsi", &self.rsi)
                    .field("rdi", &self.rdi)
                    .field("r8", &self.r8)
                    .field("r9", &self.r9)
                    .field("r10", &self.r10)
                    .field("r11", &self.r11)
                    .field("r12", &self.r12)
                    .field("r13", &self.r13)
                    .field("r14", &self.r14)
                    .field("r15", &self.r15)
                    .field("fpregs", &self.fpregs)
                    // FIXME: .field("vregs", &self.vregs)
                    .field("vcx", &self.vcx)
                    .field("dbc", &self.dbc)
                    .field("btr", &self.btr)
                    .field("bfr", &self.bfr)
                    .field("etr", &self.etr)
                    .field("efr", &self.efr)
                    .field("oldmask", &self.oldmask)
                    .field("cr2", &self.cr2)
                    .finish()
            }
        }

        impl ::hash::Hash for mcontext_t {
            fn hash<H: ::hash::Hasher>(&self, state: &mut H) {
                self.p1home.hash(state);
                self.p2home.hash(state);
                self.p3home.hash(state);
                self.p4home.hash(state);
                self.p5home.hash(state);
                self.p6home.hash(state);
                self.ctxflags.hash(state);
                self.mxcsr.hash(state);
                self.cs.hash(state);
                self.ds.hash(state);
                self.es.hash(state);
                self.fs.hash(state);
                self.gs.hash(state);
                self.ss.hash(state);
                self.eflags.hash(state);
                self.dr0.hash(state);
                self.dr1.hash(state);
                self.dr2.hash(state);
                self.dr3.hash(state);
                self.dr6.hash(state);
                self.dr7.hash(state);
                self.rax.hash(state);
                self.rcx.hash(state);
                self.rdx.hash(state);
                self.rbx.hash(state);
                self.rsp.hash(state);
                self.rbp.hash(state);
                self.rsi.hash(state);
                self.rdi.hash(state);
                self.r8.hash(state);
                self.r9.hash(state);
                self.r10.hash(state);
                self.r11.hash(state);
                self.r12.hash(state);
                self.r13.hash(state);
                self.r14.hash(state);
                self.r15.hash(state);
                self.rip.hash(state);
                self.fpregs.hash(state);
                self.vregs.hash(state);
                self.vcx.hash(state);
                self.dbc.hash(state);
                self.btr.hash(state);
                self.bfr.hash(state);
                self.etr.hash(state);
                self.efr.hash(state);
                self.oldmask.hash(state);
                self.cr2.hash(state);
            }
        }
    }
}

s_no_extra_traits! {
    pub struct sigevent {
        pub sigev_value: ::sigval,
        pub sigev_signo: ::c_int,
        pub sigev_notify: ::c_int,
        __unused1: *mut ::c_void,       //actually a function pointer
        pub sigev_notify_attributes: *mut pthread_attr_t
    }
}

cfg_if! {
    if #[cfg(feature = "extra_traits")] {
        impl PartialEq for sigevent {
            fn eq(&self, other: &sigevent) -> bool {
                self.sigev_value == other.sigev_value
                    && self.sigev_signo == other.sigev_signo
                    && self.sigev_notify == other.sigev_notify
                    && self.sigev_notify_attributes
                        == other.sigev_notify_attributes
            }
        }

        impl Eq for sigevent {}

        impl ::fmt::Debug for sigevent {
            fn fmt(&self, f: &mut ::fmt::Formatter) -> ::fmt::Result {
                f.debug_struct("sigevent")
                    .field("sigev_value", &self.sigev_value)
                    .field("sigev_signo", &self.sigev_signo)
                    .field("sigev_notify", &self.sigev_notify)
                    .field("sigev_notify_attributes",
                           &self.sigev_notify_attributes)
                    .finish()
            }
        }

        impl ::hash::Hash for sigevent {
            fn hash<H: ::hash::Hasher>(&self, state: &mut H) {
                self.sigev_value.hash(state);
                self.sigev_signo.hash(state);
                self.sigev_notify.hash(state);
                self.sigev_notify_attributes.hash(state);
            }
        }
    }
}

s_no_extra_traits! {
    pub struct siginfo_t {
        pub si_signo: ::c_int,
        pub si_code: ::c_int,
        pub si_pid: ::pid_t,
        pub si_uid: ::uid_t,
        pub si_errno: ::c_int,
        #[doc(hidden)]
        #[deprecated(
            since="0.2.54",
            note="Please leave a comment on \
                  https://github.com/rust-lang/libc/pull/1316 if you're using \
                  this field"
        )]
        pub __pad: [u32; 32],
    }
}

impl siginfo_t {
    pub unsafe fn si_addr(&self) -> *mut ::c_void {
        #[repr(C)]
        struct siginfo_si_addr {
            _si_signo: ::c_int,
            _si_code: ::c_int,
            _si_pid: ::pid_t,
            _si_uid: ::uid_t,
            _si_errno: ::c_int,
            si_addr: *mut ::c_void,
        }
        (*(self as *const siginfo_t as *const siginfo_si_addr)).si_addr
    }

    pub unsafe fn si_status(&self) -> ::c_int {
        #[repr(C)]
        struct siginfo_sigchld {
            _si_signo: ::c_int,
            _si_code: ::c_int,
            _si_pid: ::pid_t,
            _si_uid: ::uid_t,
            _si_errno: ::c_int,
            si_status: ::c_int,
        }
        (*(self as *const siginfo_t as *const siginfo_sigchld)).si_status
    }

    pub unsafe fn si_pid(&self) -> ::pid_t {
        self.si_pid
    }

    pub unsafe fn si_uid(&self) -> ::uid_t {
        self.si_uid
    }

    pub unsafe fn si_value(&self) -> ::sigval {
        #[repr(C)]
        struct siginfo_si_value {
            _si_signo: ::c_int,
            _si_code: ::c_int,
            _si_pid: ::pid_t,
            _si_uid: ::uid_t,
            _si_errno: ::c_int,
            si_value: ::sigval,
        }
        (*(self as *const siginfo_t as *const siginfo_si_value)).si_value
    }
}

cfg_if! {
    if #[cfg(feature = "extra_traits")] {
        impl PartialEq for siginfo_t {
            fn eq(&self, other: &siginfo_t) -> bool {
                self.si_signo == other.si_signo
                    && self.si_code == other.si_code
                    && self.si_pid == other.si_pid
                    && self.si_uid == other.si_uid
                    && self.si_errno == other.si_errno
                    // Ignore __pad
            }
        }

        impl Eq for siginfo_t {}

        impl ::fmt::Debug for siginfo_t {
            fn fmt(&self, f: &mut ::fmt::Formatter) -> ::fmt::Result {
                f.debug_struct("siginfo_t")
                    .field("si_signo", &self.si_signo)
                    .field("si_code", &self.si_code)
                    .field("si_pid", &self.si_pid)
                    .field("si_uid", &self.si_uid)
                    .field("si_errno", &self.si_errno)
                    // Ignore __pad
                    .finish()
            }
        }

        impl ::hash::Hash for siginfo_t {
            fn hash<H: ::hash::Hasher>(&self, state: &mut H) {
                self.si_signo.hash(state);
                self.si_code.hash(state);
                self.si_pid.hash(state);
                self.si_uid.hash(state);
                self.si_errno.hash(state);
                // Ignore __pad
            }
        }
    }
}

// si_code values for SIGBUS signal
pub const BUS_ADRALN: ::c_int = 25;
pub const BUS_ADRERR: ::c_int = 26;
pub const BUS_OBJERR: ::c_int = 27;

// si_code values for SIGCHLD signal
pub const CLD_EXITED: ::c_int = 28;
pub const CLD_KILLED: ::c_int = 29;
pub const CLD_DUMPED: ::c_int = 30;
pub const CLD_TRAPPED: ::c_int = 31;
pub const CLD_STOPPED: ::c_int = 32;
pub const CLD_CONTINUED: ::c_int = 33;

pub const SIGEV_SIGNAL: ::c_int = 0;
pub const SIGEV_NONE: ::c_int = 1;
pub const SIGEV_THREAD: ::c_int = 2;

s! {
    pub struct sigaction {
        pub sa_sigaction: ::sighandler_t,
        pub sa_mask: sigset_t,
        pub sa_flags: ::c_int,
    }
}

pub const SA_NOCLDSTOP: ::c_int = 0x00000001;
pub const SA_NOCLDWAIT: ::c_int = 0; // FIXME: does not exist on Cygwin!
pub const SA_SIGINFO: ::c_int = 0x00000002;
pub const SA_RESTART: ::c_int = 0x10000000;
pub const SA_ONSTACK: ::c_int = 0x20000000;
pub const SA_NODEFER: ::c_int = 0x40000000;
pub const SA_RESETHAND: ::c_int = 0x80000000;
pub const MINSIGSTKSZ: ::size_t = 8192;
pub const SIGSTKSZ: ::size_t = 32768;
pub const SIGHUP: ::c_int = 1;
pub const SIGINT: ::c_int = 2;
pub const SIGQUIT: ::c_int = 3;
pub const SIGILL: ::c_int = 4;
pub const SIGTRAP: ::c_int = 5;
pub const SIGABRT: ::c_int = 6;
pub const SIGEMT: ::c_int = 7;
pub const SIGFPE: ::c_int = 8;
pub const SIGKILL: ::c_int = 9;
pub const SIGBUS: ::c_int = 10;
pub const SIGSEGV: ::c_int = 11;
pub const SIGSYS: ::c_int = 12;
pub const SIGPIPE: ::c_int = 13;
pub const SIGALRM: ::c_int = 14;
pub const SIGTERM: ::c_int = 15;
pub const SIGURG: ::c_int = 16;
pub const SIGSTOP: ::c_int = 17;
pub const SIGTSTP: ::c_int = 18;
pub const SIGCONT: ::c_int = 19;
pub const SIGCHLD: ::c_int = 20;
pub const SIGTTIN: ::c_int = 21;
pub const SIGTTOU: ::c_int = 22;
pub const SIGIO: ::c_int = 23;
pub const SIGPOLL: ::c_int = 23;
pub const SIGXCPU: ::c_int = 24;
pub const SIGXFSZ: ::c_int = 25;
pub const SIGVTALRM: ::c_int = 26;
pub const SIGPROF: ::c_int = 27;
pub const SIGWINCH: ::c_int = 28;
pub const SIGPWR: ::c_int = 29;
pub const SIGUSR1: ::c_int = 30;
pub const SIGUSR2: ::c_int = 31;

extern "C" {
    pub fn sigwait(set: *const sigset_t, sig: *mut ::c_int) -> ::c_int;
    pub fn sigwaitinfo(set: *const sigset_t, info: *mut siginfo_t) -> ::c_int;
}

// sys/ucontext.h
s_no_extra_traits! {
    #[repr(align(8))]
    pub struct ucontext_t {
        pub uc_mcontext: mcontext_t,
        pub uc_link: *mut ucontext_t,
        pub uc_sigmask: sigset_t,
        pub uc_stack: stack_t,
        pub uc_flags: c_ulong,
    }
}

cfg_if! {
    if #[cfg(feature = "extra_traits")] {
        impl PartialEq for ucontext_t {
            fn eq(&self, other: &ucontext_t) -> bool {
                self.uc_mcontext == other.uc_mcontext
                    && self.uc_link == other.uc_link
                    && self.uc_sigmask == other.uc_sigmask
                    && self.uc_stack == other.uc_stack
                    && self.uc_flags == other.uc_flags
            }
        }

        impl Eq for ucontext_t {}

        impl ::fmt::Debug for ucontext_t {
            fn fmt(&self, f: &mut ::fmt::Formatter) -> ::fmt::Result {
                f.debug_struct("ucontext_t")
                    .field("uc_mcontext", &self.uc_mcontext)
                    .field("uc_link", &self.uc_link)
                    .field("uc_sigmask", &self.uc_sigmask)
                    .field("uc_stack", &self.uc_stack)
                    .field("uc_flags", &self.uc_flags)
                    .finish()
            }
        }

        impl ::hash::Hash for ucontext_t {
            fn hash<H: ::hash::Hasher>(&self, state: &mut H) {
                self.uc_mcontext.hash(state);
                self.uc_link.hash(state);
                self.uc_sigmask.hash(state);
                self.uc_stack.hash(state);
                self.uc_flags.hash(state);
            }
        }
    }
}

// sys/signal.h
pub const SS_ONSTACK: ::c_int = 0x1;
pub const SS_DISABLE: ::c_int = 0x2;

s! {
    pub struct stack_t {
        pub ss_sp: *mut ::c_void,
        pub ss_flags: ::c_int,
        pub ss_size: ::size_t,
    }
}

pub const SIG_SETMASK: ::c_int = 0;
pub const SIG_BLOCK: ::c_int = 1;
pub const SIG_UNBLOCK: ::c_int = 2;

extern "C" {
    pub fn pthread_sigmask(how: ::c_int, set: *const sigset_t, oldset: *mut sigset_t) -> ::c_int;
    pub fn sigsuspend(mask: *const sigset_t) -> ::c_int;
    pub fn sigaltstack(ss: *const stack_t, oss: *mut stack_t) -> ::c_int;
    pub fn pthread_kill(thread: pthread_t, sig: ::c_int) -> ::c_int;

    pub fn sigtimedwait(
        set: *const sigset_t,
        info: *mut siginfo_t,
        timeout: *const ::timespec,
    ) -> ::c_int;
}

// time.h
s! {
    pub struct tm {
        pub tm_sec: ::c_int,
        pub tm_min: ::c_int,
        pub tm_hour: ::c_int,
        pub tm_mday: ::c_int,
        pub tm_mon: ::c_int,
        pub tm_year: ::c_int,
        pub tm_wday: ::c_int,
        pub tm_yday: ::c_int,
        pub tm_isdst: ::c_int,
        pub tm_gmtoff: c_long,
        pub tm_zone: *const c_char,
    }
}

extern "C" {
    pub fn strftime(
        s: *mut c_char,
        max: ::size_t,
        format: *const c_char,
        tm: *const tm,
    ) -> ::size_t;

    pub fn asctime_r(tm: *const tm, buf: *mut c_char) -> *mut c_char;
    pub fn ctime_r(timep: *const time_t, buf: *mut c_char) -> *mut c_char;
    pub fn strptime(s: *const c_char, format: *const c_char, tm: *mut tm) -> *mut c_char;
    pub fn clock_settime(clk_id: clockid_t, tp: *const ::timespec) -> ::c_int;
    pub fn clock_gettime(clk_id: clockid_t, tp: *mut ::timespec) -> ::c_int;
    pub fn clock_getres(clk_id: clockid_t, tp: *mut ::timespec) -> ::c_int;

    pub fn timer_create(
        clockid: clockid_t,
        sevp: *mut sigevent,
        timerid: *mut timer_t,
    ) -> ::c_int;

    pub fn timer_delete(timerid: timer_t) -> ::c_int;

    pub fn timer_settime(
        timerid: timer_t,
        flags: ::c_int,
        new_value: *const itimerspec,
        old_value: *mut itimerspec,
    ) -> ::c_int;

    pub fn timer_gettime(timerid: timer_t, curr_value: *mut itimerspec) -> ::c_int;
    pub fn timer_getoverrun(timerid: timer_t) -> ::c_int;

    pub fn clock_nanosleep(
        clk_id: clockid_t,
        flags: ::c_int,
        rqtp: *const ::timespec,
        rmtp: *mut ::timespec,
    ) -> ::c_int;
}

pub const TIMER_ABSTIME: ::c_int = 4;
pub const CLOCK_REALTIME_COARSE: clockid_t = 0;
pub const CLOCK_REALTIME: clockid_t = 1;
pub const CLOCK_PROCESS_CPUTIME_ID: clockid_t = 2;
pub const CLOCK_THREAD_CPUTIME_ID: clockid_t = 3;
pub const CLOCK_MONOTONIC: clockid_t = 4;
pub const CLOCK_MONOTONIC_RAW: clockid_t = 5;
pub const CLOCK_MONOTONIC_COARSE: clockid_t = 6;
pub const CLOCK_BOOTTIME: clockid_t = 7;
pub const CLOCK_REALTIME_ALARM: clockid_t = 8;
pub const CLOCK_BOOTTIME_ALARM: clockid_t = 9;

extern "C" {
    pub fn clock_getcpuclockid(pid: ::pid_t, clk_id: *mut clockid_t) -> ::c_int;
}

// sys/time.h
#[cfg_attr(feature = "extra_traits", derive(Debug))]
pub enum timezone {}
impl ::Copy for timezone {}
impl ::Clone for timezone {
    fn clone(&self) -> timezone {
        *self
    }
}

s! {
    pub struct bintime {
        pub sec: time_t,
        pub frac: u64,
    }
}

pub const ITIMER_REAL: ::c_int = 0;
pub const ITIMER_VIRTUAL: ::c_int = 1;
pub const ITIMER_PROF: ::c_int = 2;

extern "C" {
    pub fn futimes(fd: ::c_int, times: *const ::timeval) -> ::c_int;
    pub fn lutimes(file: *const c_char, times: *const ::timeval) -> ::c_int;
    pub fn settimeofday(tv: *const ::timeval, tz: *const timezone) -> ::c_int;
    pub fn getitimer(which: ::c_int, curr_value: *mut ::itimerval) -> ::c_int;

    pub fn setitimer(
        which: ::c_int,
        new_value: *const ::itimerval,
        old_value: *mut ::itimerval,
    ) -> ::c_int;

    pub fn gettimeofday(tp: *mut ::timeval, tz: *mut ::c_void) -> ::c_int;
    pub fn futimesat(fd: ::c_int, path: *const c_char, times: *const ::timeval) -> ::c_int;
}

// sys/resource.h
pub const PRIO_PROCESS: ::c_int = 0;
pub const PRIO_PGRP: ::c_int = 1;
pub const PRIO_USER: ::c_int = 2;
pub const RLIMIT_CPU: ::c_int = 0;
pub const RLIMIT_FSIZE: ::c_int = 1;
pub const RLIMIT_DATA: ::c_int = 2;
pub const RLIMIT_STACK: ::c_int = 3;
pub const RLIMIT_CORE: ::c_int = 4;
pub const RLIMIT_NOFILE: ::c_int = 5;
pub const RLIMIT_AS: ::c_int = 6;
pub const RLIM_NLIMITS: ::c_int = 7;
pub const RLIMIT_NLIMITS: ::c_int = RLIM_NLIMITS;
pub const RLIM_INFINITY: rlim_t = !0;
pub const RLIM_SAVED_MAX: rlim_t = RLIM_INFINITY;
pub const RLIM_SAVED_CUR: rlim_t = RLIM_INFINITY;

pub type rlim_t = c_ulong;

pub const RUSAGE_SELF: ::c_int = 0;
pub const RUSAGE_CHILDREN: ::c_int = -1;

extern "C" {
    pub fn getrlimit(resource: ::c_int, rlim: *mut ::rlimit) -> ::c_int;
    pub fn setrlimit(resource: ::c_int, rlim: *const ::rlimit) -> ::c_int;
    pub fn getpriority(which: ::c_int, who: id_t) -> ::c_int;
    pub fn setpriority(which: ::c_int, who: id_t, prio: ::c_int) -> ::c_int;
}

// pwd.h
s! {
    pub struct passwd {
        pub pw_name: *mut c_char,
        pub pw_passwd: *mut c_char,
        pub pw_uid: ::uid_t,
        pub pw_gid: ::gid_t,
        pub pw_comment: *mut c_char,
        pub pw_gecos: *mut c_char,
        pub pw_dir: *mut c_char,
        pub pw_shell: *mut c_char
    }
}

extern "C" {
    pub fn getpwnam_r(
        name: *const c_char,
        pwd: *mut passwd,
        buf: *mut c_char,
        buflen: ::size_t,
        result: *mut *mut passwd,
    ) -> ::c_int;

    pub fn getpwuid_r(
        uid: ::uid_t,
        pwd: *mut passwd,
        buf: *mut c_char,
        buflen: ::size_t,
        result: *mut *mut passwd,
    ) -> ::c_int;

    pub fn getpwent() -> *mut passwd;
    pub fn setpwent();
    pub fn endpwent();
}

// cygwin/if.h
pub const IFF_UP: ::c_int = 0x1; // interface is up
pub const IFF_BROADCAST: ::c_int = 0x2; // broadcast address valid
pub const IFF_LOOPBACK: ::c_int = 0x8; // is a loopback net
pub const IFF_POINTOPOINT: ::c_int = 0x10; // interface is point-to-point link
pub const IFF_NOTRAILERS: ::c_int = 0x20; // avoid use of trailers
pub const IFF_RUNNING: ::c_int = 0x40; // resources allocated
pub const IFF_NOARP: ::c_int = 0x80; // no address resolution protocol
pub const IFF_PROMISC: ::c_int = 0x100; // receive all packets
pub const IFF_MULTICAST: ::c_int = 0x1000; // supports multicast
pub const IFF_LOWER_UP: ::c_int = 0x10000; // driver signals L1 up
pub const IFF_DORMANT: ::c_int = 0x20000; // driver signals dormant

s! {
    pub struct if_nameindex {
        pub if_index: ::c_uint,
        pub if_name: *mut c_char,
    }
}

pub const IF_NAMESIZE: ::size_t = 44;
pub const IFNAMSIZ: ::size_t = IF_NAMESIZE;

s_no_extra_traits! {
    #[cfg(libc_union)]
    pub union __c_anonymous_ifr_ifru {
        pub ifru_addr: sockaddr,
        pub ifru_broadaddr: sockaddr,
        pub ifru_dstaddr: sockaddr,
        pub ifru_netmask: sockaddr,
        pub ifru_hwaddr: sockaddr,
        pub ifru_flags: ::c_int,
        pub ifru_metric: ::c_int,
        pub ifru_mtu: ::c_int,
        pub ifru_ifindex: ::c_int,
        pub ifru_data: *mut c_char,
        __ifru_pad: [c_char; 28],
    }
}

cfg_if! {
    if #[cfg(feature = "extra_traits")] {
        #[cfg(libc_union)]
        impl ::fmt::Debug for __c_anonymous_ifr_ifru {
            fn fmt(&self, f: &mut ::fmt::Formatter) -> ::fmt::Result {
                f.debug_struct("ifr_ifru")
                    .field("ifru_addr", unsafe { &self.ifru_addr })
                    .field("ifru_broadaddr", unsafe { &self.ifru_broadaddr })
                    .field("ifru_dstaddr", unsafe { &self.ifru_dstaddr })
                    .field("ifru_netmask", unsafe { &self.ifru_netmask })
                    .field("ifru_hwaddr", unsafe { &self.ifru_hwaddr })
                    .field("ifru_flags", unsafe { &self.ifru_flags })
                    .field("ifru_metric", unsafe { &self.ifru_metric })
                    .field("ifru_ifindex", unsafe { &self.ifru_ifindex })
                    .field("ifru_mtu", unsafe { &self.ifru_mtu })
                    .field("ifru_data", unsafe { &self.ifru_data })
                    .finish()
            }
        }

        #[cfg(libc_union)]
        impl PartialEq for __c_anonymous_ifr_ifru {
            fn eq(&self, other: &__c_anonymous_ifr_ifru) -> bool {
                unsafe {
                    self.ifru_addr == other.ifru_addr &&
                    self.ifru_broadaddr == other.ifru_broadaddr &&
                    self.ifru_dstaddr == other.ifru_dstaddr &&
                    self.ifru_netmask == other.ifru_netmask &&
                    self.ifru_hwaddr == other.ifru_hwaddr &&
                    self.ifru_flags == other.ifru_flags &&
                    self.ifru_metric == other.ifru_metric &&
                    self.ifru_ifindex == other.ifru_ifindex &&
                    self.ifru_mtu == other.ifru_mtu &&
                    self.ifru_data == other.ifru_data
                }
            }
        }

        #[cfg(libc_union)]
        impl Eq for __c_anonymous_ifr_ifru {}

        #[cfg(libc_union)]
        impl ::hash::Hash for __c_anonymous_ifr_ifru {
            fn hash<H: ::hash::Hasher>(&self, state: &mut H) {
                unsafe { self.ifru_addr.hash(state) };
                unsafe { self.ifru_broadaddr.hash(state) };
                unsafe { self.ifru_dstaddr.hash(state) };
                unsafe { self.ifru_netmask.hash(state) };
                unsafe { self.ifru_hwaddr.hash(state) };
                unsafe { self.ifru_flags.hash(state) };
                unsafe { self.ifru_metric.hash(state) };
                unsafe { self.ifru_ifindex.hash(state) };
                unsafe { self.ifru_mtu.hash(state) };
                unsafe { self.ifru_data.hash(state) };
            }
        }
    }
}

s_no_extra_traits! {
    pub struct ifreq {
        /// if name, e.g. "en0"
        pub ifr_name: [c_char; IFNAMSIZ],
        #[cfg(libc_union)]
        pub ifr_ifru: __c_anonymous_ifr_ifru,
        #[cfg(not(libc_union))]
        __pad1: [c_char; 4],
        #[cfg(not(libc_union))]
        pub ifr_ifru: sockaddr,
        #[cfg(not(libc_union))]
        __pad2: [c_char; 16],
    }
}

cfg_if! {
    if #[cfg(feature = "extra_traits")] {
        impl ::fmt::Debug for ifreq {
            fn fmt(&self, f: &mut ::fmt::Formatter) -> ::fmt::Result {
                f.debug_struct("ifreq")
                    .field("ifr_name", &self.ifr_name)
                    .field("ifr_ifru", &self.ifr_ifru)
                    .finish()
            }
        }

        impl PartialEq for ifreq {
            fn eq(&self, other: &ifreq) -> bool {
                self.ifr_name == other.ifr_name && self.ifr_ifru == other.ifr_ifru
            }
        }

        impl Eq for ifreq {}

        impl ::hash::Hash for ifreq {
            fn hash<H: ::hash::Hasher>(&self, state: &mut H) {
                self.ifr_name.hash(state);
                self.ifr_ifru.hash(state);
            }
        }
    }
}

s_no_extra_traits! {
    #[cfg(libc_union)]
    pub union __c_anonymous_ifc_ifcu {
        pub ifcu_buf: caddr_t,
        pub ifcu_req: *mut ifreq,
    }
}

cfg_if! {
    if #[cfg(feature = "extra_traits")] {
        #[cfg(libc_union)]
        impl Eq for __c_anonymous_ifc_ifcu {}

        #[cfg(libc_union)]
        impl PartialEq for __c_anonymous_ifc_ifcu {
            fn eq(&self, other: &__c_anonymous_ifc_ifcu) -> bool {
                unsafe {
                    self.ifcu_buf == other.ifcu_buf &&
                    self.ifcu_req == other.ifcu_req
                }
            }
        }

        #[cfg(libc_union)]
        impl ::fmt::Debug for __c_anonymous_ifc_ifcu {
            fn fmt(&self, f: &mut ::fmt::Formatter) -> ::fmt::Result {
                f.debug_struct("ifc_ifcu")
                    .field("ifcu_buf", unsafe { &self.ifcu_buf })
                    .field("ifcu_req", unsafe { &self.ifcu_req })
                    .finish()
            }
        }

        #[cfg(libc_union)]
        impl ::hash::Hash for __c_anonymous_ifc_ifcu {
            fn hash<H: ::hash::Hasher>(&self, state: &mut H) {
                unsafe { self.ifcu_buf.hash(state) };
                unsafe { self.ifcu_req.hash(state) };
            }
        }
    }
}

s! {
    pub struct ifconf {
        pub ifc_len: ::c_int,
        #[cfg(libc_union)]
        pub ifc_ifcu: __c_anonymous_ifc_ifcu,
        #[cfg(not(libc_union))]
        pub ifc_ifcu: *mut ifreq,
    }
}

extern "C" {
    pub fn if_nameindex() -> *mut if_nameindex;
    pub fn if_freenameindex(ptr: *mut if_nameindex);
}

// asm/socket.h
pub const FIONREAD: Ioctl = 0x4008667f;
pub const FIONBIO: Ioctl = 0x8004667e;
pub const FIOASYNC: Ioctl = 0x8008667d;
pub const FIOCLEX: Ioctl = 0; // FIXME: does not exist on Cygwin!
pub const SIOCGIFCONF: c_ulong = 0x80107364;
pub const SIOCGIFFLAGS: c_ulong = 0x80507365;
pub const SIOCGIFADDR: c_ulong = 0x80507366;
pub const SIOCGIFBRDADDR: c_ulong = 0x80507367;
pub const SIOCGIFNETMASK: c_ulong = 0x80507368;
pub const SIOCGIFHWADDR: c_ulong = 0x80507369;
pub const SIOCGIFMETRIC: c_ulong = 0x8050736a;
pub const SIOCGIFMTU: c_ulong = 0x8050736b;
pub const SIOCGIFINDEX: c_ulong = 0x8050736c;
pub const SIOGIFINDEX: c_ulong = SIOCGIFINDEX;
pub const SIOCGIFDSTADDR: c_ulong = 0x8050736e;
pub const SOL_SOCKET: ::c_int = 0xffff;
pub const SO_DEBUG: ::c_int = 1;
pub const SO_ACCEPTCONN: ::c_int = 0x0002;
pub const SO_REUSEADDR: ::c_int = 0x0004;
pub const SO_KEEPALIVE: ::c_int = 0x0008;
pub const SO_DONTROUTE: ::c_int = 0x0010;
pub const SO_BROADCAST: ::c_int = 0x0020;
pub const SO_USELOOPBACK: ::c_int = 0x0040;
pub const SO_LINGER: ::c_int = 0x0080;
pub const SO_OOBINLINE: ::c_int = 0x0100;
pub const SO_PEERCRED: ::c_int = 0x0200;
pub const SO_PASSCRED: ::c_int = 0x0400;
pub const SO_SNDBUF: ::c_int = 0x1001;
pub const SO_RCVBUF: ::c_int = 0x1002;
pub const SO_SNDLOWAT: ::c_int = 0x1003;
pub const SO_RCVLOWAT: ::c_int = 0x1004;
pub const SO_SNDTIMEO: ::c_int = 0x1005;
pub const SO_RCVTIMEO: ::c_int = 0x1006;
pub const SO_ERROR: ::c_int = 0x1007;
pub const SO_TYPE: ::c_int = 0x1008;

// sys/uio.h
extern "C" {
    pub fn readv(fd: ::c_int, iov: *const ::iovec, iovcnt: ::c_int) -> ::ssize_t;
    pub fn writev(fd: ::c_int, iov: *const ::iovec, iovcnt: ::c_int) -> ::ssize_t;
}

// cygwin/_ucred.h
s! {
    pub struct ucred {
        pub pid: ::pid_t,
        pub uid: ::uid_t,
        pub gid: ::gid_t,
    }
}

// cygwin/socket.h
s_no_extra_traits! {
    pub struct sockaddr {
        pub sa_family: sa_family_t,
        pub sa_data: [c_char; 14],
    }
}

cfg_if! {
    if #[cfg(feature = "extra_traits")] {
        impl PartialEq for sockaddr {
            fn eq(&self, other: &sockaddr) -> bool {
                self.sa_family == other.sa_family
                    && self
                    .sa_data
                    .iter()
                    .zip(other.sa_data.iter())
                    .all(|(a,b)| a == b)
            }
        }

        impl Eq for sockaddr {}

        impl ::fmt::Debug for sockaddr {
            fn fmt(&self, f: &mut ::fmt::Formatter) -> ::fmt::Result {
                f.debug_struct("sockaddr")
                    .field("sa_family", &self.sa_family)
                    // FIXME: .field("sa_data", &self.sa_data)
                    .finish()
            }
        }

        impl ::hash::Hash for sockaddr {
            fn hash<H: ::hash::Hasher>(&self, state: &mut H) {
                self.sa_family.hash(state);
                self.sa_data.hash(state);
            }
        }
    }
}

s_no_extra_traits! {
    pub struct sockaddr_storage {
        pub ss_family: sa_family_t,
        __ss_pad1: [c_char; 6],
        __ss_align: i64,
        __ss_pad2: [c_char; 112],
    }
}

cfg_if! {
    if #[cfg(feature = "extra_traits")] {
        impl PartialEq for sockaddr_storage {
            fn eq(&self, other: &sockaddr_storage) -> bool {
                self.ss_family == other.ss_family
            }
        }

        impl Eq for sockaddr_storage {}

        impl ::fmt::Debug for sockaddr_storage {
            fn fmt(&self, f: &mut ::fmt::Formatter) -> ::fmt::Result {
                f.debug_struct("sockaddr_storage")
                    .field("ss_family", &self.ss_family)
                    .finish()
            }
        }

        impl ::hash::Hash for sockaddr_storage {
            fn hash<H: ::hash::Hasher>(&self, state: &mut H) {
                self.ss_family.hash(state);
            }
        }
    }
}

s! {
   pub struct msghdr {
        pub msg_name: *mut ::c_void,
        pub msg_namelen: socklen_t,
        pub msg_iov: *mut ::iovec,
        pub msg_iovlen: ::c_int,
        pub msg_control: *mut ::c_void,
        pub msg_controllen: socklen_t,
        pub msg_flags: ::c_int,
    }

    pub struct cmsghdr {
        pub cmsg_len: ::size_t,
        pub cmsg_level: ::c_int,
        pub cmsg_type: ::c_int,
    }
}

const_fn! {
    {const} fn CMSG_ALIGN(len: usize) -> usize {
        len + ::mem::size_of::<usize>() - 1 & !(::mem::size_of::<usize>() - 1)
    }
}

f! {
    pub fn CMSG_LEN(length: ::c_uint) -> ::c_uint {
        CMSG_ALIGN(::mem::size_of::<cmsghdr>()) as ::c_uint + length
    }

    pub {const} fn CMSG_SPACE(length: ::c_uint) -> ::c_uint {
        (CMSG_ALIGN(length as usize) + CMSG_ALIGN(::mem::size_of::<cmsghdr>()))
            as ::c_uint
    }

    pub fn CMSG_FIRSTHDR(mhdr: *const msghdr) -> *mut cmsghdr {
        if (*mhdr).msg_controllen as usize >= ::mem::size_of::<cmsghdr>() {
            (*mhdr).msg_control as *mut cmsghdr
        } else {
            0 as *mut cmsghdr
        }
    }

    pub fn CMSG_NXTHDR(mhdr: *const msghdr,
                       cmsg: *const cmsghdr) -> *mut cmsghdr {
        let next = (cmsg as usize +
                    CMSG_ALIGN((*cmsg).cmsg_len as usize))
            as *mut cmsghdr;
        let max = (*mhdr).msg_control as usize
            + (*mhdr).msg_controllen as usize;
        if next as usize + CMSG_ALIGN(::mem::size_of::<cmsghdr>()) as usize > max
        {
            0 as *mut cmsghdr
        } else {
            next as *mut cmsghdr
        }
    }

    pub fn CMSG_DATA(cmsg: *const cmsghdr) -> *mut ::c_uchar {
        cmsg.offset(1) as *mut ::c_uchar
    }
}

pub const SCM_RIGHTS: ::c_int = 0x01;
pub const SCM_CREDENTIALS: ::c_int = 0x02;
pub const SOCK_STREAM: ::c_int = 1;
pub const SOCK_DGRAM: ::c_int = 2;
pub const SOCK_RAW: ::c_int = 3;
pub const SOCK_RDM: ::c_int = 4;
pub const SOCK_SEQPACKET: ::c_int = 5;
pub const SOCK_NONBLOCK: ::c_int = 0x01000000;
pub const SOCK_CLOEXEC: ::c_int = 0x02000000;
pub const AF_UNSPEC: ::c_int = 0;
pub const AF_LOCAL: ::c_int = 1;
pub const AF_UNIX: ::c_int = AF_LOCAL;
pub const AF_INET: ::c_int = 2;
pub const AF_IMPLINK: ::c_int = 3;
pub const AF_PUP: ::c_int = 4;
pub const AF_CHAOS: ::c_int = 5;
pub const AF_NS: ::c_int = 6;
pub const AF_ISO: ::c_int = 7;
pub const AF_OSI: ::c_int = AF_ISO;
pub const AF_ECMA: ::c_int = 8;
pub const AF_DATAKIT: ::c_int = 9;
pub const AF_CCITT: ::c_int = 10;
pub const AF_SNA: ::c_int = 11;
pub const AF_DECnet: ::c_int = 12;
pub const AF_DLI: ::c_int = 13;
pub const AF_LAT: ::c_int = 14;
pub const AF_HYLINK: ::c_int = 15;
pub const AF_APPLETALK: ::c_int = 16;
pub const AF_NETBIOS: ::c_int = 17;
pub const AF_INET6: ::c_int = 23;
pub const PF_UNSPEC: ::c_int = AF_UNSPEC;
pub const PF_LOCAL: ::c_int = AF_LOCAL;
pub const PF_UNIX: ::c_int = PF_LOCAL;
pub const PF_INET: ::c_int = AF_INET;
pub const PF_IMPLINK: ::c_int = AF_IMPLINK;
pub const PF_PUP: ::c_int = AF_PUP;
pub const PF_CHAOS: ::c_int = AF_CHAOS;
pub const PF_NS: ::c_int = AF_NS;
pub const PF_ISO: ::c_int = AF_ISO;
pub const PF_OSI: ::c_int = AF_ISO;
pub const PF_DATAKIT: ::c_int = AF_DATAKIT;
pub const PF_CCITT: ::c_int = AF_CCITT;
pub const PF_SNA: ::c_int = AF_SNA;
pub const PF_DECnet: ::c_int = AF_DECnet;
pub const PF_DLI: ::c_int = AF_DLI;
pub const PF_LAT: ::c_int = AF_LAT;
pub const PF_HYLINK: ::c_int = AF_HYLINK;
pub const PF_APPLETALK: ::c_int = AF_APPLETALK;
pub const PF_NETBIOS: ::c_int = AF_NETBIOS;
pub const PF_INET6: ::c_int = AF_INET6;
pub const SOMAXCONN: ::c_int = 0x7fffffff;
pub const MSG_OOB: ::c_int = 0x1;
pub const MSG_PEEK: ::c_int = 0x2;
pub const MSG_DONTROUTE: ::c_int = 0x4;
pub const MSG_WAITALL: ::c_int = 0x8;
pub const MSG_DONTWAIT: ::c_int = 0x10;
pub const MSG_NOSIGNAL: ::c_int = 0x20;
pub const MSG_TRUNC: ::c_int = 0x0100;
pub const MSG_CTRUNC: ::c_int = 0x0200;
pub const MSG_BCAST: ::c_int = 0x0400;
pub const MSG_MCAST: ::c_int = 0x0800;
pub const MSG_CMSG_CLOEXEC: ::c_int = 0x1000;
pub const MSG_EOR: ::c_int = 0x8000;
pub const SOL_IP: ::c_int = 0;
pub const SOL_IPV6: ::c_int = 41;
pub const SOL_TCP: ::c_int = 6;
pub const SOL_UDP: ::c_int = 17;
pub const IPTOS_LOWDELAY: u8 = 0x10;
pub const IPTOS_THROUGHPUT: u8 = 0x08;
pub const IPTOS_RELIABILITY: u8 = 0x04;
pub const IP_DEFAULT_MULTICAST_TTL: ::c_int = 1;
pub const IP_DEFAULT_MULTICAST_LOOP: ::c_int = 1;
pub const IP_OPTIONS: ::c_int = 1;
pub const IP_HDRINCL: ::c_int = 2;
pub const IP_TOS: ::c_int = 3;
pub const IP_TTL: ::c_int = 4;
pub const IP_MULTICAST_IF: ::c_int = 9;
pub const IP_MULTICAST_TTL: ::c_int = 10;
pub const IP_MULTICAST_LOOP: ::c_int = 11;
pub const IP_ADD_MEMBERSHIP: ::c_int = 12;
pub const IP_DROP_MEMBERSHIP: ::c_int = 13;
pub const IP_ADD_SOURCE_MEMBERSHIP: ::c_int = 15;
pub const IP_DROP_SOURCE_MEMBERSHIP: ::c_int = 16;
pub const IP_BLOCK_SOURCE: ::c_int = 17;
pub const IP_UNBLOCK_SOURCE: ::c_int = 18;
pub const IP_PKTINFO: ::c_int = 19;
pub const IP_UNICAST_IF: ::c_int = 31;
pub const IPV6_HOPOPTS: ::c_int = 1;
pub const IPV6_UNICAST_HOPS: ::c_int = 4;
pub const IPV6_MULTICAST_IF: ::c_int = 9;
pub const IPV6_MULTICAST_HOPS: ::c_int = 10;
pub const IPV6_MULTICAST_LOOP: ::c_int = 11;
pub const IPV6_ADD_MEMBERSHIP: ::c_int = 12;
pub const IPV6_DROP_MEMBERSHIP: ::c_int = 13;
pub const IPV6_JOIN_GROUP: ::c_int = 12;
pub const IPV6_LEAVE_GROUP: ::c_int = 13;
pub const IPV6_DONTFRAG: ::c_int = 14;
pub const IPV6_PKTINFO: ::c_int = 19;
pub const IPV6_HOPLIMIT: ::c_int = 21;
pub const IPV6_CHECKSUM: ::c_int = 26;
pub const IPV6_V6ONLY: ::c_int = 27;
pub const IPV6_UNICAST_IF: ::c_int = 31;
pub const IPV6_RTHDR: ::c_int = 32;
pub const IPV6_RECVRTHDR: ::c_int = 38;
pub const IPV6_TCLASS: ::c_int = 39;
pub const IPV6_RECVTCLASS: ::c_int = 40;
pub const MCAST_JOIN_GROUP: ::c_int = 41;
pub const MCAST_LEAVE_GROUP: ::c_int = 42;
pub const MCAST_BLOCK_SOURCE: ::c_int = 43;
pub const MCAST_UNBLOCK_SOURCE: ::c_int = 44;
pub const MCAST_JOIN_SOURCE_GROUP: ::c_int = 45;
pub const MCAST_LEAVE_SOURCE_GROUP: ::c_int = 46;
pub const MCAST_INCLUDE: ::c_int = 0;
pub const MCAST_EXCLUDE: ::c_int = 1;
pub const SHUT_RD: ::c_int = 0;
pub const SHUT_WR: ::c_int = 1;
pub const SHUT_RDWR: ::c_int = 2;

// cygwin/stat.h
s_no_extra_traits! {
    pub struct stat {
        pub st_dev: dev_t,
        pub st_ino: ino_t,
        pub st_mode: mode_t,
        pub st_nlink: nlink_t,
        pub st_uid: ::uid_t,
        pub st_gid: ::gid_t,
        pub st_rdev: dev_t,
        pub st_size: off_t,
        pub st_atime: time_t,
        pub st_atime_nsec: c_long,
        pub st_mtime: time_t,
        pub st_mtime_nsec: c_long,
        pub st_ctime: time_t,
        pub st_ctime_nsec: c_long,
        pub st_blksize: blksize_t,
        pub st_blocks: blkcnt_t,
        pub st_birthtime: time_t,
        pub st_birthtime_nsec: c_long,
    }
}

cfg_if! {
    if #[cfg(feature = "extra_traits")] {
        impl PartialEq for stat {
            fn eq(&self, other: &stat) -> bool {
                self.st_dev == other.st_dev
                    && self.st_ino == other.st_ino
                    && self.st_mode == other.st_mode
                    && self.st_nlink == other.st_nlink
                    && self.st_uid == other.st_uid
                    && self.st_gid == other.st_gid
                    && self.st_rdev == other.st_rdev
                    && self.st_size == other.st_size
                    && self.st_atime == other.st_atime
                    && self.st_atime_nsec == other.st_atime_nsec
                    && self.st_mtime == other.st_mtime
                    && self.st_mtime_nsec == other.st_mtime_nsec
                    && self.st_ctime == other.st_ctime
                    && self.st_ctime_nsec == other.st_ctime_nsec
                    && self.st_blksize == other.st_blksize
                    && self.st_blocks == other.st_blocks
                    && self.st_birthtime == other.st_birthtime
                    && self.st_birthtime_nsec == other.st_birthtime_nsec
            }
        }

        impl Eq for stat {}

        impl ::fmt::Debug for stat {
            fn fmt(&self, f: &mut ::fmt::Formatter) -> ::fmt::Result {
                f.debug_struct("stat")
                    .field("st_dev", &self.st_dev)
                    .field("st_ino", &self.st_ino)
                    .field("st_mode", &self.st_mode)
                    .field("st_nlink", &self.st_nlink)
                    .field("st_uid", &self.st_uid)
                    .field("st_gid", &self.st_gid)
                    .field("st_rdev", &self.st_rdev)
                    .field("st_size", &self.st_size)
                    .field("st_atime", &self.st_atime)
                    .field("st_atime_nsec", &self.st_atime_nsec)
                    .field("st_mtime", &self.st_mtime)
                    .field("st_mtime_nsec", &self.st_mtime_nsec)
                    .field("st_ctime", &self.st_ctime)
                    .field("st_ctime_nsec", &self.st_ctime_nsec)
                    .field("st_blksize", &self.st_blksize)
                    .field("st_blocks", &self.st_blocks)
                    .field("st_birthtime", &self.st_birthtime)
                    .field("st_birthtime_nsec", &self.st_birthtime_nsec)
                    .finish()
            }
        }

        impl ::hash::Hash for stat {
            fn hash<H: ::hash::Hasher>(&self, state: &mut H) {
                self.st_dev.hash(state);
                self.st_ino.hash(state);
                self.st_mode.hash(state);
                self.st_nlink.hash(state);
                self.st_uid.hash(state);
                self.st_gid.hash(state);
                self.st_rdev.hash(state);
                self.st_size.hash(state);
                self.st_atime.hash(state);
                self.st_atime_nsec.hash(state);
                self.st_mtime.hash(state);
                self.st_mtime_nsec.hash(state);
                self.st_ctime.hash(state);
                self.st_ctime_nsec.hash(state);
                self.st_blksize.hash(state);
                self.st_blocks.hash(state);
                self.st_birthtime.hash(state);
                self.st_birthtime_nsec.hash(state);
            }
        }
    }
}

// sys/stat.h
pub const S_BLKSIZE: mode_t = 1024;
pub const S_IREAD: mode_t = 256;
pub const S_IWRITE: mode_t = 128;
pub const S_IEXEC: mode_t = 64;
pub const S_ENFMT: mode_t = 1024;
pub const S_IFMT: mode_t = 61440;
pub const S_IFDIR: mode_t = 16384;
pub const S_IFCHR: mode_t = 8192;
pub const S_IFBLK: mode_t = 24576;
pub const S_IFREG: mode_t = 32768;
pub const S_IFLNK: mode_t = 40960;
pub const S_IFSOCK: mode_t = 49152;
pub const S_IFIFO: mode_t = 4096;
pub const S_IRWXU: mode_t = 448;
pub const S_IRUSR: mode_t = 256;
pub const S_IWUSR: mode_t = 128;
pub const S_IXUSR: mode_t = 64;
pub const S_IRWXG: mode_t = 56;
pub const S_IRGRP: mode_t = 32;
pub const S_IWGRP: mode_t = 16;
pub const S_IXGRP: mode_t = 8;
pub const S_IRWXO: mode_t = 7;
pub const S_IROTH: mode_t = 4;
pub const S_IWOTH: mode_t = 2;
pub const S_IXOTH: mode_t = 1;
pub const UTIME_NOW: c_long = -2;
pub const UTIME_OMIT: c_long = -1;

extern "C" {
    pub fn mkfifoat(dirfd: ::c_int, pathname: *const c_char, mode: mode_t) -> ::c_int;

    pub fn mknodat(
        dirfd: ::c_int,
        pathname: *const c_char,
        mode: mode_t,
        dev: dev_t,
    ) -> ::c_int;

    pub fn utimensat(
        dirfd: ::c_int,
        path: *const c_char,
        times: *const ::timespec,
        flag: ::c_int,
    ) -> ::c_int;

    pub fn futimens(fd: ::c_int, times: *const ::timespec) -> ::c_int;
}

// cygwin/limits.h
pub const ARG_MAX: ::c_int = 32000;
pub const CHILD_MAX: ::c_int = 256;
pub const IOV_MAX: ::c_int = 1024;
pub const PTHREAD_STACK_MIN: ::size_t = 65536;
pub const PATH_MAX: ::c_int = 4096;
pub const PIPE_BUF: usize = 4096;
pub const NGROUPS_MAX: ::c_int = 1024;

// sys/dirent.h
s_no_extra_traits! {
    pub struct dirent {
        __d_version: u32,
        pub d_ino: ino_t,
        pub d_type: ::c_uchar,
        __d_unused1: [::c_uchar; 3],
        __d_internal1: u32,
        pub d_name: [c_char; 256],
    }
}

cfg_if! {
    if #[cfg(feature = "extra_traits")] {
        impl PartialEq for dirent {
            fn eq(&self, other: &dirent) -> bool {
                self.d_ino == other.d_ino
                    && self.d_type == other.d_type
                    && self
                    .d_name
                    .iter()
                    .zip(other.d_name.iter())
                    .all(|(a,b)| a == b)
            }
        }

        impl Eq for dirent {}

        impl ::fmt::Debug for dirent {
            fn fmt(&self, f: &mut ::fmt::Formatter) -> ::fmt::Result {
                f.debug_struct("dirent")
                    .field("d_ino", &self.d_ino)
                    .field("d_type", &self.d_type)
                    // FIXME: .field("d_name", &self.d_name)
                    .finish()
            }
        }

        impl ::hash::Hash for dirent {
            fn hash<H: ::hash::Hasher>(&self, state: &mut H) {
                self.d_ino.hash(state);
                self.d_type.hash(state);
                self.d_name.hash(state);
            }
        }
    }
}

// dlfcn.h

// specific to CYGWIN
pub const FORK_RELOAD: ::c_int = 1;
pub const FORK_NO_RELOAD: ::c_int = 0;
extern "C" {
    pub fn dlfork(val: ::c_int);
}

pub const RTLD_DEFAULT: *mut ::c_void = 0isize as *mut ::c_void;
pub const RTLD_LOCAL: ::c_int = 0;
pub const RTLD_LAZY: ::c_int = 1;
pub const RTLD_NOW: ::c_int = 2;
pub const RTLD_GLOBAL: ::c_int = 4;
pub const RTLD_NODELETE: ::c_int = 8;
pub const RTLD_NOLOAD: ::c_int = 16;
pub const RTLD_DEEPBIND: ::c_int = 32;

s! {
    pub struct Dl_info {
        pub dli_fname: [::c_char; ::PATH_MAX as usize],
        pub dli_fbase: *mut ::c_void,
        pub dli_sname: *const c_char,
        pub dli_saddr: *mut ::c_void,
    }
}

// sys/socket.h
extern "C" {
    pub fn accept4(
        s: ::c_int,
        addr: *mut sockaddr,
        addrlen: *mut socklen_t,
        flags: ::c_int,
    ) -> ::c_int;

    pub fn bind(socket: ::c_int, address: *const sockaddr, address_len: socklen_t) -> ::c_int;

    pub fn recvfrom(
        socket: ::c_int,
        buf: *mut ::c_void,
        len: ::size_t,
        flags: ::c_int,
        addr: *mut sockaddr,
        addrlen: *mut socklen_t,
    ) -> ::ssize_t;

    pub fn recvmsg(fd: ::c_int, msg: *mut msghdr, flags: ::c_int) -> ::ssize_t;
    pub fn sendmsg(fd: ::c_int, msg: *const msghdr, flags: ::c_int) -> ::ssize_t;
}

// cygwin/in6.h
s! {
    pub struct in6_pktinfo {
        pub ipi6_addr: ::in6_addr,
        pub ipi6_ifindex: u32,
    }

    pub struct sockaddr_in6 {
        pub sin6_family: sa_family_t,
        pub sin6_port: ::in_port_t,
        pub sin6_flowinfo: u32,
        pub sin6_addr: ::in6_addr,
        pub sin6_scope_id: u32,
    }
}

// cygwin/in.h

/// IP6 hop-by-hop options
pub const IPPROTO_HOPOPTS: ::c_int = 0;

/// gateway mgmt protocol
pub const IPPROTO_IGMP: ::c_int = 2;

/// IPIP tunnels (older KA9Q tunnels use 94)
pub const IPPROTO_IPIP: ::c_int = 4;

/// exterior gateway protocol
pub const IPPROTO_EGP: ::c_int = 8;

/// pup
pub const IPPROTO_PUP: ::c_int = 12;

/// xns idp
pub const IPPROTO_IDP: ::c_int = 22;

/// IP6 routing header
pub const IPPROTO_ROUTING: ::c_int = 43;

/// IP6 fragmentation header
pub const IPPROTO_FRAGMENT: ::c_int = 44;

/// IP6 Encap Sec. Payload
pub const IPPROTO_ESP: ::c_int = 50;

/// IP6 Auth Header
pub const IPPROTO_AH: ::c_int = 51;

/// IP6 no next header
pub const IPPROTO_NONE: ::c_int = 59;

/// IP6 destination option
pub const IPPROTO_DSTOPTS: ::c_int = 60;

pub const IPPROTO_RAW: ::c_int = 255;
pub const IPPROTO_MAX: ::c_int = 256;

s_no_extra_traits! {
    pub struct in_addr {
        pub s_addr: ::in_addr_t,
    }
}

cfg_if! {
    if #[cfg(feature = "extra_traits")] {
        impl PartialEq for in_addr {
            fn eq(&self, other: &in_addr) -> bool {
                self.s_addr == other.s_addr
            }
        }

        impl Eq for in_addr {}

        impl ::fmt::Debug for in_addr {
            fn fmt(&self, f: &mut ::fmt::Formatter) -> ::fmt::Result {
                let s_addr = self.s_addr;
                f.debug_struct("in_addr")
                    .field("s_addr", &s_addr)
                    .finish()
            }
        }

        impl ::hash::Hash for in_addr {
            fn hash<H: ::hash::Hasher>(&self, state: &mut H) {
                let s_addr = self.s_addr;
                s_addr.hash(state);
            }
        }
    }
}

s_no_extra_traits! {
    pub struct ip_mreq {
        pub imr_multiaddr: in_addr,
        pub imr_interface: in_addr,
    }
}

cfg_if! {
    if #[cfg(feature = "extra_traits")] {
        impl PartialEq for ip_mreq {
            fn eq(&self, other: &ip_mreq) -> bool {
                self.imr_multiaddr == other.imr_multiaddr
                    && self.imr_interface == other.imr_interface
            }
        }

        impl Eq for ip_mreq {}

        impl ::fmt::Debug for ip_mreq {
            fn fmt(&self, f: &mut ::fmt::Formatter) -> ::fmt::Result {
                f.debug_struct("ip_mreq")
                    .field("imr_multiaddr", &self.imr_multiaddr)
                    .field("imr_interface", &self.imr_interface)
                    .finish()
            }
        }

        impl ::hash::Hash for ip_mreq {
            fn hash<H: ::hash::Hasher>(&self, state: &mut H) {
                self.imr_multiaddr.hash(state);
                self.imr_interface.hash(state);
            }
        }
    }
}

s! {
    pub struct ip_mreq_source {
        pub imr_multiaddr: in_addr,
        pub imr_sourceaddr: in_addr,
        pub imr_interface: in_addr,
    }
}

s_no_extra_traits! {
    pub struct in_pktinfo {
        pub ipi_addr: in_addr,
        pub ipi_ifindex: u32,
    }
}

cfg_if! {
    if #[cfg(feature = "extra_traits")] {
        impl PartialEq for in_pktinfo {
            fn eq(&self, other: &in_pktinfo) -> bool {
                self.ipi_addr == other.ipi_addr
                    && self.ipi_ifindex == other.ipi_ifindex
            }
        }

        impl Eq for in_pktinfo {}

        impl ::fmt::Debug for in_pktinfo {
            fn fmt(&self, f: &mut ::fmt::Formatter) -> ::fmt::Result {
                f.debug_struct("in_pktinfo")
                    .field("ipi_addr", &self.ipi_addr)
                    .field("ipi_ifindex", &self.ipi_ifindex)
                    .finish()
            }
        }

        impl ::hash::Hash for in_pktinfo {
            fn hash<H: ::hash::Hasher>(&self, state: &mut H) {
                self.ipi_addr.hash(state);
                self.ipi_ifindex.hash(state);
            }
        }
    }
}

s_no_extra_traits! {
     pub struct sockaddr_in {
        pub sin_family: sa_family_t,
        pub sin_port: ::in_port_t,
        pub sin_addr: in_addr,
        pub sin_zero: [u8; 8],
    }
}

cfg_if! {
    if #[cfg(feature = "extra_traits")] {
        impl PartialEq for sockaddr_in {
            fn eq(&self, other: &sockaddr_in) -> bool {
                self.sin_family == other.sin_family
                    && self.sin_port == other.sin_port
                    && self.sin_addr == other.sin_addr
                    && self.sin_zero == other.sin_zero
            }
        }

        impl Eq for sockaddr_in {}

        impl ::fmt::Debug for sockaddr_in {
            fn fmt(&self, f: &mut ::fmt::Formatter) -> ::fmt::Result {
                f.debug_struct("sockaddr_in")
                    .field("sin_family", &self.sin_family)
                    .field("sin_port", &self.sin_port)
                    .field("sin_addr", &self.sin_addr)
                    .field("sin_zero", &self.sin_zero)
                    .finish()
            }
        }

        impl ::hash::Hash for sockaddr_in {
            fn hash<H: ::hash::Hasher>(&self, state: &mut H) {
                self.sin_family.hash(state);
                self.sin_port.hash(state);
                self.sin_addr.hash(state);
                self.sin_zero.hash(state);
            }
        }
    }
}

// netdb.h
s! {
    pub struct addrinfo {
        pub ai_flags: ::c_int,
        pub ai_family: ::c_int,
        pub ai_socktype: ::c_int,
        pub ai_protocol: ::c_int,
        pub ai_addrlen: socklen_t,
        pub ai_canonname: *mut c_char,
        pub ai_addr: *mut sockaddr,
        pub ai_next: *mut addrinfo,
    }
}

pub const AI_PASSIVE: ::c_int = 0x1;
pub const AI_CANONNAME: ::c_int = 0x2;
pub const AI_NUMERICHOST: ::c_int = 0x4;
pub const AI_NUMERICSERV: ::c_int = 0x8;
pub const AI_ALL: ::c_int = 0x100;
pub const AI_ADDRCONFIG: ::c_int = 0x400;
pub const AI_V4MAPPED: ::c_int = 0x800;
pub const NI_NOFQDN: ::c_int = 0x1;
pub const NI_NUMERICHOST: ::c_int = 0x2;
pub const NI_NAMEREQD: ::c_int = 0x4;
pub const NI_NUMERICSERV: ::c_int = 0x8;
pub const NI_DGRAM: ::c_int = 0x10;
pub const NI_MAXHOST: ::c_int = 1025;
pub const NI_MAXSERV: ::c_int = 32;
pub const EAI_AGAIN: ::c_int = 2;
pub const EAI_BADFLAGS: ::c_int = 3;
pub const EAI_FAIL: ::c_int = 4;
pub const EAI_FAMILY: ::c_int = 5;
pub const EAI_MEMORY: ::c_int = 6;
pub const EAI_NODATA: ::c_int = 7;
pub const EAI_NONAME: ::c_int = 8;
pub const EAI_SERVICE: ::c_int = 9;
pub const EAI_SOCKTYPE: ::c_int = 10;
pub const EAI_SYSTEM: ::c_int = 11;
pub const EAI_OVERFLOW: ::c_int = 14;

extern "C" {
    pub fn getnameinfo(
        sa: *const sockaddr,
        salen: socklen_t,
        host: *mut c_char,
        hostlen: socklen_t,
        serv: *mut c_char,
        sevlen: ::socklen_t,
        flags: ::c_int,
    ) -> ::c_int;
}

// sys/poll.h
pub const POLLIN: ::c_short = 0x1;
pub const POLLPRI: ::c_short = 0x2;
pub const POLLOUT: ::c_short = 0x4;
pub const POLLERR: ::c_short = 0x8;
pub const POLLHUP: ::c_short = 0x10;
pub const POLLNVAL: ::c_short = 0x20;
pub const POLLRDNORM: ::c_short = 0x1;
pub const POLLRDBAND: ::c_short = 0x2;
pub const POLLWRNORM: ::c_short = 0x4;
pub const POLLWRBAND: ::c_short = 0x4;

pub type nfds_t = ::c_uint;

extern "C" {
    pub fn ppoll(
        fds: *mut ::pollfd,
        nfds: nfds_t,
        timeout: *const ::timespec,
        sigmask: *const sigset_t,
    ) -> ::c_int;
}

// locale.h
pub const LC_ALL: ::c_int = 0;
pub const LC_COLLATE: ::c_int = 1;
pub const LC_CTYPE: ::c_int = 2;
pub const LC_MONETARY: ::c_int = 3;
pub const LC_NUMERIC: ::c_int = 4;
pub const LC_TIME: ::c_int = 5;
pub const LC_MESSAGES: ::c_int = 6;
pub const LC_ALL_MASK: ::c_int = 1 << 0;
pub const LC_COLLATE_MASK: ::c_int = 1 << 1;
pub const LC_CTYPE_MASK: ::c_int = 1 << 2;
pub const LC_MONETARY_MASK: ::c_int = 1 << 3;
pub const LC_NUMERIC_MASK: ::c_int = 1 << 4;
pub const LC_TIME_MASK: ::c_int = 1 << 5;
pub const LC_MESSAGES_MASK: ::c_int = 1 << 6;
pub const LC_GLOBAL_LOCALE: ::locale_t = -1isize as ::locale_t;

s! {
    pub struct lconv {
        pub decimal_point: *mut c_char,
        pub thousands_sep: *mut c_char,
        pub grouping: *mut c_char,
        pub int_curr_symbol: *mut c_char,
        pub currency_symbol: *mut c_char,
        pub mon_decimal_point: *mut c_char,
        pub mon_thousands_sep: *mut c_char,
        pub mon_grouping: *mut c_char,
        pub positive_sign: *mut c_char,
        pub negative_sign: *mut c_char,
        pub int_frac_digits: c_char,
        pub frac_digits: c_char,
        pub p_cs_precedes: c_char,
        pub p_sep_by_space: c_char,
        pub n_cs_precedes: c_char,
        pub n_sep_by_space: c_char,
        pub p_sign_posn: c_char,
        pub n_sign_posn: c_char,
        pub int_n_cs_precedes: c_char,
        pub int_n_sep_by_space: c_char,
        pub int_n_sign_posn: c_char,
        pub int_p_cs_precedes: c_char,
        pub int_p_sep_by_space: c_char,
        pub int_p_sign_posn: c_char,
    }
}

extern "C" {
    pub fn newlocale(mask: ::c_int, locale: *const c_char, base: ::locale_t) -> ::locale_t;
    pub fn freelocale(loc: ::locale_t);
    pub fn duplocale(base: ::locale_t) -> ::locale_t;
    pub fn uselocale(loc: ::locale_t) -> ::locale_t;
}

// semaphore.h
#[cfg_attr(feature = "extra_traits", derive(Debug))]
pub enum sem {}
impl ::Copy for sem {}
impl ::Clone for sem {
    fn clone(&self) -> sem {
        *self
    }
}

pub type sem_t = *mut sem;

pub const SEM_FAILED: *mut sem_t = 0 as *mut sem_t;

extern "C" {
    pub fn sem_init(sem: *mut sem_t, pshared: ::c_int, value: ::c_uint) -> ::c_int;
    pub fn sem_destroy(sem: *mut sem_t) -> ::c_int;
    pub fn sem_open(name: *const c_char, oflag: ::c_int, ...) -> *mut sem_t;
    pub fn sem_close(sem: *mut sem_t) -> ::c_int;
    pub fn sem_unlink(name: *const c_char) -> ::c_int;
    pub fn sem_timedwait(sem: *mut sem_t, abstime: *const ::timespec) -> ::c_int;
    pub fn sem_getvalue(sem: *mut sem_t, sval: *mut ::c_int) -> ::c_int;
}

// sys/statvfs.h
pub const ST_RDONLY: c_ulong = 0x80000;
pub const ST_NOSUID: c_ulong = 0;

s_no_extra_traits! {
    pub struct statvfs {
        pub f_bsize: c_ulong,
        pub f_frsize: c_ulong,
        pub f_blocks: fsblkcnt_t,
        pub f_bfree: fsblkcnt_t,
        pub f_bavail: fsblkcnt_t,
        pub f_files: fsfilcnt_t,
        pub f_ffree: fsfilcnt_t,
        pub f_favail: fsfilcnt_t,
        pub f_fsid: c_ulong,
        pub f_flag: c_ulong,
        pub f_namemax: c_ulong,
    }
}

cfg_if! {
    if #[cfg(feature = "extra_traits")] {
        impl PartialEq for statvfs {
            fn eq(&self, other: &statvfs) -> bool {
                self.f_bsize == other.f_bsize
                    && self.f_frsize == other.f_frsize
                    && self.f_blocks == other.f_blocks
                    && self.f_bfree == other.f_bfree
                    && self.f_bavail == other.f_bavail
                    && self.f_files == other.f_files
                    && self.f_ffree == other.f_ffree
                    && self.f_favail == other.f_favail
                    && self.f_fsid == other.f_fsid
                    && self.f_flag == other.f_flag
                    && self.f_namemax == other.f_namemax
            }
        }

        impl Eq for statvfs {}

        impl ::fmt::Debug for statvfs {
            fn fmt(&self, f: &mut ::fmt::Formatter) -> ::fmt::Result {
                f.debug_struct("statvfs")
                    .field("f_bsize", &self.f_bsize)
                    .field("f_frsize", &self.f_frsize)
                    .field("f_blocks", &self.f_blocks)
                    .field("f_bfree", &self.f_bfree)
                    .field("f_bavail", &self.f_bavail)
                    .field("f_files", &self.f_files)
                    .field("f_ffree", &self.f_ffree)
                    .field("f_favail", &self.f_favail)
                    .field("f_fsid", &self.f_fsid)
                    .field("f_flag", &self.f_flag)
                    .field("f_namemax", &self.f_namemax)
                    .finish()
            }
        }

        impl ::hash::Hash for statvfs {
            fn hash<H: ::hash::Hasher>(&self, state: &mut H) {
                self.f_bsize.hash(state);
                self.f_frsize.hash(state);
                self.f_blocks.hash(state);
                self.f_bfree.hash(state);
                self.f_bavail.hash(state);
                self.f_files.hash(state);
                self.f_ffree.hash(state);
                self.f_favail.hash(state);
                self.f_fsid.hash(state);
                self.f_flag.hash(state);
                self.f_namemax.hash(state);
            }
        }
    }
}

// sys/termios.h
pub const TIOCMGET: Ioctl = 0x5415;
pub const TIOCMBIS: Ioctl = 0x5416;
pub const TIOCMBIC: Ioctl = 0x5417;
pub const TIOCMSET: Ioctl = 0x5418;
pub const TIOCINQ: Ioctl = 0x541B;
pub const TIOCSCTTY: Ioctl = 0x540E;
pub const TIOCSBRK: Ioctl = 0x5427;
pub const TIOCCBRK: Ioctl = 0x5428;
pub const TIOCM_DTR: ::c_int = 0x002;
pub const TIOCM_RTS: ::c_int = 0x004;
pub const TIOCM_CTS: ::c_int = 0x020;
pub const TIOCM_CAR: ::c_int = 0x040;
pub const TIOCM_RNG: ::c_int = 0x080;
pub const TIOCM_CD: ::c_int = TIOCM_CAR;
pub const TIOCM_RI: ::c_int = TIOCM_RNG;
pub const TCOOFF: ::c_int = 0;
pub const TCOON: ::c_int = 1;
pub const TCIOFF: ::c_int = 2;
pub const TCION: ::c_int = 3;
pub const TCGETA: Ioctl = 5;
pub const TCSETA: Ioctl = 6;
pub const TCSETAW: Ioctl = 7;
pub const TCSETAF: Ioctl = 8;
pub const TCIFLUSH: ::c_int = 0;
pub const TCOFLUSH: ::c_int = 1;
pub const TCIOFLUSH: ::c_int = 2;
pub const TCFLSH: Ioctl = 3;
pub const TCSAFLUSH: ::c_int = 1;
pub const TCSANOW: ::c_int = 2;
pub const TCSADRAIN: ::c_int = 3;
pub const TIOCPKT: ::c_int = 6;
pub const TIOCPKT_DATA: ::c_int = 0x0;
pub const TIOCPKT_FLUSHREAD: ::c_int = 0x1;
pub const TIOCPKT_FLUSHWRITE: ::c_int = 0x2;
pub const TIOCPKT_STOP: ::c_int = 0x4;
pub const TIOCPKT_START: ::c_int = 0x8;
pub const TIOCPKT_NOSTOP: ::c_int = 0x10;
pub const TIOCPKT_DOSTOP: ::c_int = 0x20;
pub const IGNBRK: tcflag_t = 0x00001;
pub const BRKINT: tcflag_t = 0x00002;
pub const IGNPAR: tcflag_t = 0x00004;
pub const IMAXBEL: tcflag_t = 0x00008;
pub const INPCK: tcflag_t = 0x00010;
pub const ISTRIP: tcflag_t = 0x00020;
pub const INLCR: tcflag_t = 0x00040;
pub const IGNCR: tcflag_t = 0x00080;
pub const ICRNL: tcflag_t = 0x00100;
pub const IXON: tcflag_t = 0x00400;
pub const IXOFF: tcflag_t = 0x01000;
pub const IUCLC: tcflag_t = 0x04000;
pub const IXANY: tcflag_t = 0x08000;
pub const PARMRK: tcflag_t = 0x10000;
pub const IUTF8: tcflag_t = 0x20000;
pub const OPOST: tcflag_t = 0x00001;
pub const OLCUC: tcflag_t = 0x00002;
pub const OCRNL: tcflag_t = 0x00004;
pub const ONLCR: tcflag_t = 0x00008;
pub const ONOCR: tcflag_t = 0x00010;
pub const ONLRET: tcflag_t = 0x00020;
pub const OFILL: tcflag_t = 0x00040;
pub const CRDLY: tcflag_t = 0x00180;
pub const CR0: tcflag_t = 0x00000;
pub const CR1: tcflag_t = 0x00080;
pub const CR2: tcflag_t = 0x00100;
pub const CR3: tcflag_t = 0x00180;
pub const NLDLY: tcflag_t = 0x00200;
pub const NL0: tcflag_t = 0x00000;
pub const NL1: tcflag_t = 0x00200;
pub const BSDLY: tcflag_t = 0x00400;
pub const BS0: tcflag_t = 0x00000;
pub const BS1: tcflag_t = 0x00400;
pub const TABDLY: tcflag_t = 0x01800;
pub const TAB0: tcflag_t = 0x00000;
pub const TAB1: tcflag_t = 0x00800;
pub const TAB2: tcflag_t = 0x01000;
pub const TAB3: tcflag_t = 0x01800;
pub const XTABS: tcflag_t = 0x01800;
pub const VTDLY: tcflag_t = 0x02000;
pub const VT0: tcflag_t = 0x00000;
pub const VT1: tcflag_t = 0x02000;
pub const FFDLY: tcflag_t = 0x04000;
pub const FF0: tcflag_t = 0x00000;
pub const FF1: tcflag_t = 0x04000;
pub const OFDEL: tcflag_t = 0x08000;
pub const CBAUD: tcflag_t = 0x0100f;
pub const B0: speed_t = 0x00000;
pub const B50: speed_t = 0x00001;
pub const B75: speed_t = 0x00002;
pub const B110: speed_t = 0x00003;
pub const B134: speed_t = 0x00004;
pub const B150: speed_t = 0x00005;
pub const B200: speed_t = 0x00006;
pub const B300: speed_t = 0x00007;
pub const B600: speed_t = 0x00008;
pub const B1200: speed_t = 0x00009;
pub const B1800: speed_t = 0x0000a;
pub const B2400: speed_t = 0x0000b;
pub const B4800: speed_t = 0x0000c;
pub const B9600: speed_t = 0x0000d;
pub const B19200: speed_t = 0x0000e;
pub const B38400: speed_t = 0x0000f;
pub const CSIZE: tcflag_t = 0x00030;
pub const CS5: tcflag_t = 0x00000;
pub const CS6: tcflag_t = 0x00010;
pub const CS7: tcflag_t = 0x00020;
pub const CS8: tcflag_t = 0x00030;
pub const CSTOPB: tcflag_t = 0x00040;
pub const CREAD: tcflag_t = 0x00080;
pub const PARENB: tcflag_t = 0x00100;
pub const PARODD: tcflag_t = 0x00200;
pub const HUPCL: tcflag_t = 0x00400;
pub const CLOCAL: tcflag_t = 0x00800;
pub const CBAUDEX: tcflag_t = 0x0100f;
pub const B57600: speed_t = 0x01001;
pub const B115200: speed_t = 0x01002;
pub const B230400: speed_t = 0x01004;
pub const B460800: speed_t = 0x01006;
pub const B500000: speed_t = 0x01007;
pub const B576000: speed_t = 0x01008;
pub const B921600: speed_t = 0x01009;
pub const B1000000: speed_t = 0x0100a;
pub const B1152000: speed_t = 0x0100b;
pub const B1500000: speed_t = 0x0100c;
pub const B2000000: speed_t = 0x0100d;
pub const B2500000: speed_t = 0x0100e;
pub const B3000000: speed_t = 0x0100f;
pub const CRTSCTS: tcflag_t = 0x08000;
pub const CMSPAR: tcflag_t = 0x40000000;
pub const ISIG: tcflag_t = 0x0001;
pub const ICANON: tcflag_t = 0x0002;
pub const ECHO: tcflag_t = 0x0004;
pub const ECHOE: tcflag_t = 0x0008;
pub const ECHOK: tcflag_t = 0x0010;
pub const ECHONL: tcflag_t = 0x0020;
pub const NOFLSH: tcflag_t = 0x0040;
pub const TOSTOP: tcflag_t = 0x0080;
pub const IEXTEN: tcflag_t = 0x0100;
pub const FLUSHO: tcflag_t = 0x0200;
pub const ECHOKE: tcflag_t = 0x0400;
pub const ECHOCTL: tcflag_t = 0x0800;
pub const VDISCARD: usize = 1;
pub const VEOL: usize = 2;
pub const VEOL2: usize = 3;
pub const VEOF: usize = 4;
pub const VERASE: usize = 5;
pub const VINTR: usize = 6;
pub const VKILL: usize = 7;
pub const VLNEXT: usize = 8;
pub const VMIN: usize = 9;
pub const VQUIT: usize = 10;
pub const VREPRINT: usize = 11;
pub const VSTART: usize = 12;
pub const VSTOP: usize = 13;
pub const VSUSP: usize = 14;
pub const VSWTC: usize = 15;
pub const VTIME: usize = 16;
pub const VWERASE: usize = 17;
pub const NCCS: usize = 18;

pub type tcflag_t = ::c_uint;
pub type speed_t = ::c_uint;

s! {
    pub struct termios {
        pub c_iflag: tcflag_t,
        pub c_oflag: tcflag_t,
        pub c_cflag: tcflag_t,
        pub c_lflag: tcflag_t,
        pub c_line:  c_char,
        pub c_cc: [::cc_t; NCCS],
        pub c_ispeed: speed_t,
        pub c_ospeed: speed_t,
    }
}

pub const TIOCGWINSZ: Ioctl = 0x5401;
pub const TIOCSWINSZ: Ioctl = 0x5402;
pub const TIOCLINUX: Ioctl = 0x5403;
pub const TIOCGPGRP: Ioctl = 0x540f;
pub const TIOCSPGRP: Ioctl = 0x5410;

// cygwin/wait.h
pub const WNOHANG: ::c_int = 1;
pub const WUNTRACED: ::c_int = 2;
pub const WCONTINUED: ::c_int = 8;

safe_f! {
    pub {const} fn WIFEXITED(status: ::c_int) -> bool {
        (status & 0xff) == 0
    }

    pub {const} fn WIFSIGNALED(status: ::c_int) -> bool {
        (status & 0o177) != 0o177 && (status & 0o177) != 0
    }

    pub {const} fn WIFSTOPPED(status: ::c_int) -> bool {
        (status & 0xff) == 0o177
    }

    pub {const} fn WIFCONTINUED(status: ::c_int) -> bool {
        (status & 0o177777) == 0o177777
    }

    pub {const} fn WEXITSTATUS(status: ::c_int) -> ::c_int {
        (status >> 8) & 0xff
    }

    pub {const} fn WTERMSIG(status: ::c_int) -> ::c_int {
        status & 0o177
    }

    pub {const} fn WSTOPSIG(status: ::c_int) -> ::c_int {
        (status >> 8) & 0xff
    }

    pub {const} fn WCOREDUMP(status: ::c_int) -> bool {
        WIFSIGNALED(status) && (status & 0x80) != 0
    }
}

// cygwin/stdlib.h
extern "C" {
    pub fn clearenv() -> ::c_int;
    pub fn ptsname_r(fd: ::c_int, buf: *mut c_char, buflen: ::size_t) -> ::c_int;
    pub fn getpt() -> ::c_int;
    pub fn memalign(align: ::size_t, size: ::size_t) -> *mut ::c_void;
    pub fn getloadavg(loadavg: *mut ::c_double, nelem: ::c_int) -> ::c_int;
}

// stdlib.h
pub const EXIT_FAILURE: ::c_int = 1;
pub const EXIT_SUCCESS: ::c_int = 0;

extern "C" {
    pub fn abs(i: ::c_int) -> ::c_int;
    pub fn arc4random() -> u32;
    pub fn arc4random_uniform(l: u32) -> u32;
    pub fn arc4random_buf(buf: *mut ::c_void, size: ::size_t);
    pub fn labs(i: c_long) -> c_long;
    pub fn mkostemp(template: *mut c_char, flags: ::c_int) -> ::c_int;
    pub fn mkostemps(template: *mut c_char, suffixlen: ::c_int, flags: ::c_int) -> ::c_int;
    pub fn mkstemps(template: *mut c_char, suffixlen: ::c_int) -> ::c_int;
    pub fn rand() -> ::c_int;
    pub fn reallocarray(ptr: *mut ::c_void, nmemb: ::size_t, size: ::size_t) -> *mut ::c_void;
    pub fn reallocf(ptr: *mut ::c_void, size: ::size_t) -> *mut ::c_void;
    pub fn srand(seed: ::c_uint);
    pub fn drand48() -> ::c_double;
    pub fn erand48(xseed: *mut ::c_ushort) -> ::c_double;
    pub fn jrand48(xseed: *mut ::c_ushort) -> c_long;
    pub fn lcong48(p: *mut ::c_ushort);
    pub fn lrand48() -> c_long;
    pub fn mrand48() -> c_long;
    pub fn nrand48(xseed: *mut ::c_ushort) -> c_long;
    pub fn seed48(xseed: *mut ::c_ushort) -> *mut ::c_ushort;
    pub fn srand48(seed: c_long);

    pub fn qsort_r(
        base: *mut ::c_void,
        num: ::size_t,
        size: ::size_t,
        compar: ::Option<
            unsafe extern "C" fn(*const ::c_void, *const ::c_void, *mut ::c_void) -> ::c_int,
        >,
        arg: *mut ::c_void,
    );
}

// sys/mman.h
pub const PROT_NONE: ::c_int = 0;
pub const PROT_READ: ::c_int = 1;
pub const PROT_WRITE: ::c_int = 2;
pub const PROT_EXEC: ::c_int = 4;
pub const MAP_FILE: ::c_int = 0;
pub const MAP_SHARED: ::c_int = 1;
pub const MAP_PRIVATE: ::c_int = 2;
pub const MAP_TYPE: ::c_int = 0xf;
pub const MAP_FIXED: ::c_int = 0x10;
pub const MAP_ANON: ::c_int = 0x20;
pub const MAP_ANONYMOUS: ::c_int = MAP_ANON;
pub const MAP_NORESERVE: ::c_int = 0x4000;
pub const MAP_FAILED: *mut ::c_void = !0 as *mut ::c_void;
pub const MS_ASYNC: ::c_int = 1;
pub const MS_SYNC: ::c_int = 2;
pub const MS_INVALIDATE: ::c_int = 4;
pub const POSIX_MADV_NORMAL: ::c_int = 0;
pub const POSIX_MADV_SEQUENTIAL: ::c_int = 1;
pub const POSIX_MADV_RANDOM: ::c_int = 2;
pub const POSIX_MADV_WILLNEED: ::c_int = 3;
pub const POSIX_MADV_DONTNEED: ::c_int = 4;
pub const MADV_NORMAL: ::c_int = 0;
pub const MADV_SEQUENTIAL: ::c_int = 1;
pub const MADV_RANDOM: ::c_int = 2;
pub const MADV_WILLNEED: ::c_int = 3;
pub const MADV_DONTNEED: ::c_int = 4;

extern "C" {
    pub fn mprotect(addr: *mut ::c_void, len: ::size_t, prot: ::c_int) -> ::c_int;
    pub fn msync(addr: *mut ::c_void, len: ::size_t, flags: ::c_int) -> ::c_int;
    pub fn posix_madvise(addr: *mut ::c_void, len: ::size_t, advice: ::c_int) -> ::c_int;
    pub fn madvise(addr: *mut ::c_void, len: ::size_t, advice: ::c_int) -> ::c_int;
    pub fn shm_open(name: *const c_char, oflag: ::c_int, mode: ::mode_t) -> ::c_int;
    pub fn shm_unlink(name: *const c_char) -> ::c_int;
}

// strings.h
extern "C" {
    pub fn explicit_bzero(s: *mut ::c_void, len: ::size_t);
    pub fn ffs(value: ::c_int) -> ::c_int;
    pub fn ffsl(value: c_long) -> ::c_int;
    pub fn ffsll(value: ::c_longlong) -> ::c_int;
    pub fn fls(value: ::c_int) -> ::c_int;
    pub fn flsl(value: c_long) -> ::c_int;
    pub fn flsll(value: ::c_longlong) -> ::c_int;
    pub fn strcasecmp_l(s1: *const c_char, s2: *const c_char, loc: ::locale_t) -> ::c_int;

    pub fn strncasecmp_l(
        s1: *const c_char,
        s2: *const c_char,
        n: ::size_t,
        loc: ::locale_t,
    ) -> ::c_int;
}

// string.h
extern "C" {
    pub fn timingsafe_bcmp(a: *const ::c_void, b: *const ::c_void, len: ::size_t) -> ::c_int;
    pub fn timingsafe_memcmp(a: *const ::c_void, b: *const ::c_void, len: ::size_t) -> ::c_int;

    pub fn memccpy(
        dest: *mut ::c_void,
        src: *const ::c_void,
        c: ::c_int,
        count: ::size_t,
    ) -> *mut ::c_void;

    pub fn memmem(
        haystack: *const ::c_void,
        haystacklen: ::size_t,
        needle: *const ::c_void,
        needlelen: ::size_t,
    ) -> *mut ::c_void;

    pub fn memrchr(cx: *const ::c_void, c: ::c_int, n: ::size_t) -> *mut ::c_void;
    pub fn strerror_r(errnum: ::c_int, buf: *mut c_char, buflen: ::size_t) -> ::c_int;
    pub fn strsep(string: *mut *mut c_char, delim: *const c_char) -> *mut c_char;

    #[link_name = "__gnu_basename"]
    pub fn basename(path: *const c_char) -> *mut c_char;
}

// sys/un.h
s_no_extra_traits! {
    pub struct sockaddr_un {
        pub sun_family: sa_family_t,
        pub sun_path: [c_char; 108]
    }
}

cfg_if! {
    if #[cfg(feature = "extra_traits")] {
        impl PartialEq for sockaddr_un {
            fn eq(&self, other: &sockaddr_un) -> bool {
                self.sun_family == other.sun_family
                    && self
                    .sun_path
                    .iter()
                    .zip(other.sun_path.iter())
                    .all(|(a, b)| a == b)
            }
        }

        impl Eq for sockaddr_un {}

        impl ::fmt::Debug for sockaddr_un {
            fn fmt(&self, f: &mut ::fmt::Formatter) -> ::fmt::Result {
                f.debug_struct("sockaddr_un")
                    .field("sun_family", &self.sun_family)
                // FIXME: .field("sun_path", &self.sun_path)
                    .finish()
            }
        }

        impl ::hash::Hash for sockaddr_un {
            fn hash<H: ::hash::Hasher>(&self, state: &mut H) {
                self.sun_family.hash(state);
                self.sun_path.hash(state);
            }
        }
    }
}

// sys/unistd.h
extern "C" {
    pub fn daemon(nochdir: ::c_int, noclose: ::c_int) -> ::c_int;
    pub fn dup3(src: ::c_int, dst: ::c_int, flags: ::c_int) -> ::c_int;
    pub fn eaccess(pathname: *const c_char, mode: ::c_int) -> ::c_int;
    pub fn euidaccess(pathname: *const c_char, mode: ::c_int) -> ::c_int;
    // pub fn execlpe(path: *const c_char, arg0: *const c_char, ...) -> ::c_int;

    pub fn execvpe(
        file: *const c_char,
        argv: *const *const c_char,
        envp: *const *const c_char,
    ) -> ::c_int;

    pub fn faccessat(
        dirfd: ::c_int,
        pathname: *const c_char,
        mode: ::c_int,
        flags: ::c_int,
    ) -> ::c_int;

    pub fn fexecve(
        fd: ::c_int,
        argv: *const *const c_char,
        envp: *const *const c_char,
    ) -> ::c_int;

    pub fn fdatasync(fd: ::c_int) -> ::c_int;
    pub fn getdomainname(name: *mut c_char, len: ::size_t) -> ::c_int;
    pub fn getentropy(buf: *mut ::c_void, buflen: ::size_t) -> ::c_int;
    pub fn gethostid() -> c_long;
    pub fn getpagesize() -> ::c_int;
    pub fn getpeereid(socket: ::c_int, euid: *mut ::uid_t, egid: *mut ::gid_t) -> ::c_int;
}

pub const F_ULOCK: ::c_int = 0;
pub const F_LOCK: ::c_int = 1;
pub const F_TLOCK: ::c_int = 2;
pub const F_TEST: ::c_int = 3;

extern "C" {
     pub fn pthread_atfork(
        prepare: ::Option<unsafe extern "C" fn()>,
        parent: ::Option<unsafe extern "C" fn()>,
        child: ::Option<unsafe extern "C" fn()>,
    ) -> ::c_int;

    pub fn pipe2(fds: *mut ::c_int, flags: ::c_int) -> ::c_int;
    pub fn sbrk(increment: ::intptr_t) -> *mut ::c_void;
    pub fn setgroups(ngroups: ::c_int, ptr: *const ::gid_t) -> ::c_int;
    pub fn sethostname(name: *const c_char, len: ::size_t) -> ::c_int;
    pub fn vhangup() -> ::c_int;
    pub fn getdtablesize() -> ::c_int;
    pub fn sync();
}

pub const F_OK: ::c_int = 0;
pub const R_OK: ::c_int = 4;
pub const W_OK: ::c_int = 2;
pub const X_OK: ::c_int = 1;
pub const SEEK_SET: ::c_int = 0;
pub const SEEK_CUR: ::c_int = 1;
pub const SEEK_END: ::c_int = 2;
pub const STDIN_FILENO: ::c_int = 0;
pub const STDOUT_FILENO: ::c_int = 1;
pub const STDERR_FILENO: ::c_int = 2;
pub const _SC_ARG_MAX: ::c_int = 0;
pub const _SC_CHILD_MAX: ::c_int = 1;
pub const _SC_CLK_TCK: ::c_int = 2;
pub const _SC_NGROUPS_MAX: ::c_int = 3;
pub const _SC_OPEN_MAX: ::c_int = 4;
pub const _SC_JOB_CONTROL: ::c_int = 5;
pub const _SC_SAVED_IDS: ::c_int = 6;
pub const _SC_VERSION: ::c_int = 7;
pub const _SC_PAGESIZE: ::c_int = 8;
pub const _SC_PAGE_SIZE: ::c_int = _SC_PAGESIZE;
pub const _SC_NPROCESSORS_CONF: ::c_int = 9;
pub const _SC_NPROCESSORS_ONLN: ::c_int = 10;
pub const _SC_PHYS_PAGES: ::c_int = 11;
pub const _SC_AVPHYS_PAGES: ::c_int = 12;
pub const _SC_MQ_OPEN_MAX: ::c_int = 13;
pub const _SC_MQ_PRIO_MAX: ::c_int = 14;
pub const _SC_RTSIG_MAX: ::c_int = 15;
pub const _SC_SEM_NSEMS_MAX: ::c_int = 16;
pub const _SC_SEM_VALUE_MAX: ::c_int = 17;
pub const _SC_SIGQUEUE_MAX: ::c_int = 18;
pub const _SC_TIMER_MAX: ::c_int = 19;
pub const _SC_TZNAME_MAX: ::c_int = 20;
pub const _SC_ASYNCHRONOUS_IO: ::c_int = 21;
pub const _SC_FSYNC: ::c_int = 22;
pub const _SC_MAPPED_FILES: ::c_int = 23;
pub const _SC_MEMLOCK: ::c_int = 24;
pub const _SC_MEMLOCK_RANGE: ::c_int = 25;
pub const _SC_MEMORY_PROTECTION: ::c_int = 26;
pub const _SC_MESSAGE_PASSING: ::c_int = 27;
pub const _SC_PRIORITIZED_IO: ::c_int = 28;
pub const _SC_REALTIME_SIGNALS: ::c_int = 29;
pub const _SC_SEMAPHORES: ::c_int = 30;
pub const _SC_SHARED_MEMORY_OBJECTS: ::c_int = 31;
pub const _SC_SYNCHRONIZED_IO: ::c_int = 32;
pub const _SC_TIMERS: ::c_int = 33;
pub const _SC_AIO_LISTIO_MAX: ::c_int = 34;
pub const _SC_AIO_MAX: ::c_int = 35;
pub const _SC_AIO_PRIO_DELTA_MAX: ::c_int = 36;
pub const _SC_DELAYTIMER_MAX: ::c_int = 37;
pub const _SC_THREAD_KEYS_MAX: ::c_int = 38;
pub const _SC_THREAD_STACK_MIN: ::c_int = 39;
pub const _SC_THREAD_THREADS_MAX: ::c_int = 40;
pub const _SC_TTY_NAME_MAX: ::c_int = 41;
pub const _SC_THREADS: ::c_int = 42;
pub const _SC_THREAD_ATTR_STACKADDR: ::c_int = 43;
pub const _SC_THREAD_ATTR_STACKSIZE: ::c_int = 44;
pub const _SC_THREAD_PRIORITY_SCHEDULING: ::c_int = 45;
pub const _SC_THREAD_PRIO_INHERIT: ::c_int = 46;
pub const _SC_THREAD_PRIO_PROTECT: ::c_int = 47;
pub const _SC_THREAD_PRIO_CEILING: ::c_int = _SC_THREAD_PRIO_PROTECT;
pub const _SC_THREAD_PROCESS_SHARED: ::c_int = 48;
pub const _SC_THREAD_SAFE_FUNCTIONS: ::c_int = 49;
pub const _SC_GETGR_R_SIZE_MAX: ::c_int = 50;
pub const _SC_GETPW_R_SIZE_MAX: ::c_int = 51;
pub const _SC_LOGIN_NAME_MAX: ::c_int = 52;
pub const _SC_THREAD_DESTRUCTOR_ITERATIONS: ::c_int = 53;
pub const _SC_ADVISORY_INFO: ::c_int = 54;
pub const _SC_ATEXIT_MAX: ::c_int = 55;
pub const _SC_BARRIERS: ::c_int = 56;
pub const _SC_BC_BASE_MAX: ::c_int = 57;
pub const _SC_BC_DIM_MAX: ::c_int = 58;
pub const _SC_BC_SCALE_MAX: ::c_int = 59;
pub const _SC_BC_STRING_MAX: ::c_int = 60;
pub const _SC_CLOCK_SELECTION: ::c_int = 61;
pub const _SC_COLL_WEIGHTS_MAX: ::c_int = 62;
pub const _SC_CPUTIME: ::c_int = 63;
pub const _SC_EXPR_NEST_MAX: ::c_int = 64;
pub const _SC_HOST_NAME_MAX: ::c_int = 65;
pub const _SC_IOV_MAX: ::c_int = 66;
pub const _SC_IPV6: ::c_int = 67;
pub const _SC_LINE_MAX: ::c_int = 68;
pub const _SC_MONOTONIC_CLOCK: ::c_int = 69;
pub const _SC_RAW_SOCKETS: ::c_int = 70;
pub const _SC_READER_WRITER_LOCKS: ::c_int = 71;
pub const _SC_REGEXP: ::c_int = 72;
pub const _SC_RE_DUP_MAX: ::c_int = 73;
pub const _SC_SHELL: ::c_int = 74;
pub const _SC_SPAWN: ::c_int = 75;
pub const _SC_SPIN_LOCKS: ::c_int = 76;
pub const _SC_SPORADIC_SERVER: ::c_int = 77;
pub const _SC_SS_REPL_MAX: ::c_int = 78;
pub const _SC_SYMLOOP_MAX: ::c_int = 79;
pub const _SC_THREAD_CPUTIME: ::c_int = 80;
pub const _SC_THREAD_SPORADIC_SERVER: ::c_int = 81;
pub const _SC_TIMEOUTS: ::c_int = 82;
pub const _SC_TRACE: ::c_int = 83;
pub const _SC_TRACE_EVENT_FILTER: ::c_int = 84;
pub const _SC_TRACE_EVENT_NAME_MAX: ::c_int = 85;
pub const _SC_TRACE_INHERIT: ::c_int = 86;
pub const _SC_TRACE_LOG: ::c_int = 87;
pub const _SC_TRACE_NAME_MAX: ::c_int = 88;
pub const _SC_TRACE_SYS_MAX: ::c_int = 89;
pub const _SC_TRACE_USER_EVENT_MAX: ::c_int = 90;
pub const _SC_TYPED_MEMORY_OBJECTS: ::c_int = 91;
pub const _SC_V7_ILP32_OFF32: ::c_int = 92;
pub const _SC_V6_ILP32_OFF32: ::c_int = _SC_V7_ILP32_OFF32;
pub const _SC_XBS5_ILP32_OFF32: ::c_int = _SC_V7_ILP32_OFF32;
pub const _SC_V7_ILP32_OFFBIG: ::c_int = 93;
pub const _SC_V6_ILP32_OFFBIG: ::c_int = _SC_V7_ILP32_OFFBIG;
pub const _SC_XBS5_ILP32_OFFBIG: ::c_int = _SC_V7_ILP32_OFFBIG;
pub const _SC_V7_LP64_OFF64: ::c_int = 94;
pub const _SC_V6_LP64_OFF64: ::c_int = _SC_V7_LP64_OFF64;
pub const _SC_XBS5_LP64_OFF64: ::c_int = _SC_V7_LP64_OFF64;
pub const _SC_V7_LPBIG_OFFBIG: ::c_int = 95;
pub const _SC_V6_LPBIG_OFFBIG: ::c_int = _SC_V7_LPBIG_OFFBIG;
pub const _SC_XBS5_LPBIG_OFFBIG: ::c_int = _SC_V7_LPBIG_OFFBIG;
pub const _SC_XOPEN_CRYPT: ::c_int = 96;
pub const _SC_XOPEN_ENH_I18N: ::c_int = 97;
pub const _SC_XOPEN_LEGACY: ::c_int = 98;
pub const _SC_XOPEN_REALTIME: ::c_int = 99;
pub const _SC_STREAM_MAX: ::c_int = 100;
pub const _SC_PRIORITY_SCHEDULING: ::c_int = 101;
pub const _SC_XOPEN_REALTIME_THREADS: ::c_int = 102;
pub const _SC_XOPEN_SHM: ::c_int = 103;
pub const _SC_XOPEN_STREAMS: ::c_int = 104;
pub const _SC_XOPEN_UNIX: ::c_int = 105;
pub const _SC_XOPEN_VERSION: ::c_int = 106;
pub const _SC_2_CHAR_TERM: ::c_int = 107;
pub const _SC_2_C_BIND: ::c_int = 108;
pub const _SC_2_C_DEV: ::c_int = 109;
pub const _SC_2_FORT_DEV: ::c_int = 110;
pub const _SC_2_FORT_RUN: ::c_int = 111;
pub const _SC_2_LOCALEDEF: ::c_int = 112;
pub const _SC_2_PBS: ::c_int = 113;
pub const _SC_2_PBS_ACCOUNTING: ::c_int = 114;
pub const _SC_2_PBS_CHECKPOINT: ::c_int = 115;
pub const _SC_2_PBS_LOCATE: ::c_int = 116;
pub const _SC_2_PBS_MESSAGE: ::c_int = 117;
pub const _SC_2_PBS_TRACK: ::c_int = 118;
pub const _SC_2_SW_DEV: ::c_int = 119;
pub const _SC_2_UPE: ::c_int = 120;
pub const _SC_2_VERSION: ::c_int = 121;
pub const _SC_THREAD_ROBUST_PRIO_INHERIT: ::c_int = 122;
pub const _SC_THREAD_ROBUST_PRIO_PROTECT: ::c_int = 123;
pub const _SC_XOPEN_UUCP: ::c_int = 124;
pub const _SC_LEVEL1_ICACHE_SIZE: ::c_int = 125;
pub const _SC_LEVEL1_ICACHE_ASSOC: ::c_int = 126;
pub const _SC_LEVEL1_ICACHE_LINESIZE: ::c_int = 127;
pub const _SC_LEVEL1_DCACHE_SIZE: ::c_int = 128;
pub const _SC_LEVEL1_DCACHE_ASSOC: ::c_int = 129;
pub const _SC_LEVEL1_DCACHE_LINESIZE: ::c_int = 130;
pub const _SC_LEVEL2_CACHE_SIZE: ::c_int = 131;
pub const _SC_LEVEL2_CACHE_ASSOC: ::c_int = 132;
pub const _SC_LEVEL2_CACHE_LINESIZE: ::c_int = 133;
pub const _SC_LEVEL3_CACHE_SIZE: ::c_int = 134;
pub const _SC_LEVEL3_CACHE_ASSOC: ::c_int = 135;
pub const _SC_LEVEL3_CACHE_LINESIZE: ::c_int = 136;
pub const _SC_LEVEL4_CACHE_SIZE: ::c_int = 137;
pub const _SC_LEVEL4_CACHE_ASSOC: ::c_int = 138;
pub const _SC_LEVEL4_CACHE_LINESIZE: ::c_int = 139;
pub const _PC_LINK_MAX: ::c_int = 0;
pub const _PC_MAX_CANON: ::c_int = 1;
pub const _PC_MAX_INPUT: ::c_int = 2;
pub const _PC_NAME_MAX: ::c_int = 3;
pub const _PC_PATH_MAX: ::c_int = 4;
pub const _PC_PIPE_BUF: ::c_int = 5;
pub const _PC_CHOWN_RESTRICTED: ::c_int = 6;
pub const _PC_NO_TRUNC: ::c_int = 7;
pub const _PC_VDISABLE: ::c_int = 8;
pub const _PC_ASYNC_IO: ::c_int = 9;
pub const _PC_PRIO_IO: ::c_int = 10;
pub const _PC_SYNC_IO: ::c_int = 11;
pub const _PC_FILESIZEBITS: ::c_int = 12;
pub const _PC_2_SYMLINKS: ::c_int = 13;
pub const _PC_SYMLINK_MAX: ::c_int = 14;
pub const _PC_ALLOC_SIZE_MIN: ::c_int = 15;
pub const _PC_REC_INCR_XFER_SIZE: ::c_int = 16;
pub const _PC_REC_MAX_XFER_SIZE: ::c_int = 17;
pub const _PC_REC_MIN_XFER_SIZE: ::c_int = 18;
pub const _PC_REC_XFER_ALIGN: ::c_int = 19;
pub const _PC_TIMESTAMP_RESOLUTION: ::c_int = 20;
pub const _CS_PATH: ::c_int = 0;

// sys/_default_fcntl.h
pub const O_ACCMODE: ::c_int = 0x3;
pub const O_RDONLY: ::c_int = 0;
pub const O_WRONLY: ::c_int = 1;
pub const O_RDWR: ::c_int = 2;
pub const O_APPEND: ::c_int = 0x0008;
pub const O_CREAT: ::c_int = 0x0200;
pub const O_TRUNC: ::c_int = 0x0400;
pub const O_EXCL: ::c_int = 0x0800;
pub const O_SYNC: ::c_int = 0x2000;
pub const O_NONBLOCK: ::c_int = 0x4000;
pub const O_NOCTTY: ::c_int = 0x8000;
pub const O_CLOEXEC: ::c_int = 0x40000;
pub const O_NOFOLLOW: ::c_int = 0x100000;
pub const O_DIRECTORY: ::c_int = 0x200000;
pub const O_EXEC: ::c_int = 0x400000;
pub const O_SEARCH: ::c_int = 0x400000;
pub const O_DIRECT: ::c_int = 0x80000;
pub const O_DSYNC: ::c_int = 0x2000;
pub const O_RSYNC: ::c_int = 0x2000;
pub const O_TMPFILE: ::c_int = 0x800000;
pub const O_NOATIME: ::c_int = 0x1000000;
pub const O_PATH: ::c_int = 0x2000000;
pub const F_DUPFD: ::c_int = 0;
pub const F_GETFD: ::c_int = 1;
pub const F_SETFD: ::c_int = 2;
pub const F_GETFL: ::c_int = 3;
pub const F_SETFL: ::c_int = 4;
pub const F_GETOWN: ::c_int = 5;
pub const F_SETOWN: ::c_int = 6;
pub const F_GETLK: ::c_int = 7;
pub const F_SETLK: ::c_int = 8;
pub const F_SETLKW: ::c_int = 9;
pub const F_RGETLK: ::c_int = 10;
pub const F_RSETLK: ::c_int = 11;
pub const F_CNVT: ::c_int = 12;
pub const F_RSETLKW: ::c_int = 13;
pub const F_DUPFD_CLOEXEC: ::c_int = 14;
pub const F_RDLCK: ::c_int = 1;
pub const F_WRLCK: ::c_int = 2;
pub const F_UNLCK: ::c_int = 3;
pub const AT_FDCWD: ::c_int = -2;
pub const AT_EACCESS: ::c_int = 1;
pub const AT_SYMLINK_NOFOLLOW: ::c_int = 2;
pub const AT_SYMLINK_FOLLOW: ::c_int = 4;
pub const AT_REMOVEDIR: ::c_int = 8;
pub const AT_EMPTY_PATH: ::c_int = 16;
pub const LOCK_SH: ::c_int = 1;
pub const LOCK_EX: ::c_int = 2;
pub const LOCK_NB: ::c_int = 4;
pub const LOCK_UN: ::c_int = 8;

// sys/errno.h
extern "C" {
    pub fn __errno() -> *mut ::c_int;
}

pub const EPERM: ::c_int = 1;
pub const ENOENT: ::c_int = 2;
pub const ESRCH: ::c_int = 3;
pub const EINTR: ::c_int = 4;
pub const EIO: ::c_int = 5;
pub const ENXIO: ::c_int = 6;
pub const E2BIG: ::c_int = 7;
pub const ENOEXEC: ::c_int = 8;
pub const EBADF: ::c_int = 9;
pub const ECHILD: ::c_int = 10;
pub const EAGAIN: ::c_int = 11;
pub const ENOMEM: ::c_int = 12;
pub const EACCES: ::c_int = 13;
pub const EFAULT: ::c_int = 14;
pub const ENOTBLK: ::c_int = 15;
pub const EBUSY: ::c_int = 16;
pub const EEXIST: ::c_int = 17;
pub const EXDEV: ::c_int = 18;
pub const ENODEV: ::c_int = 19;
pub const ENOTDIR: ::c_int = 20;
pub const EISDIR: ::c_int = 21;
pub const EINVAL: ::c_int = 22;
pub const ENFILE: ::c_int = 23;
pub const EMFILE: ::c_int = 24;
pub const ENOTTY: ::c_int = 25;
pub const ETXTBSY: ::c_int = 26;
pub const EFBIG: ::c_int = 27;
pub const ENOSPC: ::c_int = 28;
pub const ESPIPE: ::c_int = 29;
pub const EROFS: ::c_int = 30;
pub const EMLINK: ::c_int = 31;
pub const EPIPE: ::c_int = 32;
pub const EDOM: ::c_int = 33;
pub const ERANGE: ::c_int = 34;
pub const ENOMSG: ::c_int = 35;
pub const EIDRM: ::c_int = 36;
pub const ECHRNG: ::c_int = 37;
pub const EL2NSYNC: ::c_int = 38;
pub const EL3HLT: ::c_int = 39;
pub const EL3RST: ::c_int = 40;
pub const ELNRNG: ::c_int = 41;
pub const EUNATCH: ::c_int = 42;
pub const ENOCSI: ::c_int = 43;
pub const EL2HLT: ::c_int = 44;
pub const EDEADLK: ::c_int = 45;
pub const ENOLCK: ::c_int = 46;
pub const EBADE: ::c_int = 50;
pub const EBADR: ::c_int = 51;
pub const EXFULL: ::c_int = 52;
pub const ENOANO: ::c_int = 53;
pub const EBADRQC: ::c_int = 54;
pub const EBADSLT: ::c_int = 55;
pub const EDEADLOCK: ::c_int = 56;
pub const EBFONT: ::c_int = 57;
pub const ENOSTR: ::c_int = 60;
pub const ENODATA: ::c_int = 61;
pub const ETIME: ::c_int = 62;
pub const ENOSR: ::c_int = 63;
pub const ENONET: ::c_int = 64;
pub const ENOPKG: ::c_int = 65;
pub const EREMOTE: ::c_int = 66;
pub const ENOLINK: ::c_int = 67;
pub const EADV: ::c_int = 68;
pub const ESRMNT: ::c_int = 69;
pub const ECOMM: ::c_int = 70;
pub const EPROTO: ::c_int = 71;
pub const EMULTIHOP: ::c_int = 74;
pub const EDOTDOT: ::c_int = 76;
pub const EBADMSG: ::c_int = 77;
pub const EFTYPE: ::c_int = 79;
pub const ENOTUNIQ: ::c_int = 80;
pub const EBADFD: ::c_int = 81;
pub const EREMCHG: ::c_int = 82;
pub const ELIBACC: ::c_int = 83;
pub const ELIBBAD: ::c_int = 84;
pub const ELIBSCN: ::c_int = 85;
pub const ELIBMAX: ::c_int = 86;
pub const ELIBEXEC: ::c_int = 87;
pub const ENOSYS: ::c_int = 88;
pub const ENOTEMPTY: ::c_int = 90;
pub const ENAMETOOLONG: ::c_int = 91;
pub const ELOOP: ::c_int = 92;
pub const EOPNOTSUPP: ::c_int = 95;
pub const EPFNOSUPPORT: ::c_int = 96;
pub const ECONNRESET: ::c_int = 104;
pub const ENOBUFS: ::c_int = 105;
pub const EAFNOSUPPORT: ::c_int = 106;
pub const EPROTOTYPE: ::c_int = 107;
pub const ENOTSOCK: ::c_int = 108;
pub const ENOPROTOOPT: ::c_int = 109;
pub const ESHUTDOWN: ::c_int = 110;
pub const ECONNREFUSED: ::c_int = 111;
pub const EADDRINUSE: ::c_int = 112;
pub const ECONNABORTED: ::c_int = 113;
pub const ENETUNREACH: ::c_int = 114;
pub const ENETDOWN: ::c_int = 115;
pub const ETIMEDOUT: ::c_int = 116;
pub const EHOSTDOWN: ::c_int = 117;
pub const EHOSTUNREACH: ::c_int = 118;
pub const EINPROGRESS: ::c_int = 119;
pub const EALREADY: ::c_int = 120;
pub const EDESTADDRREQ: ::c_int = 121;
pub const EMSGSIZE: ::c_int = 122;
pub const EPROTONOSUPPORT: ::c_int = 123;
pub const ESOCKTNOSUPPORT: ::c_int = 124;
pub const EADDRNOTAVAIL: ::c_int = 125;
pub const ENETRESET: ::c_int = 126;
pub const EISCONN: ::c_int = 127;
pub const ENOTCONN: ::c_int = 128;
pub const ETOOMANYREFS: ::c_int = 129;
pub const EPROCLIM: ::c_int = 130;
pub const EUSERS: ::c_int = 131;
pub const EDQUOT: ::c_int = 132;
pub const ESTALE: ::c_int = 133;
pub const ENOTSUP: ::c_int = 134;
pub const ENOMEDIUM: ::c_int = 135;
pub const EILSEQ: ::c_int = 138;
pub const EOVERFLOW: ::c_int = 139;
pub const ECANCELED: ::c_int = 140;
pub const ENOTRECOVERABLE: ::c_int = 141;
pub const EOWNERDEAD: ::c_int = 142;
pub const ESTRPIPE: ::c_int = 143;
pub const EWOULDBLOCK: ::c_int = EAGAIN; /* Operation would block */

// sys/sched.h
pub const SCHED_OTHER: ::c_int = 3;
pub const SCHED_FIFO: ::c_int = 1;
pub const SCHED_RR: ::c_int = 2;

s! {
    pub struct sched_param {
        pub sched_priority: ::c_int,
    }
}

// sched.h
extern "C" {
    pub fn sched_setparam(pid: ::pid_t, param: *const sched_param) -> ::c_int;
    pub fn sched_getparam(pid: ::pid_t, param: *mut sched_param) -> ::c_int;

    pub fn sched_setscheduler(
        pid: ::pid_t,
        policy: ::c_int,
        param: *const sched_param,
    ) -> ::c_int;

    pub fn sched_getscheduler(pid: ::pid_t) -> ::c_int;
    pub fn sched_get_priority_max(policy: ::c_int) -> ::c_int;
    pub fn sched_get_priority_min(policy: ::c_int) -> ::c_int;
    pub fn sched_rr_get_interval(pid: ::pid_t, t: *mut ::timespec) -> ::c_int;
    pub fn sched_getcpu() -> ::c_int;
    pub fn sched_getaffinity(pid: ::pid_t, cpusetsize: ::size_t, mask: *mut cpu_set_t) -> ::c_int;

    pub fn sched_setaffinity(
        pid: ::pid_t,
        cpusetsize: ::size_t,
        cpuset: *const cpu_set_t,
    ) -> ::c_int;
}

// pthread.h
pub const PTHREAD_COND_INITIALIZER: pthread_cond_t = 21 as *mut _;
pub const PTHREAD_CREATE_DETACHED: ::c_int = 1;
pub const PTHREAD_CREATE_JOINABLE: ::c_int = 0;
pub const PTHREAD_MUTEX_RECURSIVE: ::c_int = 0;
pub const PTHREAD_MUTEX_ERRORCHECK: ::c_int = 1;
pub const PTHREAD_MUTEX_NORMAL: ::c_int = 2;
pub const PTHREAD_MUTEX_DEFAULT: ::c_int = PTHREAD_MUTEX_NORMAL;
pub const PTHREAD_RECURSIVE_MUTEX_INITIALIZER_NP: pthread_mutex_t = 18 as *mut _;
pub const PTHREAD_ERRORCHECK_MUTEX_INITIALIZER_NP: pthread_mutex_t = 20 as *mut _;
pub const PTHREAD_MUTEX_INITIALIZER: pthread_mutex_t = 19 as *mut _;
pub const PTHREAD_PROCESS_SHARED: ::c_int = 1;
pub const PTHREAD_PROCESS_PRIVATE: ::c_int = 0;
pub const PTHREAD_RWLOCK_INITIALIZER: pthread_rwlock_t = 22 as *mut _;

extern "C" {
    pub fn pthread_attr_getguardsize(
        attr: *const pthread_attr_t,
        guardsize: *mut ::size_t,
    ) -> ::c_int;

    pub fn pthread_attr_getschedparam(
        attr: *const pthread_attr_t,
        param: *mut sched_param,
    ) -> ::c_int;

    pub fn pthread_attr_setschedparam(
        attr: *mut pthread_attr_t,
        param: *const sched_param,
    ) -> ::c_int;

    pub fn pthread_attr_getstack(
        attr: *const pthread_attr_t,
        stackaddr: *mut *mut ::c_void,
        stacksize: *mut ::size_t,
    ) -> ::c_int;

    pub fn pthread_cancel(thread: pthread_t) -> ::c_int;

    pub fn pthread_condattr_getclock(
        attr: *const pthread_condattr_t,
        clock_id: *mut clockid_t,
    ) -> ::c_int;

    pub fn pthread_condattr_getpshared(
        attr: *const pthread_condattr_t,
        pshared: *mut ::c_int,
    ) -> ::c_int;

    pub fn pthread_condattr_setclock(
        attr: *mut pthread_condattr_t,
        clock_id: clockid_t,
    ) -> ::c_int;

    pub fn pthread_condattr_setpshared(attr: *mut pthread_condattr_t, pshared: ::c_int) -> ::c_int;
    pub fn pthread_barrierattr_init(attr: *mut pthread_barrierattr_t) -> ::c_int;

    pub fn pthread_barrierattr_setpshared(
        attr: *mut pthread_barrierattr_t,
        shared: ::c_int,
    ) -> ::c_int;

    pub fn pthread_barrierattr_getpshared(
        attr: *const pthread_barrierattr_t,
        shared: *mut ::c_int,
    ) -> ::c_int;

    pub fn pthread_barrierattr_destroy(attr: *mut pthread_barrierattr_t) -> ::c_int;

    pub fn pthread_barrier_init(
        barrier: *mut pthread_barrier_t,
        attr: *const pthread_barrierattr_t,
        count: ::c_uint,
    ) -> ::c_int;

    pub fn pthread_barrier_destroy(barrier: *mut pthread_barrier_t) -> ::c_int;
    pub fn pthread_barrier_wait(barrier: *mut pthread_barrier_t) -> ::c_int;

    pub fn pthread_create(
        native: *mut pthread_t,
        attr: *const pthread_attr_t,
        f: extern "C" fn(*mut ::c_void) -> *mut ::c_void,
        value: *mut ::c_void,
    ) -> ::c_int;

    pub fn pthread_getcpuclockid(thread: pthread_t, clk_id: *mut clockid_t) -> ::c_int;

    pub fn pthread_getschedparam(
        native: pthread_t,
        policy: *mut ::c_int,
        param: *mut sched_param,
    ) -> ::c_int;

    pub fn pthread_mutex_timedlock(
        lock: *mut pthread_mutex_t,
        abstime: *const ::timespec,
    ) -> ::c_int;

    pub fn pthread_mutexattr_getprotocol(
        attr: *const pthread_mutexattr_t,
        protocol: *mut ::c_int,
    ) -> ::c_int;

    pub fn pthread_mutexattr_getpshared(
        attr: *const pthread_mutexattr_t,
        pshared: *mut ::c_int,
    ) -> ::c_int;

    pub fn pthread_mutexattr_setprotocol(
        attr: *mut pthread_mutexattr_t,
        protocol: ::c_int,
    ) -> ::c_int;

    pub fn pthread_mutexattr_setpshared(
        attr: *mut pthread_mutexattr_t,
        pshared: ::c_int,
    ) -> ::c_int;

    pub fn pthread_spin_destroy(lock: *mut pthread_spinlock_t) -> ::c_int;
    pub fn pthread_spin_init(lock: *mut pthread_spinlock_t, pshared: ::c_int) -> ::c_int;
    pub fn pthread_spin_lock(lock: *mut pthread_spinlock_t) -> ::c_int;
    pub fn pthread_spin_trylock(lock: *mut pthread_spinlock_t) -> ::c_int;
    pub fn pthread_spin_unlock(lock: *mut pthread_spinlock_t) -> ::c_int;

    pub fn pthread_rwlockattr_getpshared(
        attr: *const pthread_rwlockattr_t,
        val: *mut ::c_int,
    ) -> ::c_int;

    pub fn pthread_rwlockattr_setpshared(attr: *mut pthread_rwlockattr_t, val: ::c_int) -> ::c_int;

    pub fn pthread_setschedparam(
        native: pthread_t,
        policy: ::c_int,
        param: *const sched_param,
    ) -> ::c_int;

    pub fn pthread_setschedprio(native: pthread_t, priority: ::c_int) -> ::c_int;

    pub fn pthread_getaffinity_np(
        thread: pthread_t,
        cpusetsize: ::size_t,
        cpuset: *mut cpu_set_t,
    ) -> ::c_int;

    pub fn pthread_getattr_np(native: pthread_t, attr: *mut pthread_attr_t) -> ::c_int;
    pub fn pthread_getname_np(thread: pthread_t, name: *mut c_char, len: ::size_t) -> ::c_int;

    pub fn pthread_setaffinity_np(
        thread: pthread_t,
        cpusetsize: ::size_t,
        cpuset: *const cpu_set_t,
    ) -> ::c_int;

    pub fn pthread_setname_np(thread: pthread_t, name: *const c_char) -> ::c_int;
    pub fn pthread_sigqueue(thread: *mut pthread_t, sig: ::c_int, value: ::sigval) -> ::c_int;
}

// bits/endian.h
pub const LITTLE_ENDIAN: ::c_int = 1234;
pub const BIG_ENDIAN: ::c_int = 4321;

// netinet/tcp.h
pub const TCP_NODELAY: ::c_int = 1;
pub const TCP_KEEPIDLE: ::c_int = 3;
pub const TCP_MAXSEG: ::c_int = 4;
pub const TCP_QUICKACK: ::c_int = 12;
pub const TCP_USER_TIMEOUT: ::c_int = 14;
pub const TCP_FASTOPEN: ::c_int = 15;
pub const TCP_KEEPCNT: ::c_int = 16;
pub const TCP_KEEPINTVL: ::c_int = 17;

// sys/ioctl.h
pub const WINDOWS_POST: Ioctl = 0;
pub const WINDOWS_SEND: Ioctl = 1;
pub const WINDOWS_HWND: Ioctl = 2;

extern "C" {
    pub fn ioctl(fd: ::c_int, request: ::c_int, ...) -> ::c_int;
}

// sys/random.h
extern "C" {
    pub fn getrandom(buf: *mut ::c_void, buflen: ::size_t, flags: ::c_uint) -> ::ssize_t;
}

// sys/mount.h
pub const MOUNT_TEXT: ::c_uint = 0x01;
pub const MOUNT_SYSTEM: ::c_uint = 0x08;
pub const MOUNT_EXEC: ::c_uint = 0x10;
pub const MOUNT_CYGDRIVE: ::c_uint = 0x20;
pub const MOUNT_CYGWIN_EXEC: ::c_uint = 0x40;
pub const MOUNT_SPARSE: ::c_uint = 0x80;
pub const MOUNT_NOTEXEC: ::c_uint = 0x100;
pub const MOUNT_DEVFS: ::c_uint = 0x200;
pub const MOUNT_PROC: ::c_uint = 0x400;
pub const MOUNT_RO: ::c_uint = 0x1000;
pub const MOUNT_NOACL: ::c_uint = 0x2000;
pub const MOUNT_NOPOSIX: ::c_uint = 0x4000;
pub const MOUNT_OVERRIDE: ::c_uint = 0x8000;
pub const MOUNT_IMMUTABLE: ::c_uint = 0x10000;
pub const MOUNT_AUTOMATIC: ::c_uint = 0x20000;
pub const MOUNT_DOS: ::c_uint = 0x40000;
pub const MOUNT_IHASH: ::c_uint = 0x80000;
pub const MOUNT_BIND: ::c_uint = 0x100000;
pub const MOUNT_USER_TEMP: ::c_uint = 0x200000;
pub const MOUNT_DONT_USE: ::c_uint = 0x80000000;

extern "C" {
    pub fn mount(
        src: *const c_char,
        target: *const c_char,
        flags: ::c_uint,
    ) -> ::c_int;

    pub fn umount(target: *const c_char) -> ::c_int;
    pub fn cygwin_umount(target: *const c_char, flags: ::c_uint) -> ::c_int;
}

// sys/features.h
pub const _POSIX_VDISABLE: ::cc_t = 0;

// dirent.h
extern "C" {
    pub fn dirfd(dirp: *mut ::DIR) -> ::c_int;
    pub fn seekdir(dirp: *mut ::DIR, loc: ::c_long);
    pub fn telldir(dirp: *mut ::DIR) -> ::c_long;
}

// machine/types.h
pub type vm_offset_t = c_ulong;

s! {
    pub struct flock {
        pub l_type: ::c_short,
        pub l_whence: ::c_short,
        pub l_start: off_t,
        pub l_len: off_t,
        pub l_pid: ::pid_t,
    }
}

// sys/utsname.h
s_no_extra_traits! {
    pub struct utsname {
        pub sysname: [::c_char; 65],
        pub nodename: [::c_char; 65],
        pub release: [::c_char; 65],
        pub version: [::c_char; 65],
        pub machine: [::c_char; 65],
        pub domainname: [::c_char; 65],
    }
}


cfg_if! {
    if #[cfg(feature = "extra_traits")] {
        impl PartialEq for utsname {
            fn eq(&self, other: &utsname) -> bool {
                self.sysname
                    .iter()
                    .zip(other.sysname.iter())
                    .all(|(a,b)| a == b)
                    && self
                    .nodename
                    .iter()
                    .zip(other.nodename.iter())
                    .all(|(a,b)| a == b)
                    && self
                    .release
                    .iter()
                    .zip(other.release.iter())
                    .all(|(a,b)| a == b)
                    && self
                    .version
                    .iter()
                    .zip(other.version.iter())
                    .all(|(a,b)| a == b)
                    && self
                    .machine
                    .iter()
                    .zip(other.machine.iter())
                    .all(|(a,b)| a == b)
                    && self
                    .domainname
                    .iter()
                    .zip(other.domainname.iter())
                    .all(|(a,b)| a == b)
            }
        }

        impl Eq for utsname {}

        impl ::fmt::Debug for utsname {
            fn fmt(&self, f: &mut ::fmt::Formatter) -> ::fmt::Result {
                f.debug_struct("utsname")
                // FIXME: .field("sysname", &self.sysname)
                // FIXME: .field("nodename", &self.nodename)
                // FIXME: .field("release", &self.release)
                // FIXME: .field("version", &self.version)
                // FIXME: .field("machine", &self.machine)
                // FIXME: .field("domainname", &self.domainname)
                    .finish()
            }
        }

        impl ::hash::Hash for utsname {
            fn hash<H: ::hash::Hasher>(&self, state: &mut H) {
                self.sysname.hash(state);
                self.nodename.hash(state);
                self.release.hash(state);
                self.version.hash(state);
                self.machine.hash(state);
                self.domainname.hash(state);
            }
        }
    }
}

extern "C" {
    pub fn uname(buf: *mut utsname) -> ::c_int;
}
