use std::io::{BufRead, Read, Result, Write};

use crate::wrapper::{Wrapper, WrapperOwned};

/// line reader
/// # Optional
/// + `pre_read`: pre-read hook, run on every read
/// # Examples
/// ```
/// # use algosul_core::util::std::io::{LineReadExt, LineRead};
/// let mut reader1 = "hi\n".as_bytes().line_reader();
/// let mut reader2 =
///   "hi\n".as_bytes().line_reader_with(move || println!("pre-read"));
/// let mut input1 = String::new();
/// let mut input2 = String::new();
/// reader1.read_line(&mut input1).unwrap();
/// reader2.read_line(&mut input2).unwrap();
/// assert_eq!("hi\n", input1);
/// assert_eq!("hi\n", input2);
/// ```
/// You can use it with [`PromptWriteExt`]
/// ```
/// # use algosul_core::util::std::io::{LineReadExt, LineRead, PromptWriteExt};
/// use std::io::stdout;
/// let mut reader = "hi\n".as_bytes().line_reader();
/// let mut writer = stdout().prompt_writer("> ");
/// let mut input = String::new();
/// reader.read_line_with_prompt(&mut input, &mut writer).unwrap();
/// assert_eq!("hi\n", input);
/// ```
pub trait LineReadExt: BufRead
{
  /// line reader
  /// # Examples
  /// ```
  /// # use algosul_core::util::std::io::{LineReadExt, LineRead};
  /// let mut reader = "hi\n".as_bytes().line_reader();
  /// let mut input = String::new();
  /// reader.read_line(&mut input).unwrap();
  /// assert_eq!("hi\n", input);
  /// ```
  fn line_reader(self) -> LineReader<Self, ()>
  where
    Self: Sized,
  {
    LineReader::<_, ()>::new(self)
  }
  /// line reader
  /// # Params
  /// + `pre_read`: pre-read hook, run on every read
  /// # Examples
  /// ```
  /// # use algosul_core::util::std::io::{LineReadExt, LineRead};
  /// let mut reader =
  ///   "hi\n".as_bytes().line_reader_with(move || println!("pre-read"));
  /// let mut input = String::new();
  /// reader.read_line(&mut input).unwrap();
  /// assert_eq!("hi\n", input);
  /// ```
  fn line_reader_with<F: FnMut()>(self, pre_read: F) -> LineReader<Self, F>
  where
    Self: Sized,
  {
    LineReader::<_, F>::new_with(self, pre_read)
  }
}

/// Used for repeatedly writing `data` or using `closures` to generate `data`
/// for writing.
/// You can use it with [`LineReadExt`]
/// + `data`: any `impl AsRef<[u8]>`, e.g. [`String`]
/// + `closures`: any `impl FnMut() -> data`, e.g. `|| "> "`
/// # Examples
/// ```
/// # use algosul_core::util::std::io::{PromptWriteExt, PromptWrite};
/// # use std::io::Read;
/// let (mut reader1, writer1) = std::io::pipe().unwrap();
/// let (mut reader2, writer2) = std::io::pipe().unwrap();
///
/// let now = "2027/07/12";
///
/// let mut writer1 = writer1.prompt_writer(format!("{now}> "));
/// let mut writer2 = writer2.prompt_writer_with(move || format!("{now}> "));
/// writer1.write_prompt().unwrap();
/// writer2.write_prompt().unwrap();
/// writer1.flush().unwrap();
/// writer2.flush().unwrap();
///
/// # drop(writer1);
/// # drop(writer2);
/// # let mut read1 = String::new();
/// # let mut read2 = String::new();
/// # reader1.read_to_string(&mut read1).unwrap();
/// # reader2.read_to_string(&mut read2).unwrap();
/// # assert_eq!(read1, format!("{now}> "));
/// # assert_eq!(read2, format!("{now}> "));
/// ```
pub trait PromptWriteExt: Write
{
  /// Used for repeatedly writing `data`
  /// + `data`: any `impl AsRef<[u8]>`, e.g. [`String`]
  /// # Examples
  /// ```
  /// # use algosul_core::util::std::io::{PromptWriteExt, PromptWrite};
  /// # use std::io::Read;
  /// let (mut reader, writer) = std::io::pipe().unwrap();
  ///
  /// let now = "2027/07/12";
  ///
  /// let mut writer = writer.prompt_writer(format!("{now}> "));
  /// writer.write_prompt().unwrap();
  /// writer.flush().unwrap();
  ///
  /// # drop(writer);
  /// # let mut read = String::new();
  /// # reader.read_to_string(&mut read).unwrap();
  /// # assert_eq!(read, format!("{now}> "));
  /// ```
  fn prompt_writer<P: AsRef<[u8]>>(self, prompt: P) -> PromptWriter<Self, P>
  where
    Self: Sized,
  {
    PromptWriter::<_, P>::new(self, prompt)
  }

