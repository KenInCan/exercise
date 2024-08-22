use crate::garden::vegetables::*;
pub mod garden;
pub mod di;

pub fn test_crate() {
    print_vege();
    di::hello();
}
