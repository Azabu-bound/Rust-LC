impl Solution {
    pub fn convert_temperature(celsius: f64) -> Vec<f64> {
        let mut result = vec![];

        result.push(&celsius + 273.15);
        result.push(&celsius * 1.8 + 32.0);
        
        result
    }
}