//! The file transport writes the emails to the given directory. The name of the file will be
//! `message_id.txt`.
//! It can be useful for testing purposes, or if you want to keep track of sent messages.
//!
//! #### File Transport
//!
//! The file transport writes the emails to the given directory. The name of the file will be
//! `message_id.json`.
//! It can be useful for testing purposes, or if you want to keep track of sent messages.
//!
//! ```rust
//! # #[cfg(feature = "file-transport")]
//! # {
//! use std::env::temp_dir;
//! use lettre::{Transport, Envelope, Message, FileTransport};
//!
//! // Write to the local temp directory
//! let sender = FileTransport::new(temp_dir());
//! let email = Message::builder()
//!     .from("NoBody <nobody@domain.tld>".parse().unwrap())
//!     .reply_to("Yuin <yuin@domain.tld>".parse().unwrap())
//!     .to("Hei <hei@domain.tld>".parse().unwrap())
//!     .subject("Happy new year")
//!     .body("Be happy!")
//!     .unwrap();
//!
//! let result = sender.send(&email);
//! assert!(result.is_ok());
//! # }
//! ```
//!
//! Example result in `/tmp/b7c211bc-9811-45ce-8cd9-68eab575d695.json`:
//!
//! ```json
//! TODO
//! ```

use crate::{transport::file::error::Error, Envelope, Transport};
use std::{
    fs::File,
    io::prelude::*,
    path::{Path, PathBuf},
    str,
};
use uuid::Uuid;

pub mod error;

type Id = String;

/// Writes the content and the envelope information to a file
#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct FileTransport {
    path: PathBuf,
}

impl FileTransport {
    /// Creates a new transport to the given directory
    pub fn new<P: AsRef<Path>>(path: P) -> FileTransport {
        FileTransport {
            path: PathBuf::from(path.as_ref()),
        }
    }
}

#[derive(PartialEq, Eq, Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
struct SerializableEmail<'a> {
    envelope: Envelope,
    raw_message: Option<&'a [u8]>,
    message: Option<&'a str>,
}

impl Transport for FileTransport {
    type Ok = Id;
    type Error = Error;

    fn send_raw(&self, envelope: &Envelope, email: &[u8]) -> Result<Self::Ok, Self::Error> {
        let email_id = Uuid::new_v4();

        let mut file = self.path.clone();
        file.push(format!("{}.json", email_id));

        let serialized = match str::from_utf8(email) {
            // Serialize as UTF-8 string if possible
            Ok(m) => serde_json::to_string(&SerializableEmail {
                envelope: envelope.clone(),
                message: Some(m),
                raw_message: None,
            }),
            Err(_) => serde_json::to_string(&SerializableEmail {
                envelope: envelope.clone(),
                message: None,
                raw_message: Some(email),
            }),
        }?;

        File::create(file.as_path())?.write_all(serialized.as_bytes())?;
        Ok(email_id.to_string())
    }
}