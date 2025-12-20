use crate::runtime::Result;
use std::path::Path;

use super::isolation::{
    chdir_to_container_root, detach_and_remove_old_root, ensure_rootfs_is_mountpoint,
    enter_mount_and_uts_namespaces, make_mounts_private, mount_minimal_dev, mount_proc,
    pivot_root_into, set_container_hostname,
};

pub struct ContainerSetup<'a> {
    rootfs: &'a Path,
    hostname: Option<&'a str>,
    mount_proc: bool,
    mount_dev: bool,
}

impl<'a> ContainerSetup<'a> {
    pub fn new(rootfs: &'a Path) -> Self {
        Self {
            rootfs,
            hostname: None,
            mount_proc: true,
            mount_dev: true,
        }
    }

    pub fn hostname(mut self, hostname: Option<&'a str>) -> Self {
        self.hostname = hostname;
        self
    }

    pub fn mount_proc(mut self, enabled: bool) -> Self {
        self.mount_proc = enabled;
        self
    }

    pub fn mount_dev(mut self, enabled: bool) -> Self {
        self.mount_dev = enabled;
        self
    }

    pub fn apply(self) -> Result<()> {
        // isolation and propagation control
        enter_mount_and_uts_namespaces()?;
        make_mounts_private()?;

        // rootfs becomes "/"
        ensure_rootfs_is_mountpoint(self.rootfs)?;
        pivot_root_into(self.rootfs)?;
        chdir_to_container_root()?;

        // basic virtual filesystems
        if self.mount_proc {
            mount_proc()?;
        }

        if self.mount_dev {
            mount_minimal_dev()?;
        }

        set_container_hostname(self.hostname)?;

        detach_and_remove_old_root()?;

        Ok(())
    }
}
