pub enum Functions {
    GetUserData,
    GetAllCourses,
}
impl Functions {
    pub fn new(&self) -> &'static str {
        match self {
            Functions::GetUserData => "core_webservice_get_site_info&moodlewsrestformat=json",
            Functions::GetAllCourses => "core_enrol_get_users_courses&moodlewsrestformat=json",
        }
    }
}