use super::*;

/// Struct representing a grayscale heightmap
#[derive(Clone)]
pub struct HeightMap {
    width: usize,
    height: usize,
    data: Vec<u16>,
}

impl HeightMap {
    /// Create a new all zeros heightmap.
    pub fn new(width: usize, height: usize) -> Self {
        HeightMap {
            width,
            height,
            data: vec![0; width * height],
        }
    }

    /// Getter for heightmaps width.
    pub fn width(&self) -> usize {
        self.width
    }
    /// Getter for heightmaps height (As in it's dimensions).
    pub fn height(&self) -> usize {
        self.height
    }
    /// Gets a reference to the internal data values.
    pub fn data(&self) -> &Vec<u16> {
        &self.data
    }

    /// Sets the data to a difference vector of height values.
    /// Returns Err if the vector is the incorrect length.
    pub fn set_data(&mut self, data: Vec<u16>) -> Result<(), ()> {
        if data.len() == self.width * self.height {
            self.data = data;
            Ok(())
        } else {
            Err(())
        }
    }

    /// Gets the height of a cell at the given coordinate.
    /// Returns None if coordinate is out of bounds.
    pub fn height_at(&self, x: usize, y: usize) -> Option<u16> {
        if x < self.width && y < self.height {
            Some(self.data[x + y * self.width])
        } else {
            None
        }
    }

    /// Sets the value of the cell at the given position.
    /// Returns an Err if coordinate is out of bounds.
    pub fn set_height_at(&mut self, x: usize, y: usize, v: u16) -> Result<(), String> {
        if x < self.width && y < self.height {
            self.data[x + y * self.width] = v;
            Ok(())
        } else {
            Err(String::from("Coordinate out of bounds"))
        }
    }

    /// Returns the orthogonally adjacent cells of the given coordinate.
    /// Returns as a list of pairs of coordinates and values.
    pub fn orthogonal_neighbours(&self, x: usize, y: usize) -> Vec<([usize; 2], u16)> {
        let mut neighbours = Vec::new();

        for o in [[-1, 0], [1, 0], [0, -1], [0, 1]] {
            let x = (x as isize) + o[0];
            let y = (y as isize) + o[1];

            if x < 0 || y < 0 {
                continue;
            }

            let x = x as usize;
            let y = y as usize;

            if x >= self.width || y >= self.height {
                continue;
            }

            neighbours.push((
                [x as usize, y as usize],
                self.height_at(x as usize, y as usize).unwrap(),
            ));
        }

        return neighbours;
    }

    /// Returns the diagonal neighbours of a given cell.
    /// See orthogonal_neighbours.
    pub fn diagonal_neighbours(&self, x: usize, y: usize) -> Vec<([usize; 2], u16)> {
        let mut neighbours = Vec::new();

        for o in [[-1, -1], [-1, 1], [1, -1], [1, 1]] {
            let x = (x as isize) + o[0];
            let y = (y as isize) + o[1];

            if x < 0 || y < 0 {
                continue;
            }

            let x = x as usize;
            let y = y as usize;

            if x >= self.width || y >= self.height {
                continue;
            }

            neighbours.push((
                [x as usize, y as usize],
                self.height_at(x as usize, y as usize).unwrap(),
            ));
        }

        return neighbours;
    }

    /// Returns the union of diagonal_neighbours and orthogonal_neighbours.
    pub fn neighbours(&self, x: usize, y: usize) -> Vec<([usize; 2], u16)> {
        return self
            .orthogonal_neighbours(x, y)
            .into_iter()
            .chain(self.diagonal_neighbours(x, y).into_iter())
            .collect();
    }

    /// Returns the direction a given cell faces (North facing, South facing .etc)
    /// Returns None if coordinate is out of range
    pub fn surface_normal(&self, x: usize, y: usize) -> Option<Vector<2>> {
        return if x <= 0 || x >= self.width() - 1 || y <= 0 || y >= self.height() - 1 {
            None
        } else {
            let l = self.height_at(x - 1, y)? as f64;
            let r = self.height_at(x + 1, y)? as f64;
            let t = self.height_at(x, y - 1)? as f64;
            let b = self.height_at(x, y + 1)? as f64;

            let dx = (r - l) / 2.0;
            let dy = (b - t) / 2.0;

            Some(Vector::from([-dx, -dy]).normalise())
        }
    }
}

impl From<Image> for HeightMap {
    fn from(image: Image) -> Self {
        Self {
            width: image.width(),
            height: image.height(),
            data: image
                .data()
                .iter()
                .map(|c| (c[0] * u16::MAX as f64).round() as u16)
                .collect(),
        }
    }
}