  /// using `closures` to generate `data` for writing.
  /// + `data`: any `impl AsRef<[u8]>`, e.g. [`String`]
  /// + `closures`: any `impl FnMut() -> data`, e.g. `|| "> "`
  /// # Examples
  /// ```
  /// # use algosul_core::util::std::io::{PromptWriteExt, PromptWrite};
  /// # use std::io::Read;
  /// let (mut reader, writer) = std::io::pipe().unwrap();
  ///
  /// let now = "2027/07/12";
  ///
  /// let mut writer = writer.prompt_writer_with(move || format!("{now}> "));
  /// writer.write_prompt().unwrap();
  /// writer.flush().unwrap();
  ///
  /// # drop(writer);
  /// # let mut read = String::new();
  /// # reader.read_to_string(&mut read).unwrap();
  /// # assert_eq!(read, format!("{now}> "));
  /// ```
  fn prompt_writer_with<P: AsRef<[u8]>, F: FnMut() -> P>(
    self, prompt: F,
  ) -> PromptFnWriter<Self, F>
  where
    Self: Sized,
  {
    PromptFnWriter::<_, F>::new(self, prompt)
  }
}

pub trait LineRead
{
  /// read line
  /// see [`LineReadExt`]
  fn read_line(&mut self, buffer: &mut String) -> Result<()>;

  /// write prompt and read line
  /// see [`LineReadExt`]
  fn read_line_with_prompt<W: PromptWrite>(
    &mut self, buffer: &mut String, prompt_writer: &mut W,
  ) -> Result<()>;
}

pub trait PromptWrite
{
  /// write prompt
  /// see [`PromptWriteExt`]
  fn write_prompt(&mut self) -> Result<()>;

  /// flush buffer
  /// see [`PromptWriteExt`]
  fn flush(&mut self) -> Result<()>;
}

/// see [`PromptWriteExt`]
#[derive(Debug, Eq, PartialEq)]
pub struct PromptWriter<W: ?Sized, P: AsRef<[u8]>>
{
  prompt: P,
  inner: W,
}

/// see [`PromptWriteExt`]
#[derive(Debug, Eq, PartialEq)]
pub struct PromptFnWriter<W: ?Sized, P>
{
  prompt_fn: P,
  inner: W,
}

/// see [`LineReadExt`]
#[derive(Debug, Eq, PartialEq)]
pub struct LineReader<R: ?Sized, P>
{
  pre_read: P,
  inner: R,
}

impl<R: BufRead> LineReadExt for R {}

impl<W: Write> PromptWriteExt for W {}

impl<W: ?Sized, P: AsRef<[u8]>, F: FnMut() -> P> PromptFnWriter<W, F>
{
  /// see [`PromptWriteExt::prompt_writer_with`]
  pub fn new(inner: W, prompt_fn: F) -> Self
  where
    W: Sized,
  {
    Self { inner, prompt_fn }
  }
}

impl<W: ?Sized, P: AsRef<[u8]>> PromptWriter<W, P>
{
  /// see [`PromptWriteExt::prompt_writer`]
  pub fn new(inner: W, prompt: P) -> Self
  where
    W: Sized,
  {
    Self { inner, prompt }
  }
}

impl<R: ?Sized> LineReader<R, ()>
{
  /// see [`LineReadExt::line_reader`]
  pub fn new(inner: R) -> Self
  where
    R: Sized,
  {
    Self { inner, pre_read: () }
  }
}

