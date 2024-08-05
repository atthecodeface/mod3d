//a Imports
use std::ops::Range;

use index_vec::{index_vec, IndexVec};

//tt RegionData
/// A trait that a data for a region must satisfy
pub trait RegionData: Sized + Clone + std::fmt::Debug {}

//ip RegionData for T
impl<T> RegionData for T where T: Sized + Clone + std::fmt::Debug {}

//a RegionIndex
//tp RegionIndex
/// Create an index into the regions of a Rectangle
index_vec::define_index_type! {
    pub struct RegionIndex = u32;
}

//tp Rectangle
/// Struct that represents a rectangle broken up into a grid
/// subrectangles with fixes X and Y cuts
#[derive(Debug)]
pub struct Rectangle<R: RegionData> {
    // coords (x, then y) of boundaries of the rectangle
    //
    // the vec's are kept sorted
    coords: (Vec<isize>, Vec<isize>),
    // of size (xcoords.len()-1) * (ycoords.len()-1)
    regions: IndexVec<RegionIndex, R>,
}

//ip Rectangle
impl<R: RegionData> Rectangle<R> {
    //cp new
    /// Create a new Rectangle of a given size (with origin 0,0) and a
    /// data for the Rectangle
    pub fn new(width: isize, height: isize, data: R) -> Self {
        let xcoords = vec![0, width];
        let ycoords = vec![0, height];
        let coords = (xcoords, ycoords);
        let regions = index_vec![data];
        Self { coords, regions }
    }

    //mp test_validate
    /// In test, validate the data structure
    #[cfg(test)]
    #[track_caller]
    pub fn test_validate(&self) {
        let nx = self.coords.0.len();
        let ny = self.coords.1.len();
        let num_regions = (ny - 1) * (nx - 1);
        assert_eq!(self.len(), num_regions, "Number of regions mismatched");
    }

    //ap len
    /// Return the number of regions; one can just iterate through them
    pub fn len(&self) -> usize {
        self.regions.len()
    }

    //fi insert_x
    fn insert_x(&mut self, coord: isize, insert_at: usize) {
        let nx = self.coords.0.len();
        let ny = self.coords.1.len();
        let num_regions = (ny - 1) * nx;

        let mut x = 0;
        let old_regions =
            std::mem::replace(&mut self.regions, IndexVec::with_capacity(num_regions));
        for r in old_regions {
            let insert_at_start = (x == 0) && (insert_at == 0);
            let insert_after = insert_at == x + 1;
            if insert_at_start || insert_after {
                self.regions.push(r.clone());
            };
            self.regions.push(r);
            x = (x + 1) % (nx - 1);
        }
        self.coords.0.insert(insert_at, coord);
    }

    //fi insert_y
    fn insert_y(&mut self, coord: isize, insert_at: usize) {
        let insert_at = insert_at.max(1);
        let nx = self.coords.0.len();
        let ny = self.coords.1.len();
        let num_regions = (nx - 1) * ny;

        let mut x = 0;
        let mut y = 0;
        let old_regions =
            std::mem::replace(&mut self.regions, IndexVec::with_capacity(num_regions));
        for (i, r) in old_regions.into_iter().enumerate() {
            self.regions.push(r);
            x = x + 1;
            if x == nx - 1 {
                y += 1;
                x = 0;
                // Note that at this point i is the index of the *end* of the previous row
                //
                // So i+1 is the start of the row that replicates the row before
                //
                // and we have to replicate (i+1)-(nx-1) into (i+1)
                if insert_at == y {
                    for j in 0..nx - 1 {
                        self.regions.push(self.regions[i + 2 - nx + j].clone());
                    }
                };
            }
        }
        self.coords.1.insert(insert_at, coord);
    }

    //mp split
    /// Split X or Y at a coordinate
    pub fn split(&mut self, coord: isize, at_x: bool) {
        if at_x {
            let Err(insert_at) = self.coords.0.binary_search(&coord) else {
                return;
            };
            self.insert_x(coord, insert_at);
        } else {
            let Err(insert_at) = self.coords.1.binary_search(&coord) else {
                return;
            };
            self.insert_y(coord, insert_at);
        }
    }

    //mp split_region
    /// Split the rectangle so that a region bounded by an X range and
    /// a Y range exists in the grid of the Rectangle's regions
    ///
    /// This might split some regions of the rectangle, or it might
    /// not; if the rectangle has already been split at the start and
    /// end X, and at the start and end Y, then no change occurs
    pub fn split_region(&mut self, x_range: Range<isize>, y_range: Range<isize>) {
        self.split(x_range.start, true);
        self.split(x_range.end, true);
        self.split(y_range.start, false);
        self.split(y_range.end, false);
    }

