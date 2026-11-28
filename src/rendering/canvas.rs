use crate::common::{bitset::Bitset, cell::Cell, position::Position, size::Size};

pub struct Canvas {
    size: Size,
    cells: Vec<Cell>,
    delta: Bitset,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Delta {
    position: Position,
    cell: Cell,
}

impl Delta {
    #[inline]
    pub const fn new(position: Position, cell: Cell) -> Self {
        Self { position, cell }
    }

    #[inline]
    pub const fn position(&self) -> Position {
        self.position
    }

    #[inline]
    pub const fn cell(&self) -> Cell {
        self.cell
    }
}

impl Canvas {
    #[inline]
    pub fn new(size: Size) -> Self {
        let area = size.area() as usize;

        Self {
            size,
            cells: vec![Cell::EMPTY; area],
            delta: Bitset::new(area),
        }
    }

    #[inline]
    pub fn resize(&mut self, new_size: Size) {
        if new_size == self.size {
            return;
        }

        let area = new_size.area() as usize;

        self.cells.clear();
        self.cells.resize(area, Cell::EMPTY);
        self.size = new_size;
        self.delta = Bitset::new(area);
    }

    #[inline]
    pub fn size(&self) -> Size {
        self.size
    }

    #[inline]
    pub fn cells(&self) -> &[Cell] {
        &self.cells
    }

    #[inline]
    pub fn cells_mut(&mut self) -> &mut [Cell] {
        self.delta.set_all();
        &mut self.cells
    }

    #[inline]
    pub fn rows(&self) -> impl ExactSizeIterator<Item = &[Cell]> {
        self.cells.chunks_exact(self.width())
    }

    #[inline]
    pub fn rows_mut(&mut self) -> impl ExactSizeIterator<Item = &mut [Cell]> {
        let width = self.width();

        self.delta.set_all();
        self.cells.chunks_exact_mut(width)
    }

    #[inline]
    pub fn indexed_rows(&self) -> impl Iterator<Item = (u16, &[Cell])> {
        let width = self.width();
        (0..self.size.height()).zip(self.cells.chunks_exact(width))
    }

    #[inline]
    pub fn indexed_rows_mut(&mut self) -> impl Iterator<Item = (u16, &mut [Cell])> {
        let width = self.width();
        let height = self.size.height();

        self.delta.set_all();
        (0..height).zip(self.cells.chunks_exact_mut(width))
    }

    #[inline]
    pub fn row(&self, y: u16) -> Option<&[Cell]> {
        let width = self.width();
        let start = y as usize * width;

        self.cells.get(start..start + width)
    }

    #[inline]
    pub fn row_mut(&mut self, y: u16) -> Option<&mut [Cell]> {
        if y >= self.size.height() {
            return None;
        }

        let width = self.width();
        let start = y as usize * width;

        self.delta.set_range(start, start + width);
        self.cells.get_mut(start..start + width)
    }

    #[inline]
    pub fn get(&self, position: Position) -> Option<&Cell> {
        self.row(position.y())?.get(position.x() as usize)
    }

    #[inline]
    pub fn get_mut(&mut self, position: Position) -> Option<&mut Cell> {
        if !self.size.contains(position) {
            return None;
        }

        let index = position.y() as usize * self.width() + position.x() as usize;

        self.delta.set_bit(index);
        self.cells.get_mut(index)
    }

    #[inline]
    pub fn set(&mut self, position: Position, cell: Cell) {
        if !self.size.contains(position) {
            return;
        }

        let index = position.y() as usize * self.width() + position.x() as usize;

        if self.cells[index] == cell {
            return;
        }

        self.cells[index] = cell;
        self.delta.set_bit(index);
    }

    #[inline]
    pub fn fill_row(&mut self, y: u16, cell: Cell) {
        if y >= self.size.height() {
            return;
        }

        let width = self.width();
        let row_start = y as usize * width;

        for (x, existing) in self.cells[row_start..row_start + width]
            .iter_mut()
            .enumerate()
        {
            if *existing != cell {
                *existing = cell;
                self.delta.set_bit(row_start + x);
            }
        }
    }

