use crate::{color::Color, point::Point};

#[derive(Debug, PartialEq, Clone)]
pub struct TestPattern {}

impl TestPattern {
    pub fn color_at(&self, point: Point) -> Color {
        Color::new(point.x, point.y, point.z)
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        color::Color,
        geometry::{shape::Sphere, Shape},
        matrix::Matrix,
        pattern::test_pattern,
        point::Point,
        transform::{scaling, translation},
    };

    #[test]
    fn default_pattern_transformation() {
        let pattern = test_pattern();
        assert_eq!(pattern.transform, Matrix::identity(4, 4));
        assert_eq!(pattern.transform_inverse, Matrix::identity(4, 4));
    }

    #[test]
    fn assign_transformation() {
        let mut pattern = test_pattern();
        pattern.set_transform(translation(1, 2, 3));
        assert_eq!(pattern.transform, translation(1, 2, 3));
        assert_eq!(pattern.transform_inverse, translation(1, 2, 3).inverse());
    }

    #[test]
    fn pattern_with_object_transformation() {
        let mut object = Sphere::default();
        object.set_transform(scaling(2, 2, 2));
        let pattern = test_pattern();

        let c = pattern.color_at_shape(&object, Point::new(2, 3, 4));
        assert_eq!(c, Color::new(1.0, 1.5, 2.0));
    }

    #[test]
    fn pattern_with_pattern_transformation() {
        let object = Sphere::default();
        let mut pattern = test_pattern();
        pattern.set_transform(scaling(2, 2, 2));

        let c = pattern.color_at_shape(&object, Point::new(2, 3, 4));
        assert_eq!(c, Color::new(1.0, 1.5, 2.0));
    }

    #[test]
    fn pattern_with_pattern_and_object_transformation() {
        let mut object = Sphere::default();
        object.set_transform(scaling(2, 2, 2));

        let mut pattern = test_pattern();
        pattern.set_transform(translation(0.5, 1.0, 1.5));

        let c = pattern.color_at_shape(&object, Point::new(2.5, 3.0, 3.5));
        assert_eq!(c, Color::new(0.75, 0.5, 0.25));
    }
}
