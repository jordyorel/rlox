

pub type Value = f64;

/// A simple array of values.
/// fields:
/// count: usize - The number of values in the array.
/// values: Vec<Value> - The array of values.
#[derive(Debug)]
pub struct ValueArray {
    values: Vec<Value>,
}

impl ValueArray {
    /// Creates a new, empty `ValueArray` instance.
    /// This method initializes the array with zero values and an empty
    pub fn new() -> Self {
        Self {
            values: Vec::new(),
        }
    }

    /// Writes a value into the array.
    /// This method appends a new value to the `values` vector and increments
    pub fn write_value_array(&mut self, values: Value) {
        self.values.push(values);
    }

    pub fn len(&self) -> usize {
        self.values.len()
    }

    pub fn is_empty(&self) -> bool {
        self.values.is_empty()
    }

    pub fn get(&self, index: usize) -> Option<Value> {
        self.values.get(index).copied()
    }

    pub fn print_values(&self) {
        for value in &self.values {
            println!("{}'", value);
        }
    }
}
