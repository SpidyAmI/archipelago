use geometry::{Cell, MapError, Terrain};

pub struct GenerationGrid {
    cells: Vec<(Cell, Terrain)>, // A list of all the cells in the grid
    radius: u32,                 // The radius of the grid
    starts: Vec<usize>,          // Array with all the column starts
}

impl GenerationGrid {
    pub fn new(radius: u32) -> Result<Self, MapError> {
        if radius > (i32::MAX / 2) as u32 {
            // prevents overflow in start(), where operations like R - q can reach 2 * R
            return Err(MapError::ErrRadius);
        }
        let rad = i32::try_from(radius).map_err(|_| MapError::ErrRadius)?;

        // We use checked operations to see if it overflows
        let cells_len = radius
            .checked_add(1)
            .and_then(|n| n.checked_mul(radius))
            .and_then(|n| n.checked_mul(3))
            .and_then(|n| n.checked_add(1))
            .ok_or(MapError::ErrRadius)? as usize;

        let mut starts: Vec<usize> = Vec::with_capacity((2 * radius + 1) as usize);
        let mut cells: Vec<(Cell, Terrain)> = Vec::with_capacity(cells_len);

        // Calculate starts and fill the grid
        for q in (-rad)..=rad {
            // `q` is columns
            starts.push(cells.len()); // The `q`-th column starts at the position of the last added cell
            // We calculate how many cells should be in this column, and then we fill it
            let r_min = (-rad).max(-rad - q); // Other formulas needed to satisfy the math requisites
            let r_max = rad.min(rad - q); // Another one
            for r in r_min..=r_max {
                // We cycle over each column (2R + 1)
                cells.push((Cell { q, r }, Terrain::Clear)); // We fill it with `r` clear cells
            }
        }

        Ok(Self { radius, cells, starts })
    }

    pub fn cells(self) -> Vec<(Cell, Terrain)> {
        self.cells
    }

    const DIRS: [(i32, i32); 6] = [
        (1, 0),  // East
        (1, -1), // NorthEast
        (0, -1), // NorthWest
        (-1, 0), // West
        (-1, 1), // SouthWest
        (0, 1),
    ]; // SouthEast

    /// Given a cell coordinates, returns the index.
    /// If it's out of bounds, returns None.
    pub fn to_index(&self, cell: Cell) -> Option<usize> {
        // Checks the 3 basic conditions
        if cell.q.unsigned_abs() > self.radius
            || cell.r.unsigned_abs() > self.radius
            || cell.s().unsigned_abs() > self.radius
        {
            return None;
        }

        let radius = self.radius as i32; // Casting here so I don't have to cast later
        let column = (cell.q + radius) as usize; // q is the column
        let r_min = (-radius).max(-radius - cell.q); // We do this to keep following the rules and get the r value correctly
        let offset = (cell.r - r_min) as usize; // We calculate the position on the rows
        Some(self.starts[column] + offset) // We return the position on the column + the position on the row (if it exists)
    }

    /// Given an index, returns the cell coordinates.
    /// If it's out of bounds, returns None.
    pub fn to_cell(&self, index: usize) -> Option<Cell> {
        // We simply extract the cell from the vector
        self.cells.get(index).map(|(cell, _)| *cell)
    }

    /// Returns the terrain of the given cell.
    /// Returns None if out of bounds
    pub fn get(&self, cell: Cell) -> Option<Terrain> {
        // We safely extract the terrain
        self.to_index(cell).map(|index| self.cells[index].1)
    }

    /// Sets the terrain of the given cell to the given terrain
    /// Returns a boolean that describes success
    pub fn set(&mut self, cell: Cell, terrain: Terrain) -> bool {
        if let Some(index) = self.to_index(cell) {
            self.cells[index].1 = terrain;
            true
        } else {
            false
        }
    }

    /// Returns an iterator of all the cells around it
    pub fn neighbors(cell: Cell) -> impl Iterator<Item = Cell> {
        // Cell implements Copy, `move` copies into a new iterator
        Self::DIRS.into_iter().map(move |(dirs_q, dirs_r)| Cell {
            q: dirs_q + cell.q,
            r: dirs_r + cell.r,
        })
    }

    /// Returns an iterator over all the cells
    pub fn iter(&self) -> impl Iterator<Item = &(Cell, Terrain)> {
        self.cells.iter()
    }
}
