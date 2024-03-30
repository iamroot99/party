pub struct RunReport {
    pub message: String,
    pub success: bool,
}

impl RunReport {
    pub fn new_success(message: String) -> Self {
        Self {
            message,
            success: true,
        }
    }

    pub fn new_failed(message: String) -> Self {
        Self {
            message,
            success: false,
        }
    }
}
