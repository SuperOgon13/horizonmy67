// Заглушка для arboard под Android
pub struct Clipboard;
pub struct Get;
pub struct Set;
pub struct Clear;

impl Clipboard {
    pub fn new() -> Result<Self, Box<dyn std::error::Error>> { Ok(Self) }
}