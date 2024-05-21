use bytesize::ByteSize;
use sysinfo::{Gid, Groups, Uid, Users};

use crate::error::Result;
use std::{
    fs::{self, File, OpenOptions},
    os::unix::fs::{MetadataExt, PermissionsExt},
    path::PathBuf,
    time::{Duration, SystemTime, UNIX_EPOCH},
};

/// General file information.
#[derive(Debug)]
pub struct FileInfo<'a> {
    /// Path of the file.
    pub path: &'a str,
    /// Bytes of the file.
    pub bytes: &'a [u8],
    /// Whether if the file is read only.
    pub is_read_only: bool,
    /// Name of the file.
    pub name: String,
    /// Size of the file.
    pub size: String,
    /// Number of blocks allocated for the file.
    pub blocks: u64,
    /// Block size.
    pub block_size: u64,
    /// Device ID.
    pub device: u64,
    /// Inode number.
    pub inode: u64,
    /// Number of hard links.
    pub links: u64,
    /// Access information.
    pub access: FileAccessInfo,
    /// Date information.
    pub date: FileDateInfo,
}

/// Access information.
#[derive(Debug)]
pub struct FileAccessInfo {
    /// Access mode.
    pub mode: String,
    /// Accessed user.
    pub uid: String,
    /// Accessed group.
    pub gid: String,
}

/// Date information.
#[derive(Debug)]
pub struct FileDateInfo {
    /// Access date.
    pub access: String,
    /// Modify date.
    pub modify: String,
    /// Change date.
    pub change: String,
    /// Birth date.
    pub birth: String,
}

impl<'a> FileInfo<'a> {
    /// Constructs a new instance.
    pub fn new(path: &'a str, bytes: &'a [u8]) -> Result<Self> {
        let metadata = fs::metadata(path)?;
        let mode = metadata.permissions().mode();

        let users = Users::new_with_refreshed_list();
        let groups = Groups::new_with_refreshed_list();
        Ok(Self {
            path,
            bytes,
            is_read_only: false,
            name: PathBuf::from(path)
                .file_name()
                .map(|v| v.to_string_lossy().to_string())
                .unwrap_or_default(),
            size: ByteSize(metadata.len()).to_string(),
            blocks: metadata.blocks(),
            block_size: metadata.blksize(),
            device: metadata.dev(),
            inode: metadata.ino(),
            links: metadata.nlink(),
            access: FileAccessInfo {
                mode: format!("{:04o}/{}", mode & 0o777, {
                    let mut s = String::new();
                    s.push(if mode & 0o400 != 0 { 'r' } else { '-' });
                    s.push(if mode & 0o200 != 0 { 'w' } else { '-' });
                    s.push(if mode & 0o100 != 0 { 'x' } else { '-' });
                    s.push(if mode & 0o040 != 0 { 'r' } else { '-' });
                    s.push(if mode & 0o020 != 0 { 'w' } else { '-' });
                    s.push(if mode & 0o010 != 0 { 'x' } else { '-' });
                    s.push(if mode & 0o004 != 0 { 'r' } else { '-' });
                    s.push(if mode & 0o002 != 0 { 'w' } else { '-' });
                    s.push(if mode & 0o001 != 0 { 'x' } else { '-' });
                    s
                }),
                uid: format!(
                    "{}/{}",
                    metadata.uid(),
                    Uid::try_from(metadata.uid() as usize)
                        .ok()
                        .and_then(|uid| users.get_user_by_id(&uid))
                        .map(|v| v.name())
                        .unwrap_or("?")
                ),
                gid: format!(
                    "{}/{}",
                    metadata.gid(),
                    groups
                        .list()
                        .iter()
                        .find(|g| Gid::try_from(metadata.gid() as usize).as_ref() == Ok(g.id()))
                        .map(|v| v.name())
                        .unwrap_or("?")
                ),
            },
            date: {
                // Helper function to format SystemTime
                fn format_system_time(system_time: SystemTime) -> String {
                    let datetime: chrono::DateTime<chrono::Local> = system_time.into();
                    format!("{}", datetime.format("%Y-%m-%d %H:%M:%S.%f %z"))
                }
                FileDateInfo {
                    access: format_system_time(metadata.accessed()?),
                    modify: format_system_time(metadata.modified()?),
                    change: format_system_time(
                        UNIX_EPOCH
                            + Duration::new(
                                metadata.ctime().try_into()?,
                                metadata.ctime_nsec().try_into()?,
                            ),
                    ),
                    birth: format_system_time(metadata.created()?),
                }
            },
        })
    }

    /// Opens the file (with R/W if possible) and returns it.
    pub fn open_file(&mut self) -> Result<File> {
        Ok(
            match OpenOptions::new().write(true).read(true).open(self.path) {
                Ok(v) => v,
                Err(_) => {
                    self.is_read_only = true;
                    File::open(self.path)?
                }
            },
        )
    }
}
