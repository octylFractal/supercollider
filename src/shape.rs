use na::{allocator::Allocator, DefaultAllocator, DimName, Point, Unit, VectorN};

/// A convex shape that can describe itself using a support function.
/// N, R, S are the same as with Vector
pub trait ConvexShape<R: DimName, S: SupportProvider<R>>
where
    DefaultAllocator: Allocator<f64, R>,
{
    /// The center of the shape.
    fn center(&self) -> Point<f64, R>;
    /// Get the optimized support point provider.
    fn support_provider(&self) -> S;
}

pub trait SupportProvider<R: DimName>
where
    DefaultAllocator: Allocator<f64, R>,
{
    /// Given a normalized direction, get the point furthest in that direction.
    fn support(&self, dir: Unit<VectorN<f64, R>>) -> Point<f64, R>;
}

pub fn support_from_vertices<R>(
    vertices: &[Point<f64, R>],
    dir: Unit<VectorN<f64, R>>,
) -> &Point<f64, R>
where
    DefaultAllocator: Allocator<f64, R>,
    R: DimName,
{
    let mut maximum: Option<(&Point<f64, R>, f64)> = None;
    for v in vertices {
        let k = v.coords.dot(&dir);
        if let Some(max) = maximum {
            if k <= max.1 {
                continue;
            }
        }
        maximum = Some((v, k));
    }
    maximum.unwrap().0
}
