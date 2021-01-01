use super::sphere::Sphere;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Intersection<'a> {
    t: f64,
    object: &'a Sphere,
}

impl<'a> Intersection<'a> {
    pub fn new(t: f64, object: &'a Sphere) -> Self {
        Self { t, object }
    }

    pub fn t(&self) -> f64 {
        self.t
    }

    pub fn object(&self) -> &Sphere {
        self.object
    }
}

pub fn intersections<'a>(xs: &[Intersection<'a>]) -> Vec<Intersection<'a>> {
    let mut v = Vec::new();

    v.extend_from_slice(xs);
    v.sort_unstable_by(|a, b| a.t.partial_cmp(&b.t).unwrap());

    v
}

pub fn hit<'a>(xs: &'a [Intersection<'a>]) -> Option<&'a Intersection<'a>> {
    xs.iter().find(|&&i| i.t() >= 0.0)
}

#[cfg(test)]
mod tests {
    use std::ptr;

    use super::*;
    #[test]
    fn create_intersection() {
        let s = Sphere::default();
        let i = Intersection::new(3.5, &s);
        assert!(crate::equal(i.t, 3.5));
        assert!(ptr::eq(i.object, &s));
    }

    #[test]
    fn aggregate_intersections() {
        let s = Sphere::default();
        let i1 = Intersection::new(1.0, &s);
        let i2 = Intersection::new(2.0, &s);
        let xs = intersections(&[i1, i2]);
        assert_eq!(xs.len(), 2);
        assert!(crate::equal(xs[0].t(), 1.0));
        assert!(crate::equal(xs[1].t(), 2.0));
    }

    #[test]
    fn hit_all_intersections_positive_t() {
        let s = Sphere::default();
        let i1 = Intersection::new(1.0, &s);
        let i2 = Intersection::new(2.0, &s);
        let xs = intersections(&[i1, i2]);
        let i = hit(&xs);
        assert_eq!(*i.unwrap(), i1);
    }

    #[test]
    fn hit_some_intersections_negative_t() {
        let s = Sphere::default();
        let i1 = Intersection::new(-1.0, &s);
        let i2 = Intersection::new(1.0, &s);
        let xs = intersections(&[i1, i2]);
        let i = hit(&xs);
        assert_eq!(*i.unwrap(), i2);
    }

    #[test]
    fn hit_all_intersections_negative_t() {
        let s = Sphere::default();
        let i1 = Intersection::new(-2.0, &s);
        let i2 = Intersection::new(-1.0, &s);
        let xs = intersections(&[i1, i2]);
        let i = hit(&xs);
        assert!(i.is_none());
    }

    #[test]
    fn hit_is_always_nonnegative_intersection() {
        let s = Sphere::default();
        let i1 = Intersection::new(5.0, &s);
        let i2 = Intersection::new(7.0, &s);
        let i3 = Intersection::new(-3.0, &s);
        let i4 = Intersection::new(2.0, &s);

        let xs = intersections(&[i1, i2, i3, i4]);
        let i = hit(&xs);
        assert_eq!(*i.unwrap(), i4);
    }
}
