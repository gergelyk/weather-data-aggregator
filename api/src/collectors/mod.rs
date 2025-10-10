pub mod aemet;
pub mod common;
pub mod meteocat;
pub mod meteoclimatic;
pub mod openwindmap;
pub mod weatherlink;

pub use aemet::AemetDownloader;
pub use meteocat::MeteocatDownloader;
pub use meteoclimatic::MeteoclimaticDownloader;
pub use openwindmap::OpenWindMapDownloader;
pub use weatherlink::WeatherlinkDownloader;

pub use common::Downloader;
