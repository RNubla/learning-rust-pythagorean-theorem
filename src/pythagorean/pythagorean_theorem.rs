
use std::num::ParseFloatError;

pub fn solve_for_hypotenuse(leg_a: &String, leg_b: &String) -> Result<f64, ParseFloatError> {
    // must handle the error case when the user enters a non-numeric value
    let leg_a = match leg_a.trim().parse::<f64>() {
        Ok(leg_a) => leg_a,
        Err(e) => return Err(e),
    };
    let leg_b = match leg_b.trim().parse::<f64>() {
        Ok(leg_b) => leg_b,
        Err(e) => return Err(e),
    };
    let result = (leg_a * leg_a + leg_b * leg_b).sqrt();
    Ok(result)
}

pub fn solve_for_leg_a(hypotenuse: &String, leg_b: &String) -> Result<f64, ParseFloatError> {
    let hypotenuse = match hypotenuse.trim().parse::<f64>() {
        Ok(hypotenuse) => hypotenuse,
        Err(e) => return Err(e),
    };
    let leg_b = match leg_b.trim().parse::<f64>() {
        Ok(leg_b) => leg_b,
        Err(e) => return Err(e),
    };
    let result = (hypotenuse * hypotenuse - leg_b * leg_b).sqrt();
    Ok(result)
}

pub fn solve_for_leg_b(hypotenuse: &String, leg_a: &String) -> Result<f64, ParseFloatError> {
    let hypotenuse = match hypotenuse.trim().parse::<f64>() {
        Ok(hypotenuse) => hypotenuse,
        Err(e) => return Err(e),
    };
    let leg_a = match leg_a.trim().parse::<f64>() {
        Ok(leg_a) => leg_a,
        Err(e) => return Err(e),
    };
    let result = (hypotenuse * hypotenuse - leg_a * leg_a).sqrt();
    Ok(result)
}
