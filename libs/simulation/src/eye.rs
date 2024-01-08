use crate::*;
use std::{f32::consts::*, f64::consts::PI};

/// How far our eye can see:
///
/// -----------------
/// |               |
/// |               |
/// |               |
/// |@      %      %|
/// |               |
/// |               |
/// |               |
/// -----------------
///
/// If @ marks our birdie and % marks food, then a FOV_RANGE of:
///
/// - 0.1 = 10% of the map = bird sees no foods (at least in this case)
/// - 0.5 = 50% of the map = bird sees one of the foods
/// - 1.0 = 100% of the map = bird sees both foods
const FOV_RANGE: f32 = 0.25;

/// How wide our eye can see.
///
/// If @> marks our birdie (rotated to the right) and . marks the area
/// our birdie sees, then a FOV_ANGLE of:
///
/// - PI/2 = 90° =
///   -----------------
///   |             /.|
///   |           /...|
///   |         /.....|
///   |       @>......|
///   |         \.....|
///   |           \...|
///   |             \.|
///   -----------------
///
/// - PI = 180° =
///   -----------------
///   |       |.......|
///   |       |.......|
///   |       |.......|
///   |       @>......|
///   |       |.......|
///   |       |.......|
///   |       |.......|
///   -----------------
///
/// - 2 * PI = 360° =
///   -----------------
///   |...............|
///   |...............|
///   |...............|
///   |.......@>......|
///   |...............|
///   |...............|
///   |...............|
///   -----------------
///
/// Field of view depends on both FOV_RANGE and FOV_ANGLE:
///
/// - FOV_RANGE=0.4, FOV_ANGLE=PI/2:
///   -----------------
///   |       @       |
///   |     /.v.\     |
///   |   /.......\   |
///   |   ---------   |
///   |               |
///   |               |
///   |               |
///   -----------------
///
/// - FOV_RANGE=0.5, FOV_ANGLE=2*PI:
///   -----------------
///   |               |
///   |      ---      |
///   |     /...\     |
///   |    |..@..|    |
///   |     \.../     |
///   |      ---      |
///   |               |
///   -----------------
const FOV_ANGLE: f32 = (PI as f32) + FRAC_PI_4;

/// How much photoreceptors there are in a single eye.
///
/// More cells means our birds will have more "crisp" vision, allowing
/// them to locate the food more precisely - but the trade-off is that
/// the evolution process will then take longer, or even fail, unable
/// to find any solution.
///
/// I've found values between 3~11 sufficient, with eyes having more
/// than ~20 photoreceptors yielding progressively worse results.
const CELLS: usize = 9;

#[derive(Debug)]
pub struct Eye {
    fov_range: f32,
    fov_angle: f32,
    cells: usize,
}

impl Eye {
    fn new(fov_range: f32, fov_angle: f32, cells: usize) -> Self {
        assert!(fov_range > 0.0);
        assert!(fov_angle > 0.0);
        assert!(cells > 0);

        Self {
            fov_range,
            fov_angle,
            cells,
        }
    }

    pub fn cells(&self) -> usize {
        self.cells
    }
    pub fn process_vision(
        &self,
        position: na::Point2<f32>,
        rotation: na::Rotation2<f32>,
        foods: &[Food],
    ) -> Vec<f32> {
        let mut cells = vec![0.0; self.cells];
        for food in foods {
            let vec = food.position - position;
            let dist = vec.norm();
            if dist >= self.fov_range {
                continue;
            }

            let dist = vec.norm();

            let angle = na::Rotation2::rotation_between(&na::Vector2::y(), &vec).angle();
            let angle = angle - rotation.angle();
            let angle = na::wrap(angle, -PI as f32, PI as f32);

            //Two conditions
            //
            if angle < -self.fov_angle / 2.0 || angle > self.fov_angle / 2.0 {
                continue;
            }

            let angle = angle + self.fov_angle / 2.0;
            let cell = angle / self.fov_angle;
            let cell = cell * (self.cells as f32);
            let cell = (cell as usize).min(cells.len() - 1);

            //Create the enegery of food
            //the enegry is inversly proportional to the distance between our bird and the
            //currently checked foods.
            //Example:
            //-0.00001 = food is barely in the field of view
            //-1.0000 = food is very close to the bird.
            let energy = (self.fov_range - dist) / self.fov_range;
            cells[cell] += energy;
        }
        cells
    }
}

impl Default for Eye {
    fn default() -> Self {
        Self::new(FOV_RANGE, FOV_ANGLE, CELLS)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    const TEST_EYE_CELLS: usize = 13;

    struct TestCase {
        foods: Vec<Food>,
        fov_range: f32,
        fov_angle: f32,
        x: f32,
        y: f32,
        rot: f32,
        expected_vision: &'static str,
    }

    impl TestCase {
        fn run(self) {
            let eye = Eye::new(self.fov_range, self.fov_angle, TEST_EYE_CELLS);

            let actual_vision = eye.process_vision(
                na::Point2::new(self.x, self.y),
                na::Rotation2::new(self.rot),
                &self.foods,
            );
            let actual_vision: Vec<_> = actual_vision
                .into_iter()
                .map(|cell| {
                    if cell >= 0.7 {
                        //the food is right in front of the bird!
                        "#"
                    } else if cell >= 0.3 {
                        //the food is further
                        "+"
                    } else if cell > 0.0 {
                        //The food is far away
                        "."
                    } else {
                        //No food in sight
                        " "
                    }
                })
                .collect();

            let actual_vision = actual_vision.join("");

            assert_eq!(actual_vision, self.expected_vision);
        }
    }

    fn food(x: f32, y: f32) -> Food {
        Food {
            position: na::Point2::new(x, y),
        }
    }

    mod different_fov_ranges {
        use super::*;
        use test_case::test_case;

        #[test_case(1.0, "      +      ")] // Food is inside the FOV
        #[test_case(0.9, "      +      ")] // ditto
        #[test_case(0.8, "      +      ")] // ditto
        #[test_case(0.7, "      .      ")] // Food slowly disappears
        #[test_case(0.6, "      .      ")] // ditto
        #[test_case(0.5, "             ")] // Food disappeared!
        #[test_case(0.4, "             ")]
        #[test_case(0.3, "             ")]
        #[test_case(0.2, "             ")]
        #[test_case(0.1, "             ")]
        fn test(fov_range: f32, expected_vision: &'static str) {
            TestCase {
                foods: vec![food(0.5, 1.0)],
                fov_angle: FRAC_PI_2,
                x: 0.5,
                y: 0.5,
                rot: 0.0,
                fov_range,
                expected_vision,
            }
            .run()
        }
    }
}
