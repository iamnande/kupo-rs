use serde::Deserialize;

use crate::error::{KupoError, Result};

use std::{
    fmt,
    fs::File,
    io::{self, BufRead},
    path::PathBuf,
    process::Command,
};

#[derive(Debug)]
pub enum StashStatus {
    Open,
    Closed,
}

impl fmt::Display for StashStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Open => write!(f, "open"),
            Self::Closed => write!(f, "closed"),
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct Stash {
    pub slug: String,  // skate
    pub label: String, // "Skateboarding Videos (2026)"
}

impl Stash {
    pub fn new(slug: String, label: String) -> Self {
        Self { slug, label }
    }

    pub fn status(&self) -> Result<StashStatus> {
        let file = File::open("/proc/self/mountinfo")?;
        let mount_records = io::BufReader::new(file).lines();

        for mount_record in mount_records {
            let mount_record = mount_record?;
            let fields: Vec<&str> = mount_record.split_whitespace().collect();

            if fields.len() > 4 && fields[4] == self.mount_path().to_string_lossy() {
                return Ok(StashStatus::Open);
            }
        }

        Ok(StashStatus::Closed)
    }

    pub fn close(&self) -> Result<()> {
        println!("closing stash, kupo!");
        self.ensure_unmounted()?;
        self.ensure_mount_path_removed()?;
        Ok(())
    }

    pub fn inspect(&self) -> Result<()> {
        println!("- slug: {}", self.slug);
        println!("- label: {}", self.label);
        println!("- block_device_path: {:?}", self.block_device_path());
        println!("- mount_path: {:?}", self.mount_path());
        println!("- status: {}", self.status()?);
        Ok(())
    }

    fn ensure_mount_path_exists(&self) -> Result<()> {
        std::fs::create_dir_all(self.mount_path())?;
        Ok(())
    }

    fn ensure_mount_path_removed(&self) -> Result<()> {
        if self.mount_path().exists() {
            std::fs::remove_dir(self.mount_path())?;
        }
        Ok(())
    }

    // TODO: syscall
    fn ensure_mounted(&self) -> Result<()> {
        if let StashStatus::Open = self.status()? {
            return Err(KupoError::StashOpen);
        }

        let status = Command::new("mount")
            .arg(self.block_device_path())
            .arg(self.mount_path())
            .status()?;
        if !status.success() {
            return Err(KupoError::MountFailed(status));
        }

        Ok(())
    }

    // TODO: syscall
    fn ensure_unmounted(&self) -> Result<()> {
        if let StashStatus::Closed = self.status()? {
            return Err(KupoError::StashClosed);
        }

        let status = Command::new("umount").arg(self.mount_path()).status()?;
        if !status.success() {
            return Err(KupoError::UmountFailed(status));
        }

        Ok(())
    }

    pub fn open(&self) -> Result<()> {
        println!("opening stash, kupo!");
        self.ensure_mount_path_exists()?;
        self.ensure_mounted()?;
        Ok(())
    }

    // TODO: discovery
    fn block_device_path(&self) -> PathBuf {
        PathBuf::from("/dev").join("sda1")
    }

    // TODO: discovery++
    fn mount_path(&self) -> PathBuf {
        PathBuf::from("/mnt").join(format!("sd-card_{}", self.slug))
    }
}
