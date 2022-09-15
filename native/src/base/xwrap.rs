use std::ffi::CStr;
use std::os::unix::io::RawFd;

use libc::{
    c_char, c_uint, c_ulong, c_void, dev_t, mode_t, sockaddr, socklen_t, ssize_t, SYS_dup3,
};

use crate::{errno, error, perror, ptr_to_str};

#[no_mangle]
pub extern "C" fn xfopen(path: *const c_char, mode: *const c_char) -> *mut libc::FILE {
    unsafe {
        let fp = libc::fopen(path, mode);
        if fp.is_null() {
            perror!("fopen {}", ptr_to_str(path));
        }
        return fp;
    }
}

#[no_mangle]
pub extern "C" fn xfdopen(fd: RawFd, mode: *const c_char) -> *mut libc::FILE {
    unsafe {
        let fp = libc::fdopen(fd, mode);
        if fp.is_null() {
            perror!("fdopen");
        }
        return fp;
    }
}

#[no_mangle]
pub extern "C" fn xopen(path: *const c_char, flags: i32, mode: mode_t) -> RawFd {
    unsafe {
        let r = libc::open(path, flags, mode as c_uint);
        if r < 0 {
            perror!("open {}", ptr_to_str(path));
        }
        return r;
    }
}

#[no_mangle]
pub extern "C" fn xopenat(dirfd: RawFd, path: *const c_char, flags: i32, mode: mode_t) -> RawFd {
    unsafe {
        let r = libc::openat(dirfd, path, flags, mode as c_uint);
        if r < 0 {
            perror!("openat {}", ptr_to_str(path));
        }
        return r;
    }
}

#[macro_export]
macro_rules! xopen {
    ($path:expr, $flags:expr) => {
        xopen($path, $flags, 0)
    };
    ($path:expr, $flags:expr, $mode:expr) => {
        xopen($path, $flags, $mode)
    };
}

// Fully write data slice
pub fn xwrite(fd: RawFd, data: &[u8]) -> isize {
    unsafe {
        let mut write_sz: usize = 0;
        let mut r: ssize_t;
        let mut remain: &[u8] = data;
        loop {
            r = libc::write(fd, remain.as_ptr().cast(), remain.len());
            if r < 0 {
                if *errno() == libc::EINTR {
                    continue;
                }
                perror!("write");
                return r as isize;
            }
            let r = r as usize;
            write_sz += r;
            remain = &remain[r..];
            if r == 0 || remain.len() == 0 {
                break;
            }
        }
        if remain.len() != 0 {
            error!("write ({} != {})", write_sz, data.len())
        }
        return write_sz as isize;
    }
}

pub fn xread(fd: RawFd, data: &mut [u8]) -> isize {
    unsafe {
        let r = libc::read(fd, data.as_mut_ptr().cast(), data.len());
        if r < 0 {
            perror!("read");
        }
        return r;
    }
}

// Fully read size of data slice
pub fn xxread(fd: RawFd, data: &mut [u8]) -> isize {
    unsafe {
        let mut read_sz: usize = 0;
        let mut r: ssize_t;
        let mut remain: &mut [u8] = data;
        loop {
            r = libc::read(fd, remain.as_mut_ptr().cast(), remain.len());
            if r < 0 {
                if *errno() == libc::EINTR {
                    continue;
                }
                perror!("read");
                return r as isize;
            }
            let r = r as usize;
            read_sz += r;
            remain = &mut remain[r..];
            if r == 0 || remain.len() == 0 {
                break;
            }
        }
        if remain.len() != 0 {
            error!("read ({} != {})", read_sz, data.len())
        }
        return read_sz as isize;
    }
}

#[no_mangle]
pub extern "C" fn xlseek64(fd: RawFd, offset: i64, whence: i32) -> i64 {
    unsafe {
        let r = libc::lseek64(fd, offset, whence);
        if r < 0 {
            perror!("lseek64");
        }
        return r;
    }
}

pub fn xpipe2(fds: &mut [i32; 2], flags: i32) -> i32 {
    unsafe {
        let r = libc::pipe2(fds.as_mut_ptr(), flags);
        if r < 0 {
            perror!("pipe2");
        }
        return r;
    }
}

#[no_mangle]
pub extern "C" fn xsetns(fd: RawFd, nstype: i32) -> i32 {
    unsafe {
        let r = libc::setns(fd, nstype);
        if r < 0 {
            perror!("setns");
        }
        return r;
    }
}

#[no_mangle]
pub extern "C" fn xunshare(flags: i32) -> i32 {
    unsafe {
        let r = libc::unshare(flags);
        if r < 0 {
            perror!("unshare");
        }
        return r;
    }
}

#[no_mangle]
pub extern "C" fn xopendir(path: *const c_char) -> *mut libc::DIR {
    unsafe {
        let dp = libc::opendir(path);
        if dp.is_null() {
            perror!("opendir {}", ptr_to_str(path));
        }
        return dp;
    }
}

