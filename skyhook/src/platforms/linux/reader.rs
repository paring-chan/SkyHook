use cancellation::CancellationTokenSource;

use crate::types::Error;

#[derive(Debug)]
pub struct InputReader {
    pub file_path: String,
    cts: Option<CancellationTokenSource>,
}

impl InputReader {
    pub fn new(file_path: String) -> InputReader {
        InputReader {
            file_path: file_path,
            cts: None,
        }
    }

    pub fn start(&self) -> Result<(), Error> {
        if let Some(_) = self.cts {
            return Err(Error {
                message: String::from("Listener is already running"),
            });
        }

        Ok(())
    }

    pub fn stop(&mut self) {
        let cts = match self.cts {
            None
        };
    }
}
