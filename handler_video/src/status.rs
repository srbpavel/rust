/// video status messages
#[derive(Debug)]
pub enum Status {
    Init,
    ClearOk,
    ListAll,
    ListNone,
    EmptyVideoId,
    EmptyGroup,
    EmptyFormFilename,
    EmptyFormName,
    //EmptyForms,
    TooManyForms,
    VideoIdFound,
    VideoIdNotFound,
    UploadDone,
    GroupFound,
    GroupNotFound,
    DeleteOk,
    DeleteDetailError,
    DeleteBinaryError,
    PlayerBinaryNotFound,
}

impl Status {
    // can have as &str but then full of lifetime, time will proof
    //pub fn as_str(&self) -> &str {
    pub fn as_string(&self) -> String {
        match *self {
            Self::Init => String::from("init"),
            Self::ClearOk => String::from("clear ok"),
            Self::ListAll => String::from("some videos found"),
            Self::ListNone => String::from("none videos found"),
            Self::EmptyVideoId => String::from("header 'video_id' not provided"),
            Self::EmptyGroup => String::from("header 'group' not provided"),
            Self::EmptyFormFilename => String::from("form 'filename' not provided"),
            Self::EmptyFormName => String::from("form 'name' not provided"),
            // curl with no form -F -> Multipart boundary is not found
            // status code 400
            //Self::EmptyForms => String::from("'form' not provided"),
            Self::TooManyForms => String::from("too many forms, we accept only one form"),
            Self::VideoIdFound => String::from("video_id found"),
            Self::VideoIdNotFound => String::from("video_id not found"),
            Self::UploadDone => String::from("upload finished"),
            Self::GroupFound => String::from("group found"),
            Self::GroupNotFound => String::from("group not found"),
            Self::DeleteOk => String::from("delete ok"),
            Self::DeleteDetailError => String::from("delete detail error"),
            Self::DeleteBinaryError => String::from("delete binary error"),
            Self::PlayerBinaryNotFound => String::from(
                "{\"status\": \"player binary_id not found\"}"
            ),
        }
    }
}
