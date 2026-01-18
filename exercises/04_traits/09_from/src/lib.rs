// TODO: Implement the `From` trait for the `WrappingU32` type to make `example` compile.

pub struct WrappingU32 {
    value: u32,
}

impl From<u32> for WrappingU32 {
    fn from(value: u32) -> Self {
        WrappingU32 { value }
    }
}

fn example() {
    let wrapping: WrappingU32 = 42.into();
    let wrapping = WrappingU32::from(42);
}

// Here is an example of a real conversion

struct Temperature {
    celsius: f32,
}

struct Fahrenheit {
    value: f32,
}

impl From<Fahrenheit> for Temperature {
    fn from(f: Fahrenheit) -> Self {
        Temperature { 
            celsius: f.value - 32.0 * 5.0 / 9.0 }
    }
}

