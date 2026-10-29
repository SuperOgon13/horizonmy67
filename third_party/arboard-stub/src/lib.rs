//! Заглушка `arboard` для целей без системного буфера обмена (Android).
//!
//! Повторяет публичный API arboard 3.6, чтобы зависимые крейты собирались.
//! Системный буфер не трогаем: текст хранится в памяти процесса, поэтому
//! «копировать / вставить» внутри самой игры работает. Картинки, HTML и
//! списки файлов не поддерживаются.

use std::borrow::Cow;
use std::fmt;
use std::path::{Path, PathBuf};
use std::sync::Mutex;

static TEXT: Mutex<Option<String>> = Mutex::new(None);

fn store(text: Option<String>) {
    if let Ok(mut guard) = TEXT.lock() {
        *guard = text;
    }
}

fn load() -> Option<String> {
    TEXT.lock().ok().and_then(|guard| guard.clone())
}

/// Ошибки буфера обмена (те же варианты, что у arboard).
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum Error {
    ContentNotAvailable,
    ClipboardNotSupported,
    ClipboardOccupied,
    ConversionFailure,
    Unknown { description: String },
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::ContentNotAvailable => {
                f.write_str("The clipboard contents were not available in the requested format or the clipboard is empty.")
            }
            Error::ClipboardNotSupported => {
                f.write_str("The selected clipboard is not supported with the current system configuration.")
            }
            Error::ClipboardOccupied => {
                f.write_str("The native clipboard is not accessible due to being held by another party.")
            }
            Error::ConversionFailure => {
                f.write_str("The data could not be converted to or from the clipboard format.")
            }
            Error::Unknown { description } => {
                write!(f, "Unknown error while interacting with the clipboard: {description}")
            }
        }
    }
}

impl std::error::Error for Error {}

/// Картинка в формате RGBA8.
#[derive(Debug, Clone)]
pub struct ImageData<'a> {
    pub width: usize,
    pub height: usize,
    pub bytes: Cow<'a, [u8]>,
}

impl ImageData<'_> {
    pub fn to_owned_img(&self) -> ImageData<'static> {
        ImageData {
            width: self.width,
            height: self.height,
            bytes: Cow::Owned(self.bytes.clone().into_owned()),
        }
    }
}

#[derive(Debug)]
pub struct Clipboard {
    _private: (),
}

impl Clipboard {
    pub fn new() -> Result<Self, Error> {
        Ok(Self { _private: () })
    }

    pub fn get_text(&mut self) -> Result<String, Error> {
        load().ok_or(Error::ContentNotAvailable)
    }

    pub fn set_text<'a, T: Into<Cow<'a, str>>>(&mut self, text: T) -> Result<(), Error> {
        store(Some(text.into().into_owned()));
        Ok(())
    }

    pub fn set_html<'a, T: Into<Cow<'a, str>>>(
        &mut self,
        _html: T,
        alt_text: Option<T>,
    ) -> Result<(), Error> {
        store(alt_text.map(|t| t.into().into_owned()));
        Ok(())
    }

    pub fn get_image(&mut self) -> Result<ImageData<'static>, Error> {
        Err(Error::ContentNotAvailable)
    }

    pub fn set_image(&mut self, _image: ImageData<'_>) -> Result<(), Error> {
        Err(Error::ClipboardNotSupported)
    }

    pub fn clear(&mut self) -> Result<(), Error> {
        store(None);
        Ok(())
    }

    pub fn get(&mut self) -> Get<'_> {
        Get { _clip: self }
    }

    pub fn set(&mut self) -> Set<'_> {
        Set { _clip: self }
    }

    pub fn clear_with(&mut self) -> Clear<'_> {
        Clear { _clip: self }
    }
}

pub struct Get<'clip> {
    _clip: &'clip mut Clipboard,
}

impl Get<'_> {
    pub fn text(self) -> Result<String, Error> {
        load().ok_or(Error::ContentNotAvailable)
    }

    pub fn image(self) -> Result<ImageData<'static>, Error> {
        Err(Error::ContentNotAvailable)
    }

    pub fn html(self) -> Result<String, Error> {
        Err(Error::ContentNotAvailable)
    }

    pub fn file_list(self) -> Result<Vec<PathBuf>, Error> {
        Err(Error::ContentNotAvailable)
    }
}

pub struct Set<'clip> {
    _clip: &'clip mut Clipboard,
}

impl Set<'_> {
    pub fn text<'a, T: Into<Cow<'a, str>>>(self, text: T) -> Result<(), Error> {
        store(Some(text.into().into_owned()));
        Ok(())
    }

    pub fn html<'a, T: Into<Cow<'a, str>>>(
        self,
        _html: T,
        alt_text: Option<T>,
    ) -> Result<(), Error> {
        store(alt_text.map(|t| t.into().into_owned()));
        Ok(())
    }

    pub fn image(self, _image: ImageData<'_>) -> Result<(), Error> {
        Err(Error::ClipboardNotSupported)
    }

    pub fn file_list(self, _file_list: &[impl AsRef<Path>]) -> Result<(), Error> {
        Err(Error::ClipboardNotSupported)
    }
}

pub struct Clear<'clip> {
    _clip: &'clip mut Clipboard,
}

impl Clear<'_> {
    pub fn clear(self) -> Result<(), Error> {
        store(None);
        Ok(())
    }
}
