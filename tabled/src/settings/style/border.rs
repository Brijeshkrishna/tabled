use core::marker::PhantomData;

use crate::{
    grid::{
        config::{Border as GridBorder, ColoredConfig, Entity},
        records::{ExactRecords, Records},
    },
    settings::{style::On, CellOption},
};

/// Border represents a border of a Cell.
///
/// ```text
///                         top border
///                             |
///                             V
/// corner top left ------> +_______+  <---- corner top left
///                         |       |
/// left border ----------> |  cell |  <---- right border
///                         |       |
/// corner bottom right --> +_______+  <---- corner bottom right
///                             ^
///                             |
///                        bottom border
/// ```
///
/// ```rust,no_run
/// # use tabled::{Table, settings::{style::{Style, Border}, object::Rows}};
/// # let data: Vec<&'static str> = Vec::new();
/// let table = Table::new(&data)
///     .with(Style::ascii())
///     .modify(Rows::single(0), Border::new().set_top('x'));
/// ```
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Border<T, B, L, R> {
    inner: GridBorder<char>,
    _top: PhantomData<T>,
    _bottom: PhantomData<B>,
    _left: PhantomData<L>,
    _right: PhantomData<R>,
}

impl<T, B, L, R> Border<T, B, L, R> {
    pub(crate) const fn from_border(inner: GridBorder<char>) -> Border<T, B, L, R> {
        Border {
            inner,
            _top: PhantomData,
            _bottom: PhantomData,
            _left: PhantomData,
            _right: PhantomData,
        }
    }
}

impl Border<(), (), (), ()> {
    /// Creates an empty border.
    pub const fn new() -> Self {
        Self::from_border(GridBorder::empty())
    }
}

impl Border<On, On, On, On> {
    /// This function constructs a cell borders with all sides set.
    #[allow(clippy::too_many_arguments)]
    pub const fn full(
        top: char,
        bottom: char,
        left: char,
        right: char,
        top_left: char,
        top_right: char,
        bottom_left: char,
        bottom_right: char,
    ) -> Self {
        Border::from_border(GridBorder::full(
            top,
            bottom,
            left,
            right,
            top_left,
            top_right,
            bottom_left,
            bottom_right,
        ))
    }

    /// This function constructs a cell borders with all sides's char set to a given character.
    /// It behaves like [`Border::full`] with the same character set to each side.
    pub const fn filled(c: char) -> Self {
        Self::full(c, c, c, c, c, c, c, c)
    }

    /// Using this function you deconstruct the existing borders.
    pub const fn empty() -> EmptyBorder {
        EmptyBorder
    }
}

impl<T, B, L, R> Border<T, B, L, R> {
    /// Set a top border character.
    pub const fn set_top(mut self, c: char) -> Border<On, B, L, R> {
        self.inner.top = Some(c);
        Border::from_border(self.inner)
    }

    /// Set a bottom border character.
    pub const fn set_bottom(mut self, c: char) -> Border<T, On, L, R> {
        self.inner.bottom = Some(c);
        Border::from_border(self.inner)
    }

    /// Set a left border character.
    pub const fn set_left(mut self, c: char) -> Border<T, B, On, R> {
        self.inner.left = Some(c);
        Border::from_border(self.inner)
    }

    /// Set a right border character.
    pub const fn set_right(mut self, c: char) -> Border<T, B, L, On> {
        self.inner.right = Some(c);
        Border::from_border(self.inner)
    }

    /// Converts a border into a general data structure.
    pub const fn into_inner(self) -> GridBorder<char> {
        self.inner
    }
}

impl<T, B, L> Border<T, B, L, On> {
    /// Get a right character.
    pub const fn get_right(&self) -> char {
        get_char(self.inner.right)
    }
}

impl<T, B, R> Border<T, B, On, R> {
    /// Get a left character.
    pub const fn get_left(&self) -> char {
        get_char(self.inner.left)
    }
}

impl<B, L, R> Border<On, B, L, R> {
    /// Get a top character.
    pub const fn get_top(&self) -> char {
        get_char(self.inner.top)
    }
}

impl<T, L, R> Border<T, On, L, R> {
    /// Get a bottom character.
    pub const fn get_bottom(&self) -> char {
        get_char(self.inner.bottom)
    }
}

impl<B, R> Border<On, B, On, R> {
    /// Set a top left intersection character.
    pub const fn set_corner_top_left(mut self, c: char) -> Self {
        self.inner.left_top_corner = Some(c);
        self
    }

    /// Get a top left intersection character.
    pub const fn get_corner_top_left(&self) -> char {
        get_char(self.inner.left_top_corner)
    }
}

impl<B, L> Border<On, B, L, On> {
    /// Set a top right intersection character.
    pub const fn set_corner_top_right(mut self, c: char) -> Self {
        self.inner.right_top_corner = Some(c);
        self
    }

    /// Get a top right intersection character.
    pub const fn get_corner_top_right(&self) -> char {
        get_char(self.inner.right_top_corner)
    }
}

impl<T, R> Border<T, On, On, R> {
    /// Set a bottom left intersection character.
    pub const fn set_corner_bottom_left(mut self, c: char) -> Self {
        self.inner.left_bottom_corner = Some(c);
        self
    }

    /// Get a bottom left intersection character.
    pub const fn get_corner_bottom_left(&self) -> char {
        get_char(self.inner.left_bottom_corner)
    }
}

impl<T, L> Border<T, On, L, On> {
    /// Set a bottom right intersection character.
    pub const fn set_corner_bottom_right(mut self, c: char) -> Self {
        self.inner.right_bottom_corner = Some(c);
        self
    }

    /// Get a bottom left intersection character.
    pub const fn get_corner_bottom_right(&self) -> char {
        get_char(self.inner.right_bottom_corner)
    }
}

impl<T, B, L, R> From<Border<T, B, L, R>> for GridBorder<char> {
    fn from(value: Border<T, B, L, R>) -> Self {
        value.inner
    }
}

impl<T, B, L, R, Data> CellOption<Data, ColoredConfig> for Border<T, B, L, R>
where
    Data: Records + ExactRecords,
{
    fn change(self, records: &mut Data, cfg: &mut ColoredConfig, entity: Entity) {
        CellOption::change(self.inner, records, cfg, entity)
    }
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct EmptyBorder;

impl<R> CellOption<R, ColoredConfig> for EmptyBorder
where
    R: Records + ExactRecords,
{
    fn change(self, records: &mut R, cfg: &mut ColoredConfig, entity: Entity) {
        let shape = (records.count_rows(), records.count_columns());

        for pos in entity.iter(shape.0, shape.1) {
            cfg.remove_border(pos, shape);
        }
    }
}

const fn get_char(c: Option<char>) -> char {
    match c {
        Some(c) => c,
        None => unreachable!(),
    }
}

impl<R> CellOption<R, ColoredConfig> for GridBorder<char>
where
    R: Records + ExactRecords,
{
    fn change(self, records: &mut R, cfg: &mut ColoredConfig, entity: Entity) {
        let shape = (records.count_rows(), records.count_columns());

        for pos in entity.iter(shape.0, shape.1) {
            cfg.set_border(pos, self);
        }
    }
}
