use std::{
  fs,
  fs::ReadDir,
  io,
  path::{Path, PathBuf},
};

use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error
{
  #[error("Could not read directory {path:?} (caused by {io_error})")]
  ReadDir
  {
    path:     PathBuf,
    #[source]
    io_error: io::Error,
  },
  #[error("Directory {path:?} maybe no exists (caused by {io_error})")]
  Exists
  {
    path:     PathBuf,
    #[source]
    io_error: io::Error,
  },
  #[error("Could not create directory {path:?} (caused by {io_error})")]
  CreateDir
  {
    path:     PathBuf,
    #[source]
    io_error: io::Error,
  },
  #[error("Could not enter directory {path:?} (caused by {io_error})")]
  EnterDir
  {
    path:     PathBuf,
    #[source]
    io_error: io::Error,
  },
  #[error("Could not copy {from:?} to {to:?} (caused by {io_error})")]
  CopyFile
  {
    from:     PathBuf,
    to:       PathBuf,
    #[source]
    io_error: io::Error,
  },
  #[error(
    "Could not copy {from:?} to {to:?} (caused by not support {file_type:?})"
  )]
  CopyNotSupportFileType
  {
    from:      PathBuf,
    to:        PathBuf,
    file_type: fs::FileType,
  },
}
pub type Result<T> = std::result::Result<T, Error>;

pub fn create_dir_all(path: impl AsRef<Path>) -> Result<()>
{
  let path = path.as_ref();
  fs::create_dir_all(path)
    .map_err(|io_error| Error::CreateDir { path: path.to_path_buf(), io_error })
}

pub fn create_dir(path: impl AsRef<Path>) -> Result<()>
{
  let path = path.as_ref();
  fs::create_dir(path)
    .map_err(|io_error| Error::CreateDir { path: path.to_path_buf(), io_error })
}

pub fn read_dir(path: impl AsRef<Path>) -> Result<ReadDir>
{
  let path = path.as_ref();
  fs::read_dir(path)
    .map_err(|io_error| Error::ReadDir { path: path.to_path_buf(), io_error })
}

pub fn exists(path: impl AsRef<Path>) -> Result<bool>
{
  let path = path.as_ref();
  fs::exists(path)
    .map_err(|io_error| Error::Exists { path: path.to_path_buf(), io_error })
}

pub fn copy(from: impl AsRef<Path>, to: impl AsRef<Path>) -> Result<u64>
{
  let from = from.as_ref();
  let to = to.as_ref();
  fs::copy(from, to).map_err(|io_error| Error::CopyFile {
    from: from.to_path_buf(),
    to: to.to_path_buf(),
    io_error,
  })
}

pub fn copy_dir(from: impl AsRef<Path>, to: impl AsRef<Path>) -> Result<()>
{
  let from = from.as_ref();
  let to = to.as_ref();
  if !exists(to)?
  {
    create_dir_all(to)?;
  }
  for entry in read_dir(from)?
  {
    let entry = entry.map_err(|io_error| Error::EnterDir {
      path: from.to_path_buf(),
      io_error,
    })?;
    let path = entry.path();
    let file_type = entry.file_type().unwrap();
    let rel_path = path.strip_prefix(from).unwrap();
    let to_path = to.join(rel_path);
    if let Some(parent) = to_path.parent()
    {
      create_dir_all(parent)?;
    }
    if file_type.is_file()
    {
      copy(path, to_path)?;
    }
    else if file_type.is_dir()
    {
      copy_dir(path, to_path)?;
    }
    else
    {
      Err(Error::CopyNotSupportFileType { from: path, to: to_path, file_type })?
    }
  }
  Ok(())
}