    #[inline]
    pub fn fill_span(&mut self, y: u16, start: u16, end: u16, cell: Cell) {
        if y >= self.size.height() {
            return;
        }

        let width = self.width();
        let start = (start as usize).min(width);
        let end = (end as usize).min(width);

        if start >= end {
            return;
        }

        let row_start = y as usize * width;

        for x in start..end {
            let index = row_start + x;

            if self.cells[index] != cell {
                self.cells[index] = cell;
                self.delta.set_bit(index);
            }
        }
    }

    #[inline]
    pub fn fill(&mut self, fill: Cell) {
        let width = self.width();
        let height = self.size.height() as usize;

        for y in 0..height {
            let row_start = y * width;

            for index in row_start..row_start + width {
                if self.cells[index] != fill {
                    self.cells[index] = fill;
                    self.delta.set_bit(index);
                }
            }
        }
    }

    #[inline]
    pub fn clear(&mut self) {
        self.fill(Cell::EMPTY);
    }

    #[inline]
    pub fn fill_with(&mut self, mut source: impl FnMut(Position) -> Cell) {
        let width = self.size.width();
        let height = self.size.height();

        for y in 0..height {
            let row_start = y as usize * width as usize;

            for x in 0..width {
                let index = row_start + x as usize;
                let cell = source(Position::new(x, y));

                if self.cells[index] != cell {
                    self.cells[index] = cell;
                    self.delta.set_bit(index);
                }
            }
        }
    }

    #[inline]
    pub fn delta(&self) -> impl Iterator<Item = Delta> + '_ {
        let width = self.width();

        (0..self.size.height()).flat_map(move |y| {
            let row_start = y as usize * width;

            self.delta
                .iter_ones_range(row_start, row_start + width)
                .map(move |index| {
                    Delta::new(
                        Position::new((index - row_start) as u16, y),
                        self.cells[index],
                    )
                })
        })
    }

    #[inline]
    pub fn clear_delta(&mut self) {
        self.delta.clear_all();
    }

