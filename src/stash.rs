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

pub struct Stash {
    name: String,              // "skate (SanDisk Extreme Pro - 64GB - 280MB/s R, 100MB/s W)"
    block_device_name: String, // "sda1"
}

impl Stash {
    pub fn new(name: String, block_device_name: String) -> Self {
        Self {
            name,
            block_device_name,
        }
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
        println!("- name: {}", self.name);
        println!("- block_device_name: {:?}", self.block_device_name);
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

    fn block_device_path(&self) -> PathBuf {
        PathBuf::from("/dev").join(&self.block_device_name)
    }

    fn mount_path(&self) -> PathBuf {
        PathBuf::from("/mnt").join(format!("sd-card_{}", self.name))
    }
}
