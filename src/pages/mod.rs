// Страницы сайта. Каждая — свой файл + свой css в assets/css/<page>.css.
mod home;
mod legal;
mod service_detail;
mod services;

pub use home::Home;
pub use legal::{Privacy, Terms};
pub use service_detail::ServiceDetail;
pub use services::Services;
