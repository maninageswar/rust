mod add;
mod subtract;
pub mod multiply;

use add::*;
use subtract::*;

pub fn do_addtion_and_subtract(a: i32, b: i32) -> (i32, i32) {
    (add(a, b), subtract(a, b))
}