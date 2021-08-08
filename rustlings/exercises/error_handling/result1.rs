// result1.rs
// Make this test pass! Execute `rustlings hint result1` for hints :)

#[derive(PartialEq, Debug)]
struct PositiveNonzeroInteger(u64);

#[derive(PartialEq, Debug)]
enum CreationError {
    Negative,
    Zero,
}

use CreationError::{Negative, Zero};

impl PositiveNonzeroInteger {
    fn new(value: i64) -> Result<PositiveNonzeroInteger, CreationError> {
        
        // Plan A: use match
        let ans = match value {
            n if n < 0 => {
                Err(Negative)
            },
            m if m == 0 => {
                Err(Zero)
            },
            i => {
                Ok(PositiveNonzeroInteger(i as u64))
            },
        };
        
        // Plan B: use if
        let ans = if value < 0 {
                Err(Negative)
            
            } else if value == 0 {
                Err(Zero)

            } else {
                Ok(PositiveNonzeroInteger(value as u64))

            };

        ans
    }
}

#[test]
fn test_creation() {
    assert!(PositiveNonzeroInteger::new(10).is_ok());
    assert_eq!(
        Err(CreationError::Negative),
        PositiveNonzeroInteger::new(-10)
    );
    assert_eq!(Err(CreationError::Zero), PositiveNonzeroInteger::new(0));
}
