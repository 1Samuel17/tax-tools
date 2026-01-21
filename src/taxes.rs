/// Module for handling tax calculations.
/// This module provides structures and functions to calculate federal, state, and local taxes.
/// based on income and filing status. It includes tax brackets, rates, and standard deductions.
/// It also provides functionality to compute effective tax rates and total tax liabilities.

pub enum FilingStatus {
    Single,
    MarriedFilingJointly,
    MarriedFilingSeparately,
    HeadOfHousehold,
}

pub enum TaxType {
    Federal,
    SocialSecurity,
    Medicare,
    State(Option<String>), // State abbreviation if applicable
    Local(Option<String>), // Local jurisdiction if applicable

}

pub struct TaxBracket {
    pub lower_bound: f32,
    pub upper_bound: Option<f32>, // None indicates no upper limit
    pub rate: f32, // expressed as a decimal (e.g., 0.22 for 22%)
}


