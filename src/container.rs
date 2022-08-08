use std::time::Duration;

/// CgroupfsDefaultCgroupParent is the cgroup parent for CgroupFS in libpod.
const CGROUPFS_DEFAULT_CGROUP_PARENT: &str = "/libpod_parent";

/// SystemdDefaultCgroupParent is the cgroup parent for the systemd cgroup
/// manager in libpod.
const SYSTEMD_DEFAULT_CGROUP_PARENT: &str = "machine.slice";

/// SystemdDefaultRootlessCgroupParent is the cgroup parent for the systemd cgroup
/// manager in libpod when running as rootless.
const SYSTEMD_DEFAULT_ROOTLESS_CGROUP_PARENT: &str = "user.slice";

/// DefaultWaitInterval is the default interval between container status checks
/// while waiting.
const DEFAULT_WAIT_INTERVAL: Duration = Duration::from_millis(250);

/// Represents the ID for a [Linux namespace](https://en.wikipedia.org/wiki/Linux_namespaces).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LinuxNs {
    /// Represents an invalid namespace.
    InvalidNs,

    /// Represents the IPC namespace.
    IpcNs,

    /// Represents the mount namespace.
    MountNs,

    /// Represents the network namespace.
    NetNs,

    /// Represents the PID namespace.
    PidNs,

    /// Represents the user namesapce.
    UserNs,

    /// Represents the UTS namespace.
    UtsNs,

    /// Represents the Cgroup namesapce.
    CgroupNs,
}

impl std::fmt::Display for LinuxNs {
    /// Returns a string representation of the Linux namespace.
    ///
    /// This is guaranteed to be equivalent to the name of the namespace used in /proc.
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::InvalidNs => "invalid",
                Self::IpcNs => "ipc",
                Self::MountNs => "mnt",
                Self::NetNs => "net",
                Self::PidNs => "pid",
                Self::UserNs => "user",
                Self::UtsNs => "uts",
                Self::CgroupNs => "cgroup",
            }
        )
    }
}

pub struct Container {
    pub config: ContainerConfig,
    pub state: ContainerState,

    /// Indicates whether or not a container has been locked as part of a batch operation.
    pub batched: bool,
}

pub struct ContainerConfig {}
pub struct ContainerState {}