    //mp split_region_iter
    /// Split the rectangle so that a region bounded by an X range and
    /// a Y range exists in the grid, and return an iterator over the
    /// Rectangle for all the RegionIndex that make up the region
    pub fn split_region_iter(
        &mut self,
        x_range: Range<isize>,
        y_range: Range<isize>,
    ) -> RegionIter {
        self.split_region(x_range.clone(), y_range.clone());
        self.iter_region(x_range, y_range).unwrap()
    }

    //mp region_offset
    /// Get the RegionIndex of a neighbor to another RegionIndex -
    /// offset by (dx,dy); normally dx and dy are a unit direction
    /// vector
    pub fn region_offset(&self, ri: RegionIndex, dx: isize, dy: isize) -> Option<RegionIndex> {
        let (x, y) = self.coords_of_region(ri);
        let x = x as isize + dx;
        let y = y as isize + dy;
        if x >= 0
            && x + 1 < self.coords.0.len() as isize
            && y >= 0
            && y + 1 < self.coords.1.len() as isize
        {
            Some(self.region_of_coords(x as usize, y as usize))
        } else {
            None
        }
    }

    //mp bounds
    /// Return the bounds (min XY, max XY) of the Region
    pub fn bounds(&self, ri: RegionIndex) -> (isize, isize, isize, isize) {
        let (x, y) = self.coords_of_region(ri);
        (
            self.coords.0[x],
            self.coords.1[y],
            self.coords.0[x + 1],
            self.coords.1[y + 1],
        )
    }

    //mp map_region
    /// Invoke a function on each corner of the Region
    pub fn map_region<F, T>(&self, ri: RegionIndex, f: F) -> (T, T, T, T)
    where
        F: Fn(isize, isize) -> T,
    {
        let (x, y) = self.coords_of_region(ri);
        (
            f(self.coords.0[x], self.coords.1[y]),
            f(self.coords.0[x], self.coords.1[y + 1]),
            f(self.coords.0[x + 1], self.coords.1[y + 1]),
            f(self.coords.0[x + 1], self.coords.1[y]),
        )
    }

    //fi coords_of_region
    /// Internal function as x and y indices are never exported
    ///
    /// Given RegionIndex, return region x and y indices
    fn coords_of_region(&self, ri: RegionIndex) -> (usize, usize) {
        let nrx = self.coords.0.len() - 1;
        let ri: usize = ri.into();
        let x = ri % nrx;
        let y = ri / nrx;
        (x, y)
    }

    //fi region_of_coords
    /// Internal function as x and y indices are never exported
    ///
    /// Given region x and y indices, return the actual RegionIndex
    fn region_of_coords(&self, x: usize, y: usize) -> RegionIndex {
        let nrx = self.coords.0.len() - 1;
        (x + (y * nrx)).into()
    }

    pub fn region_containing(&self, x: isize, y: isize) -> RegionIndex {
        let x_coord = self.coords.0.binary_search(&x).unwrap_or_else(|n| n);
        let y_coord = self.coords.1.binary_search(&y).unwrap_or_else(|n| n);
        self.region_of_coords(x_coord, y_coord)
    }

    //mp iter_region
    /// Return an iterator of Region Indices through regions that
    /// overlap with (inclusive, inclusive) ranges of X and Y,
    /// invoking a callback with this, a region index for the region,
    /// and bools for whether the left of the region is the left of
    /// the X range, the right of the region is the right of the X
    /// range, the bottom of the region is the bottom of the Y range,
    /// etc.
    ///
    /// Requires the corners (that is start and end for both X and Y
    /// ranges) to be explicit in the rectangle regions so that the
    /// inclusive range start to end-1 are the beginning and end of
    /// the regions iterated over. If this condition is not met then
    /// None is returned.
    pub fn iter_region(&self, x_range: Range<isize>, y_range: Range<isize>) -> Option<RegionIter> {
        let Ok(min_x_coord) = self.coords.0.binary_search(&x_range.start) else {
            return None;
        };
        let Ok(max_x_coord) = self.coords.0.binary_search(&x_range.end) else {
            return None;
        };
        let Ok(min_y_coord) = self.coords.1.binary_search(&y_range.start) else {
            return None;
        };
        let Ok(max_y_coord) = self.coords.1.binary_search(&y_range.end) else {
            return None;
        };
        let x_bounds = min_x_coord..max_x_coord;
        let y_bounds = min_y_coord..max_y_coord;
        let nrx = self.coords.0.len() - 1;
        Some(RegionIter::new(nrx, x_bounds, y_bounds))
    }

