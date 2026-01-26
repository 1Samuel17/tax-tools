//! This is a library that contains utility functions for calculating net income, necessary hours, elected deductions, and standard tax withholding given set of expenses and a hypothetical hourly wage or salary scenario. The idea is pretty much like the "Sample Paycheck" tool found in the [Paycom](https://www.paycom.com/software/employee-self-service/) employee portal, but aimed at having more functionality and customization.

pub mod withholdings;
pub mod deductions;
pub mod income;
pub mod utils;
pub mod expenses;

pub use crate::withholdings::*;
pub use crate::deductions::*;
pub use crate::income::*;
pub use crate::expenses::*;
pub use crate::utils::*;

pub struct EmploymentScenario {
    pub hourly_rate: f32,
    pub hours_per_week: f32,
    pub filing_status: FilingStatus,
    pub pretax_deductions: PreTaxDeductions,
    pub posttax_deductions: PostTaxDeductions,
}

impl EmploymentScenario {
    pub fn new(
        hourly_rate: f32,
        hours_per_week: f32,
        filing_status: FilingStatus,
        pretax_deductions: PreTaxDeductions,
        posttax_deductions: PostTaxDeductions
    ) -> Self {
        EmploymentScenario {
            hourly_rate,
            hours_per_week,
            filing_status,
            pretax_deductions,
            posttax_deductions
        }
    }

    pub fn calculate_net_paycheck(&self) -> f32 {
        let mut gross_paycheck = determine_gross_paycheck(self.hourly_rate, self.hours_per_week);
        let total_pretax = self.pretax_deductions.total_pretax_deductions();
        gross_paycheck -= total_pretax;
        let federal_withholding = estimate_paycheck_federal_withholdings(
            gross_paycheck,
            self.filing_status,
        );
        let social_security = estimate_social_security_withholding(gross_paycheck);
        let medicare = estimate_medicare_withholding(gross_paycheck);
        let total_posttax = self.posttax_deductions.total_posttax_deductions();

        round_2_decimals(gross_paycheck - federal_withholding - social_security - medicare - total_posttax)

    }
}

// UNIT TESTS FOR LIBRARY

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_calculate_net_paycheck() {
        let pretax_deductions = PreTaxDeductions::new(vec![
            PreTaxDeduction::Medical(Some(100.0)),
            PreTaxDeduction::Dental(Some(50.0)),
            PreTaxDeduction::Vision(Some(25.0)),
            PreTaxDeduction::Traditional401K(Some(200.0)),
            PreTaxDeduction::HSA(Some(150.0)),
        ]);
        let posttax_deductions = PostTaxDeductions::new(vec![
            PostTaxDeduction::Roth401K(Some(100.0)),
            PostTaxDeduction::VoluntaryLife(Some(30.0)),
        ]);
        let expenses = Expenses::new(vec![
            Expense::Housing(Some(2000.0)),
            Expense::Energy(Some(200.0)),
            Expense::Water(Some(50.0)),
            Expense::Internet(Some(60.0)),
            Expense::Phone(Some(80.0)),
            Expense::Vehicle(Some(300.0)),
            Expense::VehicleInsurance(Some(150.0)),
            Expense::VehicleGas(Some(100.0)),
            Expense::Groceries(Some(400.0)),
        ]);
        let scenario = EmploymentScenario::new(
            25.0,
            45.0,
            FilingStatus::Single,
            pretax_deductions,
            posttax_deductions
        );
        let net_paycheck = scenario.calculate_net_paycheck();
        assert_eq!(net_paycheck, 1440.33);
    }
}
