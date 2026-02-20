pub mod cmd_add_shell;
pub mod cmd_addgroup;
pub mod cmd_true;
pub mod cmd_false;
pub mod cmd_echo;

pub mod util {
    use libc::*;

    pub fn current_uid() -> uid_t {
        unsafe { libc::getuid() }
    }

    pub fn current_euid() -> uid_t {
        unsafe { libc::geteuid() }
    }
}

