use na::{
    allocator::Allocator, DefaultAllocator, DimName, Point3, Unit, Vector2, Vector3, VectorN, U2,
    U3,
};

use crate::shape::{ConvexShape, SupportProvider, ToConvexShapes};

pub trait GJK<R: DimName, SB: ToConvexShapes<R>>
where
    DefaultAllocator: Allocator<f64, R>,
{
    fn check_collision(&self, other: &SB) -> bool;
}

impl<SA, SB> GJK<U2, SB> for SA
where
    DefaultAllocator: Allocator<f64, U2>,
    SA: ToConvexShapes<U2>,
    SB: ToConvexShapes<U2>,
{
    fn check_collision(&self, other: &SB) -> bool {
        // cache other's shapes
        let other_shapes = other.to_convex_shapes();
        // fun quadratic time op
        for self_shape in self.to_convex_shapes() {
            for other_shape in other_shapes.iter() {
                if gjk_single(self_shape, *other_shape) {
                    return true;
                }
            }
        }

        false
    }
}

fn gjk_single(a: &dyn ConvexShape<U2>, b: &dyn ConvexShape<U2>) -> bool
where
    DefaultAllocator: Allocator<f64, U2> + Allocator<f64, U3>,
{
    let a_sp = a.support_provider();
    let b_sp = b.support_provider();
    let mut d = match Unit::<Vector2<f64>>::try_new(b.center() - a.center(), 1.0e-6) {
        Some(u) => u,
        None => {
            // Centers are the same, collision
            return true;
        }
    };

    let mut simplex = Vec::<Vector3<f64>>::with_capacity(3);
    simplex.push(support(&*a_sp, &*b_sp, d).to_homogeneous());
    let origin: Point3<f64> = Point3::origin();
    d = match Unit::try_new(
        Vector2::from_homogeneous(&origin.coords - &simplex[0]).unwrap(),
        1.0e-6,
    ) {
        Some(v) => v,
        None => {
            // The simplex contains the origin, collision
            return true;
        }
    };

    loop {
        let new_point = support(&*a_sp, &*b_sp, d.clone());
        if new_point.dot(&d) < 0.0 {
            // Failed to cross the origin line
            return false;
        }
        simplex.push(new_point.to_homogeneous());
        match simplex.as_slice() {
            [b, a] => {
                // No triangle yet
                let ab: Vector3<f64> = b - a;
                let ao: Vector3<f64> = (&origin.coords) - a;
                let ab_perp = triple_product(&ab, &ao, &ab);
                d = match handle_redirect(b, a, ab, ab_perp) {
                    Ok(v) => v,
                    Err(e) => return e,
                };
            }
            [c, b, a] => {
                // Triangle formed, validate
                let ab: Vector3<f64> = b - a;
                let ac: Vector3<f64> = c - a;
                let ao: Vector3<f64> = (&origin.coords) - a;
                let ab_perp = ac.cross(&ab).cross(&ab);
                let ac_perp = ab.cross(&ac).cross(&ac);
                // check region AB
                if ab_perp.dot(&ao) > 0.0 {
                    // It's outside the simplex, shift simplex forward
                    d = match handle_redirect(b, a, ab, ab_perp) {
                        Ok(v) => v,
                        Err(e) => return e,
                    };
                    simplex.remove(0);
                    continue;
                }
                // check region AC
                if ac_perp.dot(&ao) > 0.0 {
                    // It's outside the simplex, shift simplex forward
                    d = match handle_redirect(c, a, ac, ac_perp) {
                        Ok(v) => v,
                        Err(e) => return e,
                    };
                    simplex.remove(1);
                    continue;
                }
                return true;
            }
            _ => unreachable!(),
        }
    }
}

/// Given points [a], [b] and the line between them [ab], and the perpendicular line [ab_perp],
/// get the direction of [ab_perp], or, if it is zero magnitude, if the origin is on [ab].
fn handle_redirect(
    b: &Vector3<f64>,
    a: &Vector3<f64>,
    ab: Vector3<f64>,
    ab_perp: Vector3<f64>,
) -> Result<Unit<Vector2<f64>>, bool> {
    Unit::try_new(Vector2::from_homogeneous(ab_perp).unwrap(), 1.0e-6).ok_or_else(|| {
        if ab.x.abs() >= ab.y.abs() {
            if ab.x > 0.0 {
                a.x <= 0.0 && 0.0 <= b.x
            } else {
                b.x <= 0.0 && 0.0 <= a.x
            }
        } else {
            if ab.y > 0.0 {
                a.y <= 0.0 && 0.0 <= b.y
            } else {
                b.y <= 0.0 && 0.0 <= a.y
            }
        }
    })
}

fn triple_product(a: &Vector3<f64>, b: &Vector3<f64>, c: &Vector3<f64>) -> Vector3<f64> {
    b * c.dot(a) - a * c.dot(b)
}

fn support<R: DimName>(
    a: &dyn SupportProvider<R>,
    b: &dyn SupportProvider<R>,
    dir: Unit<VectorN<f64, R>>,
) -> VectorN<f64, R>
where
    DefaultAllocator: Allocator<f64, R>,
{
    let a_sup = a.support(dir.clone());
    let b_sup = b.support(-dir.clone());
    return a_sup - b_sup;
}
