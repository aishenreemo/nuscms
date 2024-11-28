pub mod background;
pub mod button;
pub mod header;
pub mod icon;
pub mod logo;
pub mod title;
pub mod footer;

pub mod prelude {
    pub use super::background::Background;
    pub use super::button::Button;
    pub use super::button::RedirectButton;
    pub use super::header::Header;
    pub use super::icon::MaterialIcon;
    pub use super::logo::Logo;
    pub use super::title::Title;
}
