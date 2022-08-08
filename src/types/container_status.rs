/// Indicates the status of a [`crate::container::Container`].
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ContainerStatus {
    /// Indicates that the container is in an error state in which information
    /// about it cannot be retrieved.
    Unknown,

    /// Indicates that the container's storage has been configured but that it
    /// has not been created in the OCI runtime yet.
    Configured,

    /// Indicates that the container has been created in the OCI runtime but has
    /// not been started yet.
    Initialized,

    /// Indicates that the container is currently running.
    Running,

    /// Indicates that the container is in the process of stopping.
    Stopping,

    /// Indicates that the container is stopped (but has previously been
    /// running).
    Stopped,

    /// Indicates that the container has been paused.
    Paused,

    /// Indicates that the container has been exited.
    Exited,

    /// Indicates that the container is currently being removed.
    Removing,
}

impl std::fmt::Display for ContainerStatus {
    /// Stringifies the [`ContainerStatus`].
    ///
    /// These match Docker's `docker ps` statuses.
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Unknown => "unknown",
                Self::Configured => "configured",
                Self::Initialized => "initialized",
                Self::Running => "running",
                Self::Stopping => "stopping",
                Self::Stopped => "stopped",
                Self::Paused => "paused",
                Self::Exited => "exited",
                Self::Removing => "removing",
            }
        )
    }
}

impl From<String> for ContainerStatus {
    /// Converts a stringified status back to a [`ContainerStatus`].
    ///
    /// If none of the options are matched, [`ContainerStatus::Unknown`] will be
    /// returned.
    fn from(val: String) -> Self {
        match val.as_str() {
            "unknown" => Self::Unknown,
            "configured" => Self::Configured,
            "initialized" => Self::Initialized,
            "running" => Self::Running,
            "stopping" => Self::Stopping,
            "stopped" => Self::Stopped,
            "paused" => Self::Paused,
            "exited" => Self::Exited,
            "removing" => Self::Removing,
            _ => Self::Unknown,
        }
    }
}

/// Indicates the status of a container exec session.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ContainerExecStatus {
    /// Indicates that the exec session is in an unknown state.
    Unknown,

    /// Indicates that the exec session has been created but not started yet.
    Created,

    /// Indicates that the exec session has been started but has not exited yet.
    Running,

    /// Indicates that the exec session has stopped.
    Stopped,
}
