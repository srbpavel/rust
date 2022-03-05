/// video status messages
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
    //UploadStarted,
    UploadDone,
    // GROUP
    GroupFound,
    GroupNotFound,
    //GroupsAvailable,
    //NoGroupsAvailable,
    // LIST
    // ...
    // UPDATE
    //UpdateOk,
    //UpdateError,
    // DELETE
    DeleteOk,
    DeleteError,
    DeleteInvalidId,
}

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
            Self::EmptyFormName => String::from("form 'name' not provided"),
            // curl with no form -F -> Multipart boundary is not found
            // status code 400
            //Self::EmptyForms => String::from("'form' not provided"),
            Self::TooManyForms => String::from("too many forms, we accept only one form"),
            Self::VideoIdFound => String::from("video_id found"),
            Self::VideoIdNotFound => String::from("video_id not found"),
            Self::VideoIdWrongFormat => String::from("video_id wrong format"),
            //Self::UploadStarted => String::from("upload started"),
            Self::UploadDone => String::from("upload finished"),
            Self::GroupFound => String::from("group found"),
            Self::GroupNotFound => String::from("group not found"),
            //Self::GroupsAvailable => String::from("some groups found"),
            //Self::NoGroupsAvailable => String::from("no groups found"),
            //Self::UpdateOk => String::from("update ok"),
            //Self::UpdateError => String::from("update error"),
            Self::DeleteOk => String::from("delete ok"),
            Self::DeleteError => String::from("delete error"),
            Self::DeleteInvalidId => String::from("delete invalid id"),
        }
    }
}
