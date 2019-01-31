use dox::Option;

extern {
    pub fn pthread_self() -> ::pthread_t;
    #[cfg_attr(all(target_os = "macos", target_arch = "x86"),
               link_name = "pthread_join$UNIX2003")]
    pub fn pthread_join(native: ::pthread_t,
                        value: *mut *mut ::c_void) -> ::c_int;
    pub fn pthread_exit(value: *mut ::c_void);
    pub fn pthread_attr_init(attr: *mut ::pthread_attr_t) -> ::c_int;
    pub fn pthread_attr_destroy(attr: *mut ::pthread_attr_t) -> ::c_int;
    pub fn pthread_attr_setstacksize(attr: *mut ::pthread_attr_t,
                                     stack_size: ::size_t) -> ::c_int;
    pub fn pthread_attr_setdetachstate(attr: *mut ::pthread_attr_t,
                                       state: ::c_int) -> ::c_int;
    pub fn pthread_detach(thread: ::pthread_t) -> ::c_int;
    #[cfg_attr(target_os = "netbsd", link_name = "__libc_thr_yield")]
    pub fn sched_yield() -> ::c_int;
    pub fn pthread_key_create(key: *mut pthread_key_t,
                              dtor: Option<unsafe extern fn(*mut ::c_void)>)
                              -> ::c_int;
    pub fn pthread_key_delete(key: pthread_key_t) -> ::c_int;
    pub fn pthread_getspecific(key: pthread_key_t) -> *mut ::c_void;
    pub fn pthread_setspecific(key: pthread_key_t, value: *const ::c_void)
                               -> ::c_int;
    pub fn pthread_mutex_init(lock: *mut pthread_mutex_t,
                              attr: *const pthread_mutexattr_t) -> ::c_int;
    pub fn pthread_mutex_destroy(lock: *mut pthread_mutex_t) -> ::c_int;
    pub fn pthread_mutex_lock(lock: *mut pthread_mutex_t) -> ::c_int;
    pub fn pthread_mutex_trylock(lock: *mut pthread_mutex_t) -> ::c_int;
    pub fn pthread_mutex_unlock(lock: *mut pthread_mutex_t) -> ::c_int;

    pub fn pthread_mutexattr_init(attr: *mut pthread_mutexattr_t) -> ::c_int;
    #[cfg_attr(all(target_os = "macos", target_arch = "x86"),
               link_name = "pthread_mutexattr_destroy$UNIX2003")]
    pub fn pthread_mutexattr_destroy(attr: *mut pthread_mutexattr_t) -> ::c_int;
    pub fn pthread_mutexattr_settype(attr: *mut pthread_mutexattr_t,
                                     _type: ::c_int) -> ::c_int;

    #[cfg_attr(all(target_os = "macos", target_arch = "x86"),
               link_name = "pthread_cond_init$UNIX2003")]
    pub fn pthread_cond_init(cond: *mut pthread_cond_t,
                             attr: *const pthread_condattr_t) -> ::c_int;
    #[cfg_attr(all(target_os = "macos", target_arch = "x86"),
               link_name = "pthread_cond_wait$UNIX2003")]
    pub fn pthread_cond_wait(cond: *mut pthread_cond_t,
                             lock: *mut pthread_mutex_t) -> ::c_int;
    #[cfg_attr(all(target_os = "macos", target_arch = "x86"),
               link_name = "pthread_cond_timedwait$UNIX2003")]
    pub fn pthread_cond_timedwait(cond: *mut pthread_cond_t,
                              lock: *mut pthread_mutex_t,
                              abstime: *const ::timespec) -> ::c_int;
    pub fn pthread_cond_signal(cond: *mut pthread_cond_t) -> ::c_int;
    pub fn pthread_cond_broadcast(cond: *mut pthread_cond_t) -> ::c_int;
    pub fn pthread_cond_destroy(cond: *mut pthread_cond_t) -> ::c_int;
    pub fn pthread_condattr_init(attr: *mut pthread_condattr_t) -> ::c_int;
    pub fn pthread_condattr_destroy(attr: *mut pthread_condattr_t) -> ::c_int;
    #[cfg_attr(all(target_os = "macos", target_arch = "x86"),
               link_name = "pthread_rwlock_init$UNIX2003")]
    pub fn pthread_rwlock_init(lock: *mut pthread_rwlock_t,
                               attr: *const pthread_rwlockattr_t) -> ::c_int;
    #[cfg_attr(all(target_os = "macos", target_arch = "x86"),
               link_name = "pthread_rwlock_destroy$UNIX2003")]
    pub fn pthread_rwlock_destroy(lock: *mut pthread_rwlock_t) -> ::c_int;
    #[cfg_attr(all(target_os = "macos", target_arch = "x86"),
               link_name = "pthread_rwlock_rdlock$UNIX2003")]
    pub fn pthread_rwlock_rdlock(lock: *mut pthread_rwlock_t) -> ::c_int;
    #[cfg_attr(all(target_os = "macos", target_arch = "x86"),
               link_name = "pthread_rwlock_tryrdlock$UNIX2003")]
    pub fn pthread_rwlock_tryrdlock(lock: *mut pthread_rwlock_t) -> ::c_int;
    #[cfg_attr(all(target_os = "macos", target_arch = "x86"),
               link_name = "pthread_rwlock_wrlock$UNIX2003")]
    pub fn pthread_rwlock_wrlock(lock: *mut pthread_rwlock_t) -> ::c_int;
    #[cfg_attr(all(target_os = "macos", target_arch = "x86"),
               link_name = "pthread_rwlock_trywrlock$UNIX2003")]
    pub fn pthread_rwlock_trywrlock(lock: *mut pthread_rwlock_t) -> ::c_int;
    #[cfg_attr(all(target_os = "macos", target_arch = "x86"),
               link_name = "pthread_rwlock_unlock$UNIX2003")]
    pub fn pthread_rwlock_unlock(lock: *mut pthread_rwlock_t) -> ::c_int;
    pub fn pthread_rwlockattr_init(attr: *mut pthread_rwlockattr_t) -> ::c_int;
    pub fn pthread_rwlockattr_destroy(attr: *mut pthread_rwlockattr_t)
                                      -> ::c_int;

    pub fn sem_destroy(sem: *mut sem_t) -> ::c_int;
    #[cfg_attr(all(target_os = "macos", target_arch = "x86"),
               link_name = "sem_wait$UNIX2003")]
    pub fn sem_wait(sem: *mut sem_t) -> ::c_int;
    pub fn sem_trywait(sem: *mut sem_t) -> ::c_int;
    pub fn sem_post(sem: *mut sem_t) -> ::c_int;
    pub fn sem_init(sem: *mut sem_t,
                    pshared: ::c_int,
                    value: ::c_uint)
                    -> ::c_int;
}
