/// Module for handling paycheck income calculations for hourly paid employees with potential overtime.
/// When considering hourly income, this module calculates from a bi-weekly paycheck perspective to synthesize how an employee thinks, views, and plans their income.

use crate::utils::*;

// per paycheck

pub fn determine_gross_paycheck(rate: f32, hours_per_week: f32) -> f32 {

    let regular_hours = 
        if hours_per_week > STANDARD_HOURS_PER_WEEK {
            STANDARD_HOURS_PER_WEEK
        }
        else {hours_per_week};

    let overtime_hours = 
        if hours_per_week > STANDARD_HOURS_PER_WEEK {
            hours_per_week - STANDARD_HOURS_PER_WEEK
        }
        else {0.0};
    
    let gross_paycheck = ((regular_hours * rate) + (overtime_hours * (rate * OVERTIME_MULTIPLIER))) * PAY_PERIOD;

    round_2_decimals(gross_paycheck)
}

// helper functions
fn round_2_decimals(value: f32) -> f32 {
    (value * 100.0).round() / 100.0
}


// UNIT TESTS FOR INCOME MODULE

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_determine_gross_paycheck() {
        let rate = 20.0;
        let hours_per_week = 45.0;
        let gross_paycheck = determine_gross_paycheck(rate, hours_per_week);
        assert_eq!(gross_paycheck, 1900.00);
    }

    #[test]
    fn test_determine_gross_paycheck_no_overtime() {
        let rate = 15.0;
        let hours_per_week = 35.0;
        let gross_paycheck = determine_gross_paycheck(rate, hours_per_week);
        assert_eq!(gross_paycheck, 1050.00);
    }
}