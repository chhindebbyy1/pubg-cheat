pub struct Settings {
    pub aimbot_enabled: bool,
    pub esp_enabled: bool,
}

impl Settings {
    pub fn new() -> Self {
        Settings {
            aimbot_enabled: true,
            esp_enabled: true,
        }
    }
}