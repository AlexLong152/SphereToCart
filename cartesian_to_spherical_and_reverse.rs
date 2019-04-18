extern crate nalgebra-glm as glm;
extern crate assert_approx_eq;

fn cartesian_to_spherical(cartesian: &glm::Vec3) -> glm::Vec3 {
    let r: f32 = glm::length(&cartesian);
    if cartesian.x == 0.0 && cartesian.y == 0.0 {
        return glm::vec3(r, 0.0, 0.0);
    }
    let mut theta: f32 = (cartesian.y / cartesian.x).atan();
    let phi: f32 = (glm::length(&glm::vec2(cartesian.x, cartesian.y)) / cartesian.z).atan();
    if cartesian.x < 0.0 && cartesian.y >= 0.0 && theta == 0.0 {
        theta = std::f32::consts::PI;
    } else if cartesian.x < 0.0 && cartesian.y < 0.0 && theta.signum() > 0.0 {
        theta -= std::f32::consts::PI;
    } else if cartesian.x < 0.0 && cartesian.y > 0.0 && theta.signum() < 0.0 {
        theta += std::f32::consts::PI;
    }
    glm::vec3(r, theta, phi)
}

fn spherical_to_cartesian(spherical: &glm::Vec3) -> glm::Vec3 {
    let (r, theta, phi) = (spherical.x, spherical.y, spherical.z);
    let x = r * phi.sin() * theta.cos();
    let y = r * phi.sin() * theta.sin();
    let z = r * phi.cos();
    glm::vec3(x, y, z)
}

#[cfg(test)]
mod tests {
    mod cartesian_to_spherical_and_back {
        use super::super::cartesian_to_spherical;
        use super::super::spherical_to_cartesian;
        use assert_approx_eq::assert_approx_eq;

        static EPSILON_ARRAY: [f32; 3] = [0.00001, 0.00001, 0.00001];

        macro_rules! test_row_coordinates {
            ($($name_cartesian:ident, $name_spherical:ident: $value:expr,)*) => {
                $(
                    #[test]
                    fn $name_cartesian() {
                        let input = $value;
                        let expected = glm::make_vec3(&input.1);
                        let actual = cartesian_to_spherical(&glm::make_vec3(&input.0));
                        assert_approx_eq!(expected, actual, glm::make_vec3(&EPSILON_ARRAY));
                    }

                    #[test]
                    fn $name_spherical() {
                        let input = $value;
                        let expected = glm::make_vec3(&input.0);
                        let actual = spherical_to_cartesian(&glm::make_vec3(&input.1));
                        assert_approx_eq!(expected, actual, glm::make_vec3(&EPSILON_ARRAY));
                    }
                )*
            }
        }

        test_row_coordinates! {
            cartesian_1, spherical_1: ([0.0, 0.0, 293.0], [293.0, 0.0, 0.0]),
            cartesian_2, spherical_2: ([60.0, 0.0, 293.0], [299.08025678737, 0.0, 0.20198569154707]),
            cartesian_3, spherical_3: ([0.0, 60.0, 293.0], [299.08025678737, 1.5707963267949, 0.20198569154707]),
            cartesian_4, spherical_4: ([60.0, 60.0, 293.0], [305.03934172497, 0.78539816339745, 0.28188845613468]),
            cartesian_5, spherical_5: ([-60.0, 0.0, 293.0], [299.08025678737, 3.1415926535898, 0.20198569154707]),
            cartesian_6, spherical_6: ([0.0, -60.0, 293.0], [299.08025678737, -1.5707963267949, 0.20198569154707]),
            cartesian_7, spherical_7: ([-60.0, -60.0, 293.0], [305.03934172497, -2.3561944901923, 0.28188845613468]),
            cartesian_8, spherical_8: ([-60.0, 60.0, 293.0], [305.03934172497, 2.3561944901923, 0.28188845613468]),
            cartesian_9, spherical_9: ([60.0, -60.0, 293.0], [305.03934172497, -0.78539816339745, 0.28188845613468]),
            cartesian_10, spherical_10: ([-25.0, 60.0, 293.0], [300.1233079919, 1.9655874464947, 0.21830754364898]),
            cartesian_11, spherical_11: ([25.0, -60.0, 293.0], [300.1233079919, -1.1760052070951, 0.21830754364898]),
            cartesian_12, spherical_12: ([-60.0, 25.0, 293.0], [300.1233079919, 2.74680153389, 0.21830754364898]),
            cartesian_13, spherical_13: ([60.0, -25.0, 293.0], [300.1233079919, -0.3947911, 0.21830754364898]),
        }
    }
}
