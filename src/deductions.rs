/// Module for handling tax deductions.
/// This module provides structures and functions to calculate standard and itemized deductions
/// based on filing status. It includes functionality to compute total deductions applicable
/// to an individual's tax situation.

pub enum PayrollDeduction {
    HealthInsurance(f32), // monthly health insurance deduction
    RetirementContribution(f32), // monthly retirement contribution deduction
    Other(f32), // other monthly payroll deductions
}

pub struct Deductions {
    pub payroll_deductions: Vec<PayrollDeduction>,
}

impl Deductions {
    pub fn total_monthly_deductions(&self) -> f32 {
        self.payroll_deductions.iter().map(|deduction| {
            match deduction {
                PayrollDeduction::HealthInsurance(amount) |
                PayrollDeduction::RetirementContribution(amount) |
                PayrollDeduction::Other(amount) => *amount,
            }
        }).sum()
    }

    pub fn total_annual_deductions(&self) -> f32 {
        self.total_monthly_deductions() * 12.0
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_total_monthly_deductions() {
        let deductions = Deductions {
            payroll_deductions: vec![
                PayrollDeduction::HealthInsurance(200.0),
                PayrollDeduction::RetirementContribution(150.0),
                PayrollDeduction::Other(50.0),
            ],
        };
        let total = deductions.total_monthly_deductions();
        assert_eq!(total, 400.0);
    }
    #[test]
    fn test_total_annual_deductions() {
        let deductions = Deductions {
            payroll_deductions: vec![
                PayrollDeduction::HealthInsurance(200.0),
                PayrollDeduction::RetirementContribution(150.0),
                PayrollDeduction::Other(50.0),
            ],
        };
        let total = deductions.total_annual_deductions();
        assert_eq!(total, 4800.0);
    }
}