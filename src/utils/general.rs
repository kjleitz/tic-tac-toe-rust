use std::error::Error;

pub fn passes_or<T, E: Error>(value: T, condition: fn(&T) -> bool, error: E) -> Result<T, E> {
    if condition(&value) {
        Ok(value)
    } else {
        Err(error)
    }
}
