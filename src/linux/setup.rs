use nix::{
    mount::{MntFlags, MsFlags, mount, umount2},
    sched::unshare,
    unistd::{chdir, pivot_root, sethostname},
};
use std::{ffi::CString, fs, path::Path};

pub fn setup_container(rootfs: &Path, hostname: Option<&str>) -> std::io::Result<()> {
    unimplemented!()
}
