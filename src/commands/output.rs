pub struct Output {
    pub stdout: String,
    pub stderr: String,
    pub is_success: bool,
}


impl Output {

    pub fn err(stderr: String) -> Self {
        Output { stdout: String::new(), stderr, is_success: false }
    }

    pub fn ok(stdout: String) -> Self {
        Output { stdout, stderr: String::new(), is_success: true }
    }

}