#[no_mangle]
pub extern "C" fn xfdopendir(fd: RawFd) -> *mut libc::DIR {
    unsafe {
        let dp = libc::fdopendir(fd);
        if dp.is_null() {
            perror!("fdopendir");
        }
        return dp;
    }
}

#[no_mangle]
pub extern "C" fn xsetsid() -> i32 {
    unsafe {
        let r = libc::setsid();
        if r < 0 {
            perror!("setsid");
        }
        return r;
    }
}

#[no_mangle]
pub extern "C" fn xsocket(domain: i32, ty: i32, protocol: i32) -> RawFd {
    unsafe {
        let fd = libc::socket(domain, ty, protocol);
        if fd < 0 {
            perror!("socket");
        }
        return fd;
    }
}

#[no_mangle]
pub extern "C" fn xbind(socket: i32, address: *const sockaddr, len: socklen_t) -> i32 {
    unsafe {
        let r = libc::bind(socket, address, len);
        if r < 0 {
            perror!("bind");
        }
        return r;
    }
}

#[no_mangle]
pub extern "C" fn xlisten(socket: i32, backlog: i32) -> i32 {
    unsafe {
        let r = libc::listen(socket, backlog);
        if r < 0 {
            perror!("listen");
        }
        return r;
    }
}

#[no_mangle]
pub extern "C" fn xaccept4(
    sockfd: RawFd,
    addr: *mut sockaddr,
    len: *mut socklen_t,
    flg: i32,
) -> RawFd {
    unsafe {
        let fd = libc::accept4(sockfd, addr, len, flg);
        if fd < 0 {
            perror!("accept4");
        }
        return fd;
    }
}

#[no_mangle]
pub extern "C" fn xsendmsg(fd: RawFd, msg: *const libc::msghdr, flags: i32) -> ssize_t {
    unsafe {
        let r = libc::sendmsg(fd, msg, flags);
        if r < 0 {
            perror!("sendmsg");
        }
        return r;
    }
}

#[no_mangle]
pub extern "C" fn xrecvmsg(fd: RawFd, msg: *mut libc::msghdr, flags: i32) -> ssize_t {
    unsafe {
        let r = libc::recvmsg(fd, msg, flags);
        if r < 0 {
            perror!("recvmsg");
        }
        return r;
    }
}

#[no_mangle]
pub extern "C" fn xaccess(path: *const c_char, mode: i32) -> i32 {
    unsafe {
        let r = libc::access(path, mode);
        if r < 0 {
            perror!("access {}", ptr_to_str(path));
        }
        return r;
    }
}

#[no_mangle]
pub extern "C" fn xfaccessat(dirfd: RawFd, path: *const c_char, mode: i32, flags: i32) -> i32 {
    unsafe {
        #[allow(unused_mut)]
        let mut r = libc::faccessat(dirfd, path, mode, flags);
        if r < 0 {
            perror!("faccessat {}", ptr_to_str(path));
        }
        #[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
        if r > 0 && *errno() == 0 {
            r = 0
        }
        return r;
    }
}

#[no_mangle]
pub extern "C" fn xstat(path: *const c_char, buf: *mut libc::stat) -> i32 {
    unsafe {
        let r = libc::stat(path, buf);
        if r < 0 {
            perror!("stat {}", ptr_to_str(path));
        }
        return r;
    }
}

#[no_mangle]
pub extern "C" fn xlstat(path: *const c_char, buf: *mut libc::stat) -> i32 {
    unsafe {
        let r = libc::lstat(path, buf);
        if r < 0 {
            perror!("lstat {}", ptr_to_str(path));
        }
        return r;
    }
}

#[no_mangle]
pub extern "C" fn xfstat(fd: RawFd, buf: *mut libc::stat) -> i32 {
    unsafe {
        let r = libc::fstat(fd, buf);
        if r < 0 {
            perror!("fstat");
        }
        return r;
    }
}

#[no_mangle]
pub extern "C" fn xfstatat(
    dirfd: RawFd,
    path: *const c_char,
    buf: *mut libc::stat,
    flags: i32,
) -> i32 {
    unsafe {
        let r = libc::fstatat(dirfd, path, buf, flags);
        if r < 0 {
            perror!("fstatat {}", ptr_to_str(path));
        }
        return r;
    }
}

#[no_mangle]
pub extern "C" fn xdup(oldfd: RawFd) -> RawFd {
    unsafe {
        let fd = libc::dup(oldfd);
        if fd < 0 {
            perror!("dup");
        }
        return fd;
    }
}

#[no_mangle]
pub extern "C" fn xdup2(oldfd: RawFd, newfd: RawFd) -> RawFd {
    unsafe {
        let fd = libc::dup2(oldfd, newfd);
        if fd < 0 {
            perror!("dup2");
        }
        return fd;
    }
}

