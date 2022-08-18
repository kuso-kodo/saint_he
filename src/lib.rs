#[macro_export]
macro_rules! saint_he {
    ($fn:ident($first:tt $(|$others:tt)*, $t:tt)) => {
        {
            let _ = $fn($first, $t);
            $(
                let _ = $fn($others, $t);
            )*
        }
    };
}

#[allow(non_snake_case)]
fn powerCon(port: u8, value: u8) {
    println!("set port {} to {}", port, value)
}

pub fn power_con() {
    saint_he!(powerCon(1 | 2 | 6 | 7 | 11 | 52 | 57 | 58 | 65, 10))
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        crate::power_con();
    }
}
