use anyhow::Result;

pub fn percentage_of_growth(initial_value: &f64, final_value: &f64) -> Result<f64> {
    Ok(((final_value - initial_value) / initial_value) * 100.00)
}
