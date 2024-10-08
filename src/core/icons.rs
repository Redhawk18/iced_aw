//! The default icon font of the widget of this library.

use cfg_if::cfg_if;
use iced::Font;

cfg_if! {
    if #[cfg(feature = "icons")] {
        #[deprecated(since="0.10.0", note="please use `iced_fonts` crate instead")]
        pub mod bootstrap;
        #[deprecated(since="0.10.0", note="please use `iced_fonts` crate instead")]
        pub mod nerd;

        #[deprecated(since="0.10.0", note="please use `iced_fonts` crate instead")]
        pub use bootstrap::{Bootstrap};
        #[deprecated(since="0.10.0", note="please use `iced_fonts` crate instead")]
        pub use nerd::Nerd;
        #[deprecated(since="0.10.0", note="please use `iced_fonts` crate instead")]
        /// The default icon font bytes for loading the font into iced.
        pub const BOOTSTRAP_FONT_BYTES: &[u8] = include_bytes!("./fonts/bootstrap-icons.ttf");
        #[deprecated(since="0.10.0", note="please use `iced_fonts` crate instead")]
        /// the icon font that has all nerd fonts.
        pub const NERD_FONT_BYTES: &[u8] = include_bytes!("./fonts/nerd-icons.ttf");

        #[deprecated(since="0.10.0", note="please use `iced_fonts` crate instead")]
        /// The bootstrap icon font.
        pub const BOOTSTRAP_FONT: Font = Font::with_name("bootstrap-icons");
        #[deprecated(since="0.10.0", note="please use `iced_fonts` crate instead")]
        /// The nerd icon font.
        pub const NERD_FONT: Font = Font::with_name("Symbols Nerd Font");
    } else {
        #[path = "icons/required.rs"]
        pub mod bootstrap;
        pub use bootstrap::{Bootstrap};
        // pub use required::{Bootstrap, icon_to_char, icon_to_string};
        /// The default icon font bytes for loading the font into iced.
        pub const BOOTSTRAP_FONT_BYTES: &[u8] = include_bytes!("./fonts/required-icons.ttf");
        /// The default icon font.
        pub const BOOTSTRAP_FONT: Font = Font::with_name("required-icons");
    }

}
