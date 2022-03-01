use std::{path::PathBuf,
          io::{
              Error,
              ErrorKind,
          },
};


/// directory status messages
/// very raw for now, will unit response/json/err msg/... when the right time comes
#[derive(Debug)]
pub enum DirStatus {
    DirNotFound,
    AccessPermission,
}

/// directory_status -> msg
///
impl DirStatus {
    // can have as &str but then full of lifetime, time will proof
    //pub fn as_str(&self) -> &str {
    pub fn as_string(&self,
                     reason: &str) -> String {

        match *self {
            Self::DirNotFound => format!(
                "Error: video_storage directory does not exists: {:?}",
                reason,
            ),
            Self::AccessPermission => format!(
                "Error: write permission to: {}",
                reason,
            ),
        }
    }
}


/// verify dir is available and we are allowed to write
pub fn verify_dir(storage: &PathBuf,
                  write: bool) -> Result<(), Error> {

    if !storage.exists() {
        return Err(
            Error::new(
                ErrorKind::Other,
                DirStatus::DirNotFound.as_string(
                    &format!("{:?}",
                             storage,
                    ),
                )
            )
        )
        
    } else {
        if write {
            // touch file -> delete later when proof enough
            match std::fs::File::create(storage.join("touch.verify")) {
                Ok(_) => {},
                Err(why) => {
                    return Err(
                        Error::new(
                            ErrorKind::Other,
                            DirStatus::AccessPermission.as_string(
                                &format!("{:?}\nREASON >>> {:?}",
                                         storage,
                                         why,
                                )
                            ),
                        )
                    )
                },
            }
        }
    }
    
    Ok(())
}
