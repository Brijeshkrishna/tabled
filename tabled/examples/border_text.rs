//! This example demonstrates inserting text into the borders
//! of a [`Table`] with [`BorderText`]; a powerful labeling tool.
//!
//! * [`BorderText`] currently supports:
//!     * Horizontal border placement
//!     * Placement starting column offset
//!     * Text colorization
//!
//! * Note how the flexibility of [`Style`] is utilized
//! to remove horizontal borders from the table entirely,
//! and then granularly reinserts one for a highly customized
//! visualization.
//!
//! * Note how the [`Rows`] utility object is used to idiomatically
//! reference the first and last rows of a [`Table`] without writing
//! the necessary logic by hand.
//!
//! * 🚀 Combining several easy-to-use tools,
//! to create unique data representations is what makes [`tabled`] great!

use tabled::{
    settings::{
        object::Rows,
        style::{LineText, Style},
    },
    Table,
};

fn main() {
    let data = [[5, 6, 7, 8, 9], [10, 11, 12, 13, 14]];

    let table = Table::new(data)
        .with(
            Style::modern().remove_horizontal().horizontals([(
                1,
                Style::modern()
                    .get_horizontal_line()
                    .left(Style::modern().get_frame().get_left()),
            )]),
        )
        .with(LineText::new("Numbers").horizontal(Rows::first()).offset(1))
        .with(LineText::new("More numbers").horizontal(1).offset(1))
        .with(LineText::new("end.").horizontal(Rows::last()).offset(1))
        .to_string();

    println!("{table}");
}