impl<R: ?Sized, P: FnMut()> LineReader<R, P>
{
  /// see [`LineReadExt::line_reader_with`]
  pub fn new_with(inner: R, pre_read: P) -> Self
  where
    R: Sized,
  {
    Self { inner, pre_read }
  }
}

impl<W: ?Sized, P: AsRef<[u8]>, F: FnMut() -> P> Wrapper<W>
for PromptFnWriter<W, F>
{
  fn inner(&self) -> &W
  {
    &self.inner
  }

  fn inner_mut(&mut self) -> &mut W
  {
    &mut self.inner
  }
}

impl<W, P: AsRef<[u8]>, F: FnMut() -> P> WrapperOwned<W>
for PromptFnWriter<W, F>
{
  fn into_inner(self) -> W
  {
    self.inner
  }
}

impl<W: ?Sized, P: AsRef<[u8]>> Wrapper<W> for PromptWriter<W, P>
{
  fn inner(&self) -> &W
  {
    &self.inner
  }

  fn inner_mut(&mut self) -> &mut W
  {
    &mut self.inner
  }
}

impl<W, P: AsRef<[u8]>> WrapperOwned<W> for PromptWriter<W, P>
{
  fn into_inner(self) -> W
  {
    self.inner
  }
}

impl<R: ?Sized, P> Wrapper<R> for LineReader<R, P>
{
  fn inner(&self) -> &R
  {
    &self.inner
  }

  fn inner_mut(&mut self) -> &mut R
  {
    &mut self.inner
  }
}

impl<R, P> WrapperOwned<R> for LineReader<R, P>
{
  fn into_inner(self) -> R
  where
    R: Sized,
  {
    self.inner
  }
}

impl<W: Write, P: AsRef<[u8]>> PromptWrite for PromptWriter<W, P>
{
  fn write_prompt(&mut self) -> Result<()>
  {
    self.inner.write_all(self.prompt.as_ref())
  }

  fn flush(&mut self) -> Result<()>
  {
    self.inner.flush()
  }
}

impl<W: Write, P: AsRef<[u8]>, F: FnMut() -> P> PromptWrite
for PromptFnWriter<W, F>
{
  fn write_prompt(&mut self) -> Result<()>
  {
    self.inner.write_all((self.prompt_fn)().as_ref())
  }

  fn flush(&mut self) -> Result<()>
  {
    self.inner.flush()
  }
}

impl<R: BufRead + ?Sized> LineRead for LineReader<R, ()>
{
  fn read_line(&mut self, buffer: &mut String) -> Result<()>
  {
    self.inner.read_line(buffer)?;
    Ok(())
  }

  fn read_line_with_prompt<W: PromptWrite>(
    &mut self, buffer: &mut String, prompt_writer: &mut W,
  ) -> Result<()>
  {
    prompt_writer.write_prompt()?;
    prompt_writer.flush()?;
    self.read_line(buffer)
  }
}

impl<R: BufRead + ?Sized, P: FnMut()> LineRead for LineReader<R, P>
{
  fn read_line(&mut self, buffer: &mut String) -> Result<()>
  {
    (self.pre_read)();
    self.inner.read_line(buffer)?;
    Ok(())
  }

  fn read_line_with_prompt<W: PromptWrite>(
    &mut self, buffer: &mut String, prompt_writer: &mut W,
  ) -> Result<()>
  {
    (self.pre_read)();
    prompt_writer.write_prompt()?;
    prompt_writer.flush()?;
    self.read_line(buffer)
  }
}

impl<R: Read + Clone, P: Fn() + Clone> Clone for LineReader<R, P>
{
  fn clone(&self) -> Self
  {
    Self {
      inner: self.inner.clone(),
      pre_read: self.pre_read.clone(),
    }
  }
}

#[cfg(test)]
mod tests
{
  use std::io::{pipe, stdout, BufReader, PipeReader};

  use super::*;

  const INPUT_LIST: [&str; 3] = ["first", "second", "third"];
  const TEST_OUTPUT_STR: &str = "test Prompt Writer\n";
  const TEST_OUTPUT_NUM: u32 = 10000;

