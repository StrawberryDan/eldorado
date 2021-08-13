use crate::image::*;
use crate::vector::*;

mod heightmap;
mod contour;
mod tanaka;
mod shaded;

pub use heightmap::HeightMap;
pub use contour::Settings as ContourSettings;
pub use tanaka::Settings as TanakaSettings;
pub use shaded::Settings as ShadedSettings;

pub use contour::generate as generate_contour_layer;
pub use tanaka::generate  as generate_tanaka_layer;
pub use shaded::generate  as generate_shaded_layer;
