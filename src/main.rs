use current_platform::{COMPILED_ON, CURRENT_PLATFORM};

#[cfg(target_os="windows")]
fn windows_only() {
    println!("This will only get printed on Windows.");
}

fn main() {
    println!("Hello, world from {}! I was compiled on {}.", CURRENT_PLATFORM, COMPILED_ON);
    #[cfg(target_os="windows")]
    {
        windows_only();
    }
}

#[cfg(test)]
mod tests {
    use current_platform::{COMPILED_ON, CURRENT_PLATFORM};

    #[test]
    fn test_compiled_on_equals_current_platform() {
        assert_eq!(COMPILED_ON, CURRENT_PLATFORM);
    }
}