  fn input() -> String
  {
    INPUT_LIST.join("\n")
  }

  fn check_read_line(mut read_fn: impl FnMut(&mut String))
  {
    let mut buffer = String::new();
    let mut iter = INPUT_LIST.into_iter().peekable();
    while let Some(i) = iter.next()
    {
      buffer.clear();
      read_fn(&mut buffer);
      let right =
        iter.peek().map_or_else(|| i.to_string(), |_| format!("{i}\n"));
      assert_eq!(buffer, right);
    }
  }

  fn check_prompt_writer(
    mut pipe_reader: PipeReader,
    mut sync_write_fn: impl FnMut(&mut String) + Send + 'static,
  )
  {
    let handle = std::thread::spawn(move || {
      let mut buffer = String::new();
      sync_write_fn(&mut buffer);
    });

    let mut read = String::new();
    pipe_reader.read_to_string(&mut read).unwrap();
    assert_eq!(read, TEST_OUTPUT_STR);

    handle.join().unwrap();
  }

  fn check_prompt_writer_counter(
    pipe_reader: PipeReader,
    mut sync_write_fn: impl FnMut(&mut String) + Send + 'static,
  )
  {
    let handle = std::thread::spawn(move || {
      let mut buffer = String::new();
      for _ in 0..TEST_OUTPUT_NUM
      {
        buffer.clear();
        sync_write_fn(&mut buffer);
      }
    });

    let mut pipe_reader = BufReader::new(pipe_reader);
    let mut read = String::new();
    for i in 0..TEST_OUTPUT_NUM
    {
      read.clear();
      pipe_reader.read_line(&mut read).unwrap();
      assert_eq!(read, format!("{i}\n"));
    }

    handle.join().unwrap();
  }

  #[test]
  fn test_line_reader()
  {
    let input = input();
    let mut reader = input.as_bytes().line_reader();
    check_read_line(move |buffer| reader.read_line(buffer).unwrap());
  }

  #[test]
  fn test_line_reader_with_prompt_writer()
  {
    let input = input();
    let mut reader = input.as_bytes().line_reader();
    let mut writer = stdout().prompt_writer(TEST_OUTPUT_STR);
    check_read_line(move |buffer| {
      reader.read_line_with_prompt(buffer, &mut writer).unwrap()
    });
  }

  #[test]
  fn test_prompt_writer()
  {
    let (pipe_reader, writer) = pipe().unwrap();
    let mut writer = writer.prompt_writer(TEST_OUTPUT_STR);
    check_prompt_writer(pipe_reader, move |_buffer| {
      writer.write_prompt().unwrap();
      writer.flush().unwrap();
    });
  }

  #[test]
  fn test_prompt_writer_counter()
  {
    let (pipe_reader, writer) = pipe().unwrap();
    let mut counter = -1;
    let mut writer = writer.prompt_writer_with(move || {
      counter += 1;
      format!("{counter}\n")
    });
    check_prompt_writer_counter(pipe_reader, move |_buffer| {
      writer.write_prompt().unwrap();
      writer.flush().unwrap();
    });
  }

  #[test]
  fn test_prompt_writer_with_line_reader()
  {
    let (pipe_reader, writer) = pipe().unwrap();
    let mut writer = writer.prompt_writer(TEST_OUTPUT_STR);
    check_prompt_writer(pipe_reader, move |buffer| {
      let input = input();
      let mut reader = input.as_bytes().line_reader();
      reader.read_line_with_prompt(buffer, &mut writer).unwrap()
    });
  }

  #[test]
  fn test_prompt_writer_counter_with_line_reader()
  {
    let (pipe_reader, writer) = pipe().unwrap();
    let mut counter = -1;
    let mut writer = writer.prompt_writer_with(move || {
      counter += 1;
      format!("{counter}\n")
    });
    check_prompt_writer_counter(pipe_reader, move |buffer| {
      let input = input();
      let mut reader = input.as_bytes().line_reader();
      reader.read_line_with_prompt(buffer, &mut writer).unwrap()
    });
  }
}
