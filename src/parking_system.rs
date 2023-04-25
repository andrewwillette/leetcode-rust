#[allow(dead_code)]
pub struct ParkingSystem {
    max_big: i32,
    max_medium: i32,
    max_small: i32,
    big: i32,
    medium: i32,
    small: i32,
}

/**
 * https://leetcode.com/problems/design-parking-system/description/
 */
impl ParkingSystem {
    #[allow(dead_code)]
    fn new(big: i32, medium: i32, small: i32) -> Self {
        ParkingSystem {
            max_big: big,
            max_medium: medium,
            max_small: small,
            big: 0,
            medium: 0,
            small: 0,
        }
    }

    #[allow(dead_code)]
    fn add_car(&mut self, car_type: i32) -> bool {
        match car_type {
            1 => {
                if self.big < self.max_big {
                    self.big += 1;
                    return true;
                }
                false
            }
            2 => {
                if self.medium < self.max_medium {
                    self.medium += 1;
                    return true;
                }
                false
            }
            3 => {
                if self.small < self.max_small {
                    self.small += 1;
                    return true;
                }
                false
            }
            _ => false,
        }
    }
}
