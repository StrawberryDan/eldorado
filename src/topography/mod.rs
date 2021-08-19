use crate::image::*;
use crate::vector::*;

mod contour;
mod heightmap;
mod shaded;
mod tanaka;

pub use contour::Settings as ContourSettings;
pub use heightmap::HeightMap;
pub use shaded::Settings as ShadedSettings;
pub use tanaka::Settings as TanakaSettings;

pub use contour::generate as generate_contour_layer;
pub use shaded::generate as generate_shaded_layer;
pub use tanaka::generate as generate_tanaka_layer;