#[no_mangle]
pub extern "C" fn xdup3(oldfd: RawFd, newfd: RawFd, flags: i32) -> RawFd {
    unsafe {
        let fd = libc::syscall(SYS_dup3, oldfd, newfd, flags) as RawFd;
        if fd < 0 {
            perror!("dup3");
        }
        return fd;
    }
}

pub fn xreadlink(path: &CStr, data: &mut [u8]) -> isize {
    mod e {
        extern "C" {
            pub fn xreadlink(path: *const u8, buf: *mut u8, bufsz: usize) -> isize;
        }
    }
    unsafe { e::xreadlink(path.as_ptr().cast(), data.as_mut_ptr(), data.len()) }
}

pub fn xreadlinkat(dirfd: RawFd, path: &CStr, data: &mut [u8]) -> isize {
    mod e {
        extern "C" {
            pub fn xreadlinkat(dirfd: i32, path: *const u8, buf: *mut u8, bufsz: usize) -> isize;
        }
    }
    unsafe { e::xreadlinkat(dirfd, path.as_ptr().cast(), data.as_mut_ptr(), data.len()) }
}

#[no_mangle]
pub extern "C" fn xsymlink(target: *const c_char, linkpath: *const c_char) -> i32 {
    unsafe {
        let r = libc::symlink(target, linkpath);
        if r < 0 {
            perror!("symlink {} -> {}", ptr_to_str(target), ptr_to_str(linkpath));
        }
        return r;
    }
}

#[no_mangle]
pub extern "C" fn xsymlinkat(target: *const c_char, dirfd: RawFd, linkpath: *const c_char) -> i32 {
    unsafe {
        let r = libc::symlinkat(target, dirfd, linkpath);
        if r < 0 {
            perror!(
                "symlinkat {} -> {}",
                ptr_to_str(target),
                ptr_to_str(linkpath)
            );
        }
        return r;
    }
}

#[no_mangle]
pub extern "C" fn xlinkat(
    olddirfd: RawFd,
    target: *const c_char,
    newdirfd: RawFd,
    linkpath: *const c_char,
    flags: i32,
) -> i32 {
    unsafe {
        let r = libc::linkat(olddirfd, target, newdirfd, linkpath, flags);
        if r < 0 {
            perror!("linkat {} -> {}", ptr_to_str(target), ptr_to_str(linkpath));
        }
        return r;
    }
}

#[no_mangle]
pub extern "C" fn xmount(
    src: *const c_char,
    target: *const c_char,
    fstype: *const c_char,
    flags: c_ulong,
    data: *const c_void,
) -> i32 {
    unsafe {
        let r = libc::mount(src, target, fstype, flags, data);
        if r < 0 {
            perror!("mount {} -> {}", ptr_to_str(src), ptr_to_str(target));
        }
        return r;
    }
}

#[no_mangle]
pub extern "C" fn xumount(target: *const c_char) -> i32 {
    unsafe {
        let r = libc::umount(target);
        if r < 0 {
            perror!("umount {}", ptr_to_str(target));
        }
        return r;
    }
}

#[no_mangle]
pub extern "C" fn xumount2(target: *const c_char, flags: i32) -> i32 {
    unsafe {
        let r = libc::umount2(target, flags);
        if r < 0 {
            perror!("umount2 {}", ptr_to_str(target));
        }
        return r;
    }
}

#[no_mangle]
pub extern "C" fn xrename(oldname: *const c_char, newname: *const c_char) -> i32 {
    unsafe {
        let r = libc::rename(oldname, newname);
        if r < 0 {
            perror!("rename {} -> {}", ptr_to_str(oldname), ptr_to_str(newname));
        }
        return r;
    }
}

#[no_mangle]
pub extern "C" fn xmkdir(path: *const c_char, mode: mode_t) -> i32 {
    unsafe {
        let r = libc::mkdir(path, mode);
        if r < 0 && *errno() != libc::EEXIST {
            perror!("mkdir {}", ptr_to_str(path));
        }
        return r;
    }
}

#[no_mangle]
pub extern "C" fn xmkdirat(dirfd: RawFd, path: *const c_char, mode: mode_t) -> i32 {
    unsafe {
        let r = libc::mkdirat(dirfd, path, mode);
        if r < 0 && *errno() != libc::EEXIST {
            perror!("mkdirat {}", ptr_to_str(path));
        }
        return r;
    }
}

#[no_mangle]
pub extern "C" fn xfork() -> i32 {
    unsafe {
        let r = libc::fork();
        if r < 0 {
            perror!("fork");
        }
        return r;
    }
}

#[no_mangle]
pub extern "C" fn xmknod(pathname: *const c_char, mode: mode_t, dev: dev_t) -> i32 {
    unsafe {
        let r = libc::mknod(pathname, mode, dev);
        if r < 0 {
            perror!("mknod {}", ptr_to_str(pathname));
        }
        return r;
    }
}
