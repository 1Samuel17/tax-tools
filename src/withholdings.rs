// module for handling estimated federal paycheck withholdings using the method outlined by the IRS in Publication 15T (2026) (percentage method)

use crate::utils::*;
use crate::income::round_2_decimals;

// 1. annualize paycheck
// 2. adjust for w4 deductions and credits
// 3. apply tax brackets
// 4. convert back to per-paycheck

pub fn estimate_paycheck_federal_withholdings(gross_paycheck: f32, filing_status: FilingStatus) -> f32 {
    
    let gross_annualized_paycheck = gross_paycheck * PAY_PERIODS_PER_YEAR;

    let standard_deduction = 
    match filing_status {
        FilingStatus::Single => {SINGLE_DEDUCTION},
        FilingStatus::HeadOfHousehold => {HEAD_OF_HOUSEHOLD_DEDUCTION}, // for future implementation
        FilingStatus::MarriedFilingJointly => {MARRIED_FILING_JOINTLY_DEDUCTION}, // for future implementation
        FilingStatus::MarriedFilingSeparate => {MARRIED_FILING_SEPERATE_DEDUCTION} // for future implementation
    };

    let adjusted_annualized_paycheck = gross_annualized_paycheck - standard_deduction;

    let estimated_annual_withholdings = apply_tax_brackets(adjusted_annualized_paycheck);

    round_2_decimals(estimated_annual_withholdings / PAY_PERIODS_PER_YEAR) // estimated per-paycheck federal withholding
}

fn apply_tax_brackets(adjusted_annualized_paycheck: f32) -> f32 {

    if adjusted_annualized_paycheck > SINGLE_BRACKET_6_THRESHOLD {
        ((adjusted_annualized_paycheck - SINGLE_BRACKET_6_THRESHOLD) * TAX_BRACKET_7_RATE) + SINGLE_BRACKET_7_BASE_TAX

    } else if adjusted_annualized_paycheck > SINGLE_BRACKET_5_THRESHOLD {
        ((adjusted_annualized_paycheck - SINGLE_BRACKET_5_THRESHOLD) * TAX_BRACKET_6_RATE) + SINGLE_BRACKET_6_BASE_TAX

    } else if adjusted_annualized_paycheck > SINGLE_BRACKET_4_THRESHOLD {
        ((adjusted_annualized_paycheck - SINGLE_BRACKET_4_THRESHOLD) * TAX_BRACKET_5_RATE) + SINGLE_BRACKET_5_BASE_TAX

    } else if adjusted_annualized_paycheck > SINGLE_BRACKET_3_THRESHOLD {
        ((adjusted_annualized_paycheck - SINGLE_BRACKET_3_THRESHOLD) * TAX_BRACKET_4_RATE) + SINGLE_BRACKET_4_BASE_TAX

    } else if adjusted_annualized_paycheck > SINGLE_BRACKET_2_THRESHOLD {
        ((adjusted_annualized_paycheck - SINGLE_BRACKET_2_THRESHOLD) * TAX_BRACKET_3_RATE) + SINGLE_BRACKET_3_BASE_TAX

    } else if adjusted_annualized_paycheck > SINGLE_BRACKET_1_THRESHOLD {
        ((adjusted_annualized_paycheck - SINGLE_BRACKET_1_THRESHOLD) * TAX_BRACKET_2_RATE) + SINGLE_BRACKET_2_BASE_TAX

    } else {
        adjusted_annualized_paycheck * TAX_BRACKET_1_RATE
    }
    
}

pub fn estimate_social_security_withholding(gross_paycheck: f32) -> f32 {
    gross_paycheck * SOCIAL_SECURITY_RATE
}

pub fn estimate_medicare_withholding(gross_paycheck: f32) -> f32 {
    gross_paycheck * MEDICARE_RATE
}




// UNIT TESTS FOR WITHHOLDINGS MODULE

#[cfg(test)] mod tests {
    use super::*;
    
    // TESTS FOR ESTIMATE_PAYCHECK_FEDERAL_WITHHOLDINGS FUNCTION
    #[test]
    fn test_federal_withholding() {
        let gross_paycheck = 2000.0;
        let filing_status = FilingStatus::Single;
        let result = estimate_paycheck_federal_withholdings(gross_paycheck, filing_status);
        let expected = 156.15; // Expected value based on 2026 tax brackets
        assert!((result - expected).abs() < 0.01, 
            "Expected {}, got {}", expected, result);
    }

    // TESTS FOR ESTIMATE_SOCIAL_SECURITY_WITHHOLDING FUNCTION
    #[test]
    fn test_social_security_withholding() {
        let gross_paycheck = 2000.0;
        let result = estimate_social_security_withholding(gross_paycheck);
        let expected = gross_paycheck * 0.062; // 6.2%
        assert!((result - expected).abs() < 0.01, 
            "Expected {}, got {}", expected, result);
    }
    // TESTS FOR ESTIMATE_MEDICARE_WITHHOLDING FUNCTION
    #[test]
    fn test_medicare_withholding() {
        let gross_paycheck = 2000.0;
        let result = estimate_medicare_withholding(gross_paycheck);
        let expected = gross_paycheck * 0.0145; // 1.45%
        assert!((result - expected).abs() < 0.01, 
            "Expected {}, got {}", expected, result);
    }
}