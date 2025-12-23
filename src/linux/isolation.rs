use crate::runtime::Result;
use nix::{
    mount::{MntFlags, MsFlags, mount, umount2},
    sched::{CloneFlags, unshare},
    unistd::{chdir, pivot_root, sethostname},
};
use std::{fs, io, path::Path};

pub fn enter_mount_and_uts_namespaces() -> Result<()> {
    unshare(CloneFlags::CLONE_NEWNS | CloneFlags::CLONE_NEWUTS).map_err(nix_to_io)?;
    Ok(())
}

pub fn make_mounts_private() -> Result<()> {
    mount::<str, str, str, str>(None, "/", None, MsFlags::MS_REC | MsFlags::MS_PRIVATE, None)
        .map_err(nix_to_io)?;
    Ok(())
}

pub fn ensure_rootfs_is_mountpoint(rootfs: &Path) -> Result<()> {
    mount(
        Some(rootfs),
        rootfs,
        Option::<&str>::None,
        MsFlags::MS_BIND | MsFlags::MS_REC,
        Option::<&str>::None,
    )
    .map_err(nix_to_io)?;
    Ok(())
}

pub fn pivot_root_into(rootfs: &Path) -> Result<()> {
    let put_old = rootfs.join(".oldroot");
    fs::create_dir_all(&put_old)?;
    pivot_root(rootfs, &put_old).map_err(nix_to_io)?;
    Ok(())
}

pub fn chdir_to_container_root() -> Result<()> {
    chdir("/").map_err(nix_to_io)?;
    Ok(())
}

pub fn mount_proc() -> Result<()> {
    fs::create_dir_all("/proc")?;
    mount(
        Some("proc"),
        "/proc",
        Some("proc"),
        MsFlags::empty(),
        Option::<&str>::None,
    )
    .map_err(nix_to_io)?;
    Ok(())
}

pub fn mount_minimal_dev() -> Result<()> {
    fs::create_dir_all("/dev")?;
    mount(
        Some("tmpfs"),
        "/dev",
        Some("tmpfs"),
        MsFlags::empty(),
        Some("mode=755"),
    )
    .map_err(nix_to_io)?;
    Ok(())
}

pub fn set_container_hostname(hostname: Option<&str>) -> Result<()> {
    if let Some(h) = hostname {
        sethostname(h).map_err(nix_to_io)?;
    }
    Ok(())
}

pub fn detach_and_remove_old_root() -> Result<()> {
    umount2("/.oldroot", MntFlags::MNT_DETACH).map_err(nix_to_io)?;
    let _ = fs::remove_dir_all("/.oldroot");
    Ok(())
}

// TODO: Separate into a separate file
pub fn nix_to_io<E: std::fmt::Display>(e: E) -> io::Error {
    io::Error::new(io::ErrorKind::Other, e.to_string())
}
