use crate::errors::*;
use std::cell::UnsafeCell;
use std::fmt;
use std::mem;
use std::ops::{Deref, DerefMut};
use std::thread;
use std::time::Duration;

pub struct Mutex<T> {
    key: i32,
    cell: UnsafeCell<T>,
}

impl<T> Mutex<T> {
    pub fn new(key: i32, data: T) -> Self {
        Self {
            key,
            cell: UnsafeCell::new(data),
        }
    }

    pub fn lock(&self) -> Result<MutexGuard<'_, T>> {
        unsafe {
            if let Some(sem_id) = ffi::sem_init(self.key) {
                for _ in 0..10 {
                    if ffi::sem_try_wait(sem_id) {
                        trace!(
                            "sem mutex lock {} was taken for resource {:#08x}",
                            sem_id,
                            self.key
                        );
                        return Ok(MutexGuard {
                            sem_id,
                            mutex: self,
                        });
                    }
                    thread::sleep(Duration::from_millis(5));
                }
            }
        }
        Err(format!(
            "sem mutex lock: failed to obtain lock for resource {:#08x}",
            self.key
        )
        .into())
    }
}

pub struct MutexGuard<'a, T> {
    sem_id: i32,
    mutex: &'a Mutex<T>,
}

impl<T> Deref for MutexGuard<'_, T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.mutex.cell.get() }
    }
}

impl<T> DerefMut for MutexGuard<'_, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut *self.mutex.cell.get() }
    }
}

impl<T> Drop for MutexGuard<'_, T> {
    fn drop(&mut self) {
        unsafe {
            ffi::sem_post(self.sem_id);
            trace!("sem mutex lock {} released", self.sem_id);
        }
    }
}

#[must_use = "if unused the Mutex will immediately unlock"]
pub struct SemMutex {
    sem_id: i32,
}

impl Drop for SemMutex {
    fn drop(&mut self) {
        unsafe {
            ffi::sem_post(self.sem_id);
            trace!("sem mutex lock {} released", self.sem_id);
        }
    }
}

impl fmt::Display for SemMutex {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "SemMutex({})", self.sem_id)
    }
}

impl SemMutex {
    pub fn lock(key: i32) -> Result<Self> {
        unsafe {
            if let Some(sem_id) = ffi::sem_init(key as i32) {
                for _ in 0..10 {
                    if ffi::sem_try_wait(sem_id) {
                        trace!(
                            "sem mutex lock {} was taken for resource {:#08x}",
                            sem_id,
                            key
                        );
                        return Ok(SemMutex { sem_id });
                    }
                    thread::sleep(Duration::from_millis(5));
                }
            }
        }
        Err(format!(
            "sem mutex lock: failed to obtain lock for resource {:#08x}",
            key
        )
        .into())
    }
}

mod ffi {
    #[inline]
    pub unsafe fn sem_noop(sem_id: i32) -> bool {
        let mut buf = libc::sembuf {
            sem_num: 0,
            sem_op: 0,
            sem_flg: 0,
        };
        libc::semop(sem_id, &mut buf, 1) != -1
    }

    #[inline]
    pub unsafe fn sem_init(key: i32) -> Option<i32> {
        let perm = 0666;
        let flag = libc::IPC_CREAT | libc::IPC_EXCL | perm; // 0o1000 | 0o2000
        let mut sem_id = libc::semget(key, 1, flag); //create sem
        if sem_id >= 0 {
            //init sem
            if libc::semctl(sem_id, 0, 0x10, 1) == -1 {
                return None;
            }
            //notify other process sem created
            if !sem_noop(sem_id) {
                return None;
            }
        } else {
            // existing one
            sem_id = libc::semget(key, 1, perm);
            if sem_id == -1 {
                return None;
            }
        }
        Some(sem_id)
    }

    //wait and block
    #[inline]
    pub unsafe fn sem_wait(sem_id: i32) -> bool {
        let mut buf = libc::sembuf {
            sem_num: 0,
            sem_op: -1,
            sem_flg: 0,
        };
        libc::semop(sem_id, &mut buf, 1) != -1
    }

    // wait no block
    #[inline]
    pub unsafe fn sem_try_wait(sem_id: i32) -> bool {
        let mut buf = libc::sembuf {
            sem_num: 0,
            sem_op: -1,
            sem_flg: libc::IPC_NOWAIT as i16,
        };
        libc::semop(sem_id, &mut buf, 1) != -1
    }
    #[inline]
    pub unsafe fn sem_post(sem_id: i32) -> bool {
        let mut buf = libc::sembuf {
            sem_num: 0,
            sem_op: 1,
            sem_flg: 0,
        };
        libc::semop(sem_id, &mut buf, 1) != -1
    }
}
