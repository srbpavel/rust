/// video status messages
/// very raw for now, will unit/divide when the right time comes
#[derive(Debug)]
pub enum Status {
    Init,
    StatusOk,
    // HEADERS
    EmptyVideoId,
    EmptyGroupId,
    // FORMS
    EmptyFormFilename,
    EmptyFormName,
    //EmptyForms,
    TooManyForms,
    // VIDEO
    VideoIdFound,
    VideoIdNotFound,
    VideoIdWrongFormat,
    FileNotFound,
    UploadStarted,
    UploadDone,
    // GROUP
    GroupFound,
    GroupNotFound,
    GroupsAvailable,
    NoGroupsAvailable,
    // LIST
    // ...
    // UPDATE
    UpdateOk,
    // DELETE
    DeleteOk,
    DeleteError,
    DeleteInvalidId,
    // FUTURE USE
    //AccessPermission
    //NotEnoughSpace
}

/// video_status -> msg
///
impl Status {
    // can have as &str but then full of lifetime, time will proof
    //pub fn as_str(&self) -> &str {
    pub fn as_string(&self) -> String {
        match *self {
            Self::Init => String::from("init"),
            Self::StatusOk => String::from("ok"),

            Self::EmptyVideoId => String::from("header 'video_id' not provided"),
            Self::EmptyGroupId => String::from("header 'group' not provided"),

            Self::EmptyFormFilename => String::from("form 'filename' not provided"),
            Self::EmptyFormName => String::from("'name' not provided for form"),
            // curl with no form -F -> Multipart boundary is not found
            // status code 400
            //Self::EmptyForms => String::from("'form' not provided"),
            Self::TooManyForms => String::from("too many forms, we accept only one form"),

            Self::VideoIdFound => String::from("video found"),
            
            Self::VideoIdNotFound => String::from("video_id not found"),
            Self::VideoIdWrongFormat => String::from("video_id wrong format"),
            Self::FileNotFound => String::from("file not found"),
            Self::UploadStarted => String::from("upload started"),
            Self::UploadDone => String::from("upload finished"),
            
            Self::GroupFound => String::from("group found"),
            Self::GroupNotFound => String::from("group not found"),
            Self::GroupsAvailable => String::from("some groups found"),
            Self::NoGroupsAvailable => String::from("no groups found"),
            
            Self::UpdateOk => String::from("update ok"),
            
            Self::DeleteOk => String::from("delete ok"),
            Self::DeleteError => String::from("delete error"),
            Self::DeleteInvalidId => String::from("delete invalid id"),
        }
    }
}