    #[inline]
    fn width(&self) -> usize {
        self.size.width() as usize
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::color::Color;

    const A: Cell = Cell::new(Color::BLACK, Color::BLACK, b'a');

    fn size(width: u16, height: u16) -> Size {
        Size::new(width, height)
    }

    fn positions(canvas: &Canvas) -> Vec<Position> {
        canvas.delta().map(|delta| delta.position()).collect()
    }

    fn all_positions(width: u16, height: u16) -> Vec<Position> {
        (0..height)
            .flat_map(|y| (0..width).map(move |x| Position::new(x, y)))
            .collect()
    }

    #[test]
    fn new_canvas_has_no_damage() {
        let canvas = Canvas::new(size(4, 3));

        assert_eq!(canvas.delta().count(), 0);
    }

    #[test]
    fn set_marks_only_the_changed_cell() {
        let mut canvas = Canvas::new(size(4, 3));

        canvas.set(Position::new(1, 1), A);

        assert_eq!(positions(&canvas), [Position::new(1, 1)]);
    }

    #[test]
    fn setting_the_same_cell_marks_nothing() {
        let mut canvas = Canvas::new(size(4, 3));
        canvas.set(Position::new(1, 1), A);
        canvas.clear_delta();

        canvas.set(Position::new(1, 1), A);

        assert_eq!(canvas.delta().count(), 0);
    }

    #[test]
    fn set_out_of_bounds_is_ignored() {
        let mut canvas = Canvas::new(size(4, 3));

        canvas.set(Position::new(4, 0), A);
        canvas.set(Position::new(0, 3), A);

        assert_eq!(canvas.delta().count(), 0);
    }

    #[test]
    fn fill_marks_every_changed_cell() {
        let mut canvas = Canvas::new(size(3, 2));

        canvas.fill(A);

        assert_eq!(positions(&canvas), all_positions(3, 2));
    }

    #[test]
    fn filling_with_the_same_value_marks_nothing() {
        let mut canvas = Canvas::new(size(3, 2));

        canvas.fill(Cell::EMPTY);

        assert_eq!(canvas.delta().count(), 0);
    }

    #[test]
    fn clear_marks_only_the_changed_cells() {
        let mut canvas = Canvas::new(size(3, 2));
        canvas.fill(A);
        canvas.clear_delta();

        canvas.clear();

        assert_eq!(positions(&canvas), all_positions(3, 2));
    }

    #[test]
    fn fill_row_marks_only_that_row() {
        let mut canvas = Canvas::new(size(3, 2));

        canvas.fill_row(1, A);

        assert_eq!(
            positions(&canvas),
            [
                Position::new(0, 1),
                Position::new(1, 1),
                Position::new(2, 1)
            ]
        );
    }

    #[test]
    fn fill_span_marks_only_the_span() {
        let mut canvas = Canvas::new(size(4, 2));

        canvas.fill_span(0, 1, 3, A);

        assert_eq!(
            positions(&canvas),
            [Position::new(1, 0), Position::new(2, 0)]
        );
    }

    #[test]
    fn fill_span_clamps_to_the_row() {
        let mut canvas = Canvas::new(size(4, 2));

        canvas.fill_span(0, 3, 100, A);

        assert_eq!(positions(&canvas), [Position::new(3, 0)]);
    }

    #[test]
    fn fill_with_marks_only_changed_cells() {
        let mut canvas = Canvas::new(size(2, 2));

        canvas.fill_with(|position| if position.x() == 0 { A } else { Cell::EMPTY });

        assert_eq!(
            positions(&canvas),
            [Position::new(0, 0), Position::new(0, 1)]
        );
    }

    #[test]
    fn delta_is_row_major() {
        let mut canvas = Canvas::new(size(3, 2));

        canvas.set(Position::new(2, 1), A);
        canvas.set(Position::new(0, 0), A);
        canvas.set(Position::new(1, 1), A);

        assert_eq!(
            positions(&canvas),
            [
                Position::new(0, 0),
                Position::new(1, 1),
                Position::new(2, 1)
            ]
        );
    }

    #[test]
    fn delta_reports_the_current_cell() {
        let mut canvas = Canvas::new(size(2, 1));
        canvas.set(Position::new(0, 0), A);

        let deltas: Vec<_> = canvas.delta().collect();

        assert_eq!(deltas.len(), 1);
        assert_eq!(deltas[0].position(), Position::new(0, 0));
        assert_eq!(deltas[0].cell(), A);
    }

    #[test]
    fn a_two_by_two_painter_emits_four_cells() {
        let mut canvas = Canvas::new(size(4, 4));

        for y in 1..3 {
            for x in 1..3 {
                canvas.set(Position::new(x, y), A);
            }
        }

        assert_eq!(
            positions(&canvas),
            [
                Position::new(1, 1),
                Position::new(2, 1),
                Position::new(1, 2),
                Position::new(2, 2),
            ]
        );
    }

    #[test]
    fn clear_delta_resets_tracking() {
        let mut canvas = Canvas::new(size(3, 2));
        canvas.fill(A);

        canvas.clear_delta();

        assert_eq!(canvas.delta().count(), 0);
    }

    #[test]
    fn resize_blanks_the_canvas_and_drops_damage() {
        let mut canvas = Canvas::new(size(2, 2));
        canvas.fill(A);

        canvas.resize(size(4, 3));

        assert_eq!(canvas.size(), size(4, 3));
        assert_eq!(canvas.delta().count(), 0);
        assert!(canvas.cells().iter().all(|&cell| cell == Cell::EMPTY));
    }

    #[test]
    fn get_mut_marks_its_cell() {
        let mut canvas = Canvas::new(size(3, 3));

        let _ = canvas.get_mut(Position::new(1, 2));

        assert_eq!(positions(&canvas), [Position::new(1, 2)]);
    }

    #[test]
    fn row_mut_marks_its_row() {
        let mut canvas = Canvas::new(size(3, 3));

        let _ = canvas.row_mut(1);

        assert_eq!(
            positions(&canvas),
            [
                Position::new(0, 1),
                Position::new(1, 1),
                Position::new(2, 1)
            ]
        );
    }

    #[test]
    fn cells_mut_marks_every_cell() {
        let mut canvas = Canvas::new(size(3, 2));

        let _ = canvas.cells_mut();

        assert_eq!(canvas.delta().count(), 6);
    }

    #[test]
    fn rows_mut_marks_every_cell() {
        let mut canvas = Canvas::new(size(3, 2));

        let _ = canvas.rows_mut().count();

        assert_eq!(canvas.delta().count(), 6);
    }
}
