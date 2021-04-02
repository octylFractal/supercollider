use na::{
    allocator::Allocator, DefaultAllocator, DimName, Point3, Unit, Vector2, Vector3, VectorN, U2,
};

use crate::shape::{ConvexShape, SupportProvider};

pub fn check_collision<SA, SB, SPA, SPB>(a: &SA, b: &SB) -> bool
where
    DefaultAllocator: Allocator<f64, U2>,
    SA: ConvexShape<U2, SPA>,
    SPA: SupportProvider<U2>,
    SB: ConvexShape<U2, SPB>,
    SPB: SupportProvider<U2>,
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

    let first_point = support(&a_sp, &b_sp, d).to_homogeneous();
    let origin: Point3<f64> = Point3::origin();
    d = match Unit::try_new(
        Vector2::from_homogeneous(&origin.coords - &first_point).unwrap(),
        1.0e-6,
    ) {
        Some(v) => v,
        None => {
            // The simplex contains the origin, collision
            return true;
        }
    };

    let mut simplex = Simplex::Point(first_point);

    loop {
        let new_point = support(&a_sp, &b_sp, d.clone());
        if new_point.dot(&d) < 0.0 {
            // Failed to cross the origin line
            return false;
        }
        simplex = simplex.add(new_point.to_homogeneous());
        simplex = match simplex {
            Simplex::Point(_) => unreachable!(),
            Simplex::Line(b, a) => {
                // No triangle yet
                let ab: Vector3<f64> = &b - &a;
                let ao: Vector3<f64> = (&origin.coords) - &a;
                let ab_perp = triple_product(&ab, &ao, &ab);
                d = match handle_redirect(&b, &a, ab, ab_perp) {
                    Ok(v) => v,
                    Err(e) => return e,
                };
                Simplex::Line(b, a)
            }
            Simplex::Triangle(c, b, a) => {
                // Triangle formed, validate
                let ab: Vector3<f64> = &b - &a;
                let ac: Vector3<f64> = &c - &a;
                let ao: Vector3<f64> = (&origin.coords) - &a;
                let ab_perp = ac.cross(&ab).cross(&ab);
                let ac_perp = ab.cross(&ac).cross(&ac);
                // check region AB
                if ab_perp.dot(&ao) > 0.0 {
                    // It's outside the simplex, shift simplex forward
                    d = match handle_redirect(&b, &a, ab, ab_perp) {
                        Ok(v) => v,
                        Err(e) => return e,
                    };
                    Simplex::Line(b, a)
                }
                // check region AC
                else if ac_perp.dot(&ao) > 0.0 {
                    // It's outside the simplex, shift simplex forward
                    d = match handle_redirect(&c, &a, ac, ac_perp) {
                        Ok(v) => v,
                        Err(e) => return e,
                    };
                    Simplex::Line(c, a)
                } else {
                    return true;
                }
            }
        }
    }
}

enum Simplex {
    Point(Vector3<f64>),
    Line(Vector3<f64>, Vector3<f64>),
    Triangle(Vector3<f64>, Vector3<f64>, Vector3<f64>),
}

impl Simplex {
    fn add(self, point: Vector3<f64>) -> Self {
        match self {
            Simplex::Point(a) => Simplex::Line(a, point),
            Simplex::Line(a, b) => Simplex::Triangle(a, b, point),
            Simplex::Triangle(_, _, _) => panic!("Triangle can't get another point"),
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