    //mp iter
    /// Retun an iterator over the whole rectangle as regions
    pub fn iter(&self) -> RegionIter {
        let min_x_coord = 0;
        let max_x_coord = self.coords.0.len() - 1;
        let min_y_coord = 0;
        let max_y_coord = self.coords.1.len() - 1;
        let x_bounds = min_x_coord..max_x_coord;
        let y_bounds = min_y_coord..max_y_coord;
        let nrx = self.coords.0.len() - 1;
        RegionIter::new(nrx, x_bounds, y_bounds)
    }
}

//ip Index<RegionIndex> for Rectangle
impl<R: RegionData> std::ops::Index<RegionIndex> for Rectangle<R> {
    type Output = R;
    fn index(&self, index: RegionIndex) -> &Self::Output {
        &self.regions[index]
    }
}

//ip IndexMut<RegionIndex> for Rectangle
impl<R: RegionData> std::ops::IndexMut<RegionIndex> for Rectangle<R> {
    fn index_mut(&mut self, index: RegionIndex) -> &mut Self::Output {
        &mut self.regions[index]
    }
}

//tp RegionIter
/// An iterator type that contains the range of X indices, the range
/// of Y indices, and the current x/y
pub struct RegionIter {
    x: usize,
    y: usize,
    nrx: usize,
    x_bounds: Range<usize>,
    y_bounds: Range<usize>,
}

//ip RegionIter
impl RegionIter {
    pub fn new(nrx: usize, x_bounds: Range<usize>, y_bounds: Range<usize>) -> Self {
        let x = x_bounds.start;
        let y = y_bounds.start;
        Self {
            x,
            y,
            nrx,
            x_bounds,
            y_bounds,
        }
    }
}

//tp Iterator for RegionIter
impl std::iter::Iterator for RegionIter {
    type Item = (RegionIndex, bool, bool, bool, bool);
    fn next(&mut self) -> std::option::Option<<Self as Iterator>::Item> {
        if !self.x_bounds.contains(&self.x) {
            self.x = self.x_bounds.start;
            self.y = self.y + 1;
        }
        if !self.x_bounds.contains(&self.x) {
            None
        } else if !self.y_bounds.contains(&self.y) {
            None
        } else {
            let region_index = (self.x + self.y * self.nrx).into();
            let is_min_x = self.x == self.x_bounds.start;
            let is_max_x = self.x + 1 == self.x_bounds.end;
            let is_min_y = self.y == self.y_bounds.start;
            let is_max_y = self.y + 1 == self.y_bounds.end;
            self.x += 1;
            Some((region_index, is_min_x, is_min_y, is_max_x, is_max_y))
        }
    }
}

//a Tests
#[test]
fn test0() {
    let mut r: Rectangle<()> = Rectangle::new(20, 10, ());
    assert_eq!(r.len(), 1);
    r.split(5, true);
    assert_eq!(r.len(), 2);
    r.split(7, true);
    assert_eq!(r.len(), 3);
    r.split(8, false);
    assert_eq!(r.len(), 6);
    r.split(0, false);
    assert_eq!(r.len(), 6);
    r.split(9, false);
    assert_eq!(r.len(), 9);
}

#[test]
fn test_iter0() {
    let mut r: Rectangle<()> = Rectangle::new(20, 10, ());
    assert!(
        r.iter_region(0..20, 0..10).is_some(),
        "Complete region can be iterated over"
    );
    assert!(
        r.iter_region(0..10, 0..10).is_none(),
        "Incomplete X region cannot be iterated over"
    );
    assert!(
        r.iter_region(10..20, 0..10).is_none(),
        "Incomplete X region cannot be iterated over"
    );
    assert!(
        r.iter_region(0..20, 5..10).is_none(),
        "Incomplete Y region cannot be iterated over"
    );
    assert!(
        r.iter_region(0..20, 0..5).is_none(),
        "Incomplete Y region cannot be iterated over"
    );
    r.split(10, true);
    r.split(5, false);
    assert!(
        r.iter_region(0..20, 0..10).is_some(),
        "Complete region can be iterated over"
    );
    assert!(
        r.iter_region(0..10, 0..10).is_some(),
        "Complete region can be iterated over"
    );
    assert!(
        r.iter_region(10..20, 0..10).is_some(),
        "Complete region can be iterated over"
    );
    assert!(
        r.iter_region(0..20, 5..10).is_some(),
        "Complete region can be iterated over"
    );
    assert!(
        r.iter_region(0..20, 0..5).is_some(),
        "Complete region can be iterated over"
    );
}

