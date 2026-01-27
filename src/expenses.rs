//! Module for handling expense calculations.
//! Defines common standard expense categories and functions for totaling monthly expenses.

/// Common expense categories for monthly expenses.
/// Each variant can hold an optional f32 value representing the monthly expense amount.
/// If no amount is provided, it is treated as zero in calculations.
/// # Variants
/// * `Housing(Option<f32>)` - Monthly rent or mortgage expense
/// * `Energy(Option<f32>)` - Monthly energy expense
/// * `Water(Option<f32>)` - Monthly water expense
/// * `Gas(Option<f32>)` - Monthly gas expense
/// * `Internet(Option<f32>)` - Monthly internet expense
/// * `Phone(Option<f32>)` - Monthly phone expense
/// * `Vehicle(Option<f32>)` - Monthly vehicle expense
/// * `VehicleInsurance(Option<f32>)` - Monthly vehicle insurance expense
/// * `VehicleGas(Option<f32>)` - Monthly vehicle gas expense
/// * `Groceries(Option<f32>)` - Monthly groceries expense
pub enum Expense {
    Housing(Option<f32>),          // monthly rent or mortgage expense
    Energy(Option<f32>),           // monthly energy expense
    Water(Option<f32>),            // monthly water expense
    Gas(Option<f32>),              // monthly gas expense
    Internet(Option<f32>),         // monthly internet expense
    Phone(Option<f32>),            // monthly phone expense
    Vehicle(Option<f32>),          // monthly vehicle expense
    VehicleInsurance(Option<f32>), // monthly insurance expense
    VehicleGas(Option<f32>),       // monthly vehicle gas expense
    Groceries(Option<f32>),        // monthly groceries expense
}

/// Struct to hold a collection of monthly expenses.
/// Provides functionality to total all monthly expenses.
/// # Fields
/// * `expense_items: Vec<Expense>` - Vector of Expense enum variants representing different monthly expenses.
/// # Methods
/// * `new(expenses: Vec<Expense>) -> Self` - Creates a new Expenses struct from a vector of Expense items.
/// * `total_monthly_expenses(&self) -> f32` - Calculates the total of all monthly expenses, treating None values as zero.
/// # Example
/// ```
/// use paycheck_utils::expenses::{Expense, Expenses};
///
/// let expenses = Expenses::new(vec![
///     Expense::Housing(Some(2000.0)),
///     Expense::Energy(Some(150.0)),
///     Expense::Water(None),
///     Expense::Internet(Some(60.0)),
///     Expense::Phone(Some(80.0)),
///     Expense::Vehicle(Some(300.0)),
///     Expense::VehicleInsurance(Some(100.0)),
///     Expense::VehicleGas(Some(120.0)),
///     Expense::Groceries(Some(400.0)),
/// ]);
/// let total = expenses.total_monthly_expenses();
/// assert_eq!(total, 3210.0);
/// ```
pub struct Expenses {
    pub expense_items: Vec<Expense>,
}

impl Expenses {
    pub fn new(expenses: Vec<Expense>) -> Self {
        Expenses {
            expense_items: expenses,
        }
    }

    pub fn total_monthly_expenses(&self) -> f32 {
        self.expense_items
            .iter()
            .map(|expense| match expense {
                Expense::Housing(amount)
                | Expense::Energy(amount)
                | Expense::Water(amount)
                | Expense::Gas(amount)
                | Expense::Internet(amount)
                | Expense::Phone(amount)
                | Expense::Vehicle(amount)
                | Expense::VehicleInsurance(amount)
                | Expense::VehicleGas(amount)
                | Expense::Groceries(amount) => amount.unwrap_or(0.0),
            })
            .sum()
    }
}

// UNIT TESTS FOR EXPENSES MODULE

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_total_monthly_expenses() {
        let expenses = Expenses {
            expense_items: vec![
                Expense::Housing(Some(2200.0)),
                Expense::Energy(Some(250.0)),
                Expense::Water(Some(50.0)),
                Expense::Gas(None),
                Expense::Internet(Some(60.0)),
                Expense::Phone(Some(80.0)),
                Expense::Vehicle(Some(300.0)),
                Expense::VehicleInsurance(Some(150.0)),
                Expense::VehicleGas(Some(120.0)),
                Expense::Groceries(Some(400.0)),
            ],
        };
        let total = expenses.total_monthly_expenses();
        assert_eq!(total, 3610.0);
    }
}
