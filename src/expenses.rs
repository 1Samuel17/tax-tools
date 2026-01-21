/// module for handling expense calculations.
/// This module provides structures and functions to calculate various types of expenses,
/// expenses and compute total monthly and annual expenses.


// standard expense categories that may or may not be applicable
pub enum Expense {
    Housing(Option<f32>), // monthly rent or mortgage expense
    Energy(Option<f32>), // monthly energy expense
    Water(Option<f32>), // monthly water expense
    Gas(Option<f32>), // monthly gas expense
    Internet(Option<f32>), // monthly internet expense
    Phone(Option<f32>), // monthly phone expense
    Vehicle(Option<f32>), // monthly vehicle expense
    VehicleInsurance(Option<f32>), // monthly insurance expense
    VehicleGas(Option<f32>), // monthly vehicle gas expense
    Groceries(Option<f32>), // monthly groceries expense
}

pub struct Expenses {
    pub expense_items: Vec<Expense>,
}

impl Expenses {
    pub fn total_monthly_expenses(&self) -> f32 {
        self.expense_items.iter().map(|expense| {
            match expense {
                Expense::Housing(amount) |
                Expense::Energy(amount) |
                Expense::Water(amount) |
                Expense::Gas(amount) |
                Expense::Internet(amount) |
                Expense::Phone(amount) |
                Expense::Vehicle(amount) |
                Expense::VehicleInsurance(amount) |
                Expense::VehicleGas(amount) |
                Expense::Groceries(amount) => amount.unwrap_or(0.0),
            }
        }).sum()
    }

    pub fn total_annual_expenses(&self) -> f32 {
        self.total_monthly_expenses() * 12.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_total_monthly_expenses() {
        let expenses = Expenses {
            expense_items: vec![
                Expense::Housing(Some(1200.0)),
                Expense::Energy(Some(150.0)),
                Expense::Water(None),
                Expense::Internet(Some(60.0)),
            ],
        };
        let total = expenses.total_monthly_expenses();
        assert_eq!(total, 1410.0);
    }
    #[test]
    fn test_total_annual_expenses() {
        let expenses = Expenses {
            expense_items: vec![
                Expense::Housing(Some(1200.0)),
                Expense::Energy(Some(150.0)),
                Expense::Water(None),
                Expense::Internet(Some(60.0)),
            ],
        };
        let total = expenses.total_annual_expenses();
        assert_eq!(total, 16920.0);
    }
}