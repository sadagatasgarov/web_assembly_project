// use models::{
//     school::FullSchool,
//     users::ResetForm,
// };
use moonlight::*;

// use msgs::{
//     classes::{ClassDownMsgs, ClassUpMsgs},
//     teachers::{TeacherDownMsgs, TeacherUpMsgs}, lectures::{LecturesUpMsg, LecturesDownMsg}, timetables::{TimetablesUpMsgs, TimetablesDownMsgs}, admin::{AdminDownMsgs, AdminUpMsgs}, messages::{MessagesUpMsgs, MessagesDownMsgs},
// };
pub type UserId = EntityId;




#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(crate = "serde")]
pub struct User {
    pub id: UserId,
    pub name: String,
    pub auth_token: AuthToken,
}


#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(crate = "serde")]
pub enum UpMsg {
    // ------ Auth ------
    Login { email: String, password: String },
    // ForgetPassword{email: String},
    // ResetPassword(ResetForm),
    // Signin { form: signin::SigninForm },
    // Logout,
    // AddSchool { name: String },
    // Register(String, String),
    // UpdateSchool(FullSchool),
    // GetSchool,
    // Classes(ClassUpMsgs),
    // Teachers(TeacherUpMsgs),
    // Lectures(LecturesUpMsg),
    // Timetables(TimetablesUpMsgs),
    // Messages(MessagesUpMsgs),
    // Admin(AdminUpMsgs)
}




#[derive(Debug, Serialize, Deserialize)]
#[serde(crate = "serde")]
pub enum DownMsg {
    // ------ Auth ------
    LoginError(String),
    LoggedIn(User),
    // SigninError(String),
    // Signin,
    // Register,
    // ResetPassword,
    // LoggedOut,
    // LoggedOutError(String),
    // Registered(User),
    // ResgiterErrors,
    // GetSchool { id: i32, name: String },
    // AddedSchool(School),
    // AddSchoolError(String),
    // UpdateSchool,
    // Auth(i32),
    // AuthError(String),
    // Classes(ClassDownMsgs),
    // Teachers(TeacherDownMsgs),
    // Lectures(LecturesDownMsg),
    // Timetables(TimetablesDownMsgs),
    // Messages(MessagesDownMsgs),
    // Admin(AdminDownMsgs)
}