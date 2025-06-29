use crate::Part;

impl PartialEq for Part {
    fn eq(&self, other: &Self) -> bool {
        match (&self.inner, &other.inner) {
            (Some(_), Some(_)) => {
                let intersection = self.intersect(other);

                (intersection.volume() - self.volume()).abs() < intersection.volume() * 1e-7
                    && (intersection.volume() - other.volume()).abs() < intersection.volume() * 1e-7
            }
            (Some(_), None) => false,
            (None, Some(_)) => false,
            (None, None) => true,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{Cuboid, IntoLength, Sphere};

    #[test]
    fn eq_both_none() {
        assert_eq!(Part::empty(), Part::empty())
    }

    #[test]
    fn eq_both_cuboid() {
        let cuboid1 = Cuboid::from_dim(1.m(), 1.m(), 1.m());
        let cuboid2 = Cuboid::from_dim(1.m(), 1.m(), 1.m());
        assert_eq!(cuboid1, cuboid2)
    }

    #[test]
    fn neq_both_cuboid() {
        let cuboid1 = Cuboid::from_dim(1.m(), 1.m(), 1.m());
        let cuboid2 = Cuboid::from_dim(2.m(), 2.m(), 2.m());
        assert_ne!(cuboid1, cuboid2)
    }

    #[test]
    fn eq_both_sphere() {
        let sphere1 = Sphere::from_radius(2.m());
        let sphere2 = Sphere::from_radius(2.m());
        assert_eq!(sphere1, sphere2)
    }

    #[test]
    fn neq_both_sphere() {
        let sphere1 = Sphere::from_radius(1.m());
        let sphere2 = Sphere::from_radius(2.m());
        assert_ne!(sphere1, sphere2)
    }
}
