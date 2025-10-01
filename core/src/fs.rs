use std::{
  fmt::Display,
  fs,
  fs::{FileType, ReadDir},
  io,
  ops::{Deref, DerefMut},
  path::{Path, PathBuf},
};

use thiserror::Error;

#[derive(Debug, Error)]
#[error("{info} (caused by {io_error})")]
pub struct Error
{
  info:     ErrorInfo,
  #[source]
  io_error: io::Error,
}

#[derive(Debug, Error)]
pub enum ErrorInfo
{
  #[error("Could not read directory {path:?}")]
  ReadDir
  {
    path: PathBuf
  },
  #[error("Directory {path:?} maybe no exists")]
  Exists
  {
    path: PathBuf
  },
  #[error("Could not create directory {path:?}")]
  CreateDir
  {
    path: PathBuf
  },
  #[error("Could not enter directory {path:?}")]
  EnterDir
  {
    path: PathBuf
  },
  #[error("Could not copy {from:?} to {to:?}")]
  CopyFile
  {
    from: PathBuf, to: PathBuf
  },
  #[error("Could not open file {path:?}")]
  OpenFile
  {
    path: PathBuf
  },
  #[error("Could not create file {path:?}")]
  CreateFile
  {
    path: PathBuf
  },
  #[error("Could not create new file {path:?}")]
  CreateNewFile
  {
    path: PathBuf
  },
  #[error(
    "Could not copy {from:?} to {to:?} (caused by not support {file_type:?})"
  )]
  CopyNotSupportFileType
  {
    from:      PathBuf,
    to:        PathBuf,
    file_type: FileType,
  },
}
pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
pub struct File(fs::File);

pub fn create_dir_all(path: impl AsRef<Path>) -> Result<()>
{
  let path = path.as_ref();
  fs::create_dir_all(path).map_err(Error::create_dir(path))
}

pub fn create_dir(path: impl AsRef<Path>) -> Result<()>
{
  let path = path.as_ref();
  fs::create_dir(path).map_err(Error::create_dir(path))
}

pub fn read_dir(path: impl AsRef<Path>) -> Result<ReadDir>
{
  let path = path.as_ref();
  fs::read_dir(path).map_err(Error::read_dir(path))
}

pub fn exists(path: impl AsRef<Path>) -> Result<bool>
{
  let path = path.as_ref();
  fs::exists(path).map_err(Error::exists(path))
}

pub fn copy(from: impl AsRef<Path>, to: impl AsRef<Path>) -> Result<u64>
{
  let from = from.as_ref();
  let to = to.as_ref();
  fs::copy(from, to).map_err(Error::copy_file(from, to))
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
    let entry = entry.map_err(Error::enter_dir(from))?;
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
      Err(Error::copy_not_support_file_type(path, to_path, file_type))?
    }
  }
  Ok(())
}
impl File
{
  pub fn open(path: impl AsRef<Path>) -> Result<Self>
  {
    let path = path.as_ref();
    fs::File::open(path).map_err(Error::open_file(path)).map(Self)
  }

  pub fn create(path: impl AsRef<Path>) -> Result<Self>
  {
    let path = path.as_ref();
    fs::File::create(path).map_err(Error::create_file(path)).map(Self)
  }

  pub fn create_new(path: impl AsRef<Path>) -> Result<Self>
  {
    let path = path.as_ref();
    fs::File::create_new(path).map_err(Error::create_new_file(path)).map(Self)
  }
}

macro_rules! impl_error {
  ($($(@$ty:ident)? $name:ident => $to:ident),+ $(,)?) => {
    $(impl_error!{inner: $(@$ty)? $name => $to})+
  };
  (inner: $name:ident => $to:ident) => {
    pub fn $name(
      path: impl Into<::std::path::PathBuf>,
    ) -> impl FnOnce(::std::io::Error) -> Self
    {
      move |io_error| Self {
        info: ErrorInfo::$to { path: path.into() },
        io_error,
      }
    }
  };
  (inner: @from_to $name:ident => $to:ident) => {
    pub fn $name(
      from: impl Into<::std::path::PathBuf>,
      to: impl Into<::std::path::PathBuf>,
    ) -> impl FnOnce(::std::io::Error) -> Self
    {
      move |io_error| Self {
        info: ErrorInfo::$to { from: from.into(), to: to.into() },
        io_error,
      }
    }
  };
}

impl Error
{
  impl_error! {
    open_file=>OpenFile,
    create_dir=>CreateDir,
    create_file=>CreateFile,
    create_new_file=>CreateNewFile,
    enter_dir=>EnterDir,
    read_dir=>ReadDir,
    exists=>Exists,
    @from_to copy_file=>CopyFile,
  }

  pub fn copy_not_support_file_type(
    from: impl Into<PathBuf>, to: impl Into<PathBuf>, file_type: FileType,
  ) -> Self
  {
    Self {
      info:     ErrorInfo::CopyNotSupportFileType {
        from: from.into(),
        to: to.into(),
        file_type,
      },
      io_error: io::Error::new(io::ErrorKind::Other, "not support file type"),
    }
  }
}

impl Deref for File
{
  type Target = fs::File;

  fn deref(&self) -> &Self::Target
  {
    &self.0
  }
}
impl DerefMut for File
{
  fn deref_mut(&mut self) -> &mut Self::Target
  {
    &mut self.0
  }
}