#[test]
fn test_iter1() {
    let mut r: Rectangle<()> = Rectangle::new(20, 10, ());
    let ri: Vec<(RegionIndex, bool, bool, bool, bool)> =
        r.iter_region(0..20, 0..10).unwrap().collect();
    assert_eq!(&ri, &[(0.into(), true, true, true, true)]);

    let ri: Vec<(RegionIndex, bool, bool, bool, bool)> = r.iter().collect();
    assert_eq!(&ri, &[(0.into(), true, true, true, true)]);

    r.split(10, true);
    let ri: Vec<(RegionIndex, bool, bool, bool, bool)> =
        r.iter_region(0..20, 0..10).unwrap().collect();
    assert_eq!(
        &ri,
        &[
            (0.into(), true, true, false, true),
            (1.into(), false, true, true, true),
        ]
    );

    r.split(5, false);
    let ri: Vec<(RegionIndex, bool, bool, bool, bool)> =
        r.iter_region(0..20, 0..10).unwrap().collect();
    assert_eq!(
        &ri,
        &[
            (0.into(), true, true, false, false),
            (1.into(), false, true, true, false),
            (2.into(), true, false, false, true),
            (3.into(), false, false, true, true)
        ]
    );

    let ri: Vec<(RegionIndex, bool, bool, bool, bool)> =
        r.iter_region(0..10, 0..5).unwrap().collect();
    assert_eq!(&ri, &[(0.into(), true, true, true, true)]);

    let ri: Vec<(RegionIndex, bool, bool, bool, bool)> =
        r.iter_region(10..20, 0..5).unwrap().collect();
    assert_eq!(&ri, &[(1.into(), true, true, true, true)]);

    let ri: Vec<(RegionIndex, bool, bool, bool, bool)> =
        r.iter_region(0..10, 5..10).unwrap().collect();
    assert_eq!(&ri, &[(2.into(), true, true, true, true)]);

    let ri: Vec<(RegionIndex, bool, bool, bool, bool)> =
        r.iter_region(10..20, 5..10).unwrap().collect();
    assert_eq!(&ri, &[(3.into(), true, true, true, true)]);
}

#[test]
fn test_window0() {
    let mut wall: Rectangle<bool> = Rectangle::new(900, 230, false);
    wall.test_validate();
    wall.split(100, true);
    wall.test_validate();
    wall.split(190, true);
    wall.test_validate();
    wall.split(130, false);
    wall.test_validate();
    wall.split(170, false);
    wall.test_validate();

    // Must have 9 regions now (4 x coords, 4 ycoords)

    // X should be 0, 100, 190, 900
    // Y should be 0, 130, 170, 230
    //
    // Should have 12 regions after this (5 x coords, 4 ycoords)
    //
    // will insert at 2 (so that 190, 900 are beyond it; 0 and 100 ahead of it)
    wall.split(170, true);
    wall.test_validate();
    wall.split(250, true);
    wall.test_validate();
    wall.split(150, false);
    wall.test_validate();
    wall.split(190, false);
    wall.test_validate();
}

#[test]
fn test_window() {
    let mut wall: Rectangle<bool> = Rectangle::new(900, 230, false);
    for (ri, _, _, _, _) in wall.split_region_iter(100..190, 130..170) {
        wall[ri] = true;
    }
    let ri: Vec<(RegionIndex, bool, bool, bool, bool)> = wall.iter().collect();
    assert_eq!(
        &ri,
        &[
            (0.into(), true, true, false, false),
            (1.into(), false, true, false, false),
            (2.into(), false, true, true, false),
            (3.into(), true, false, false, false),
            (4.into(), false, false, false, false),
            (5.into(), false, false, true, false),
            (6.into(), true, false, false, true),
            (7.into(), false, false, false, true),
            (8.into(), false, false, true, true)
        ]
    );
    for (ri, _, _, _, _) in wall.split_region_iter(170..250, 150..190) {
        wall[ri] = true;
    }
    for ri_xyxy in wall.iter() {
        let ri = ri_xyxy.0;
        assert_eq!(wall.region_offset(ri, -1, 0).is_none(), ri_xyxy.1);
        assert_eq!(wall.region_offset(ri, 0, -1).is_none(), ri_xyxy.2);
        assert_eq!(wall.region_offset(ri, 1, 0).is_none(), ri_xyxy.3);
        assert_eq!(wall.region_offset(ri, 0, 1).is_none(), ri_xyxy.4);
    }
}
