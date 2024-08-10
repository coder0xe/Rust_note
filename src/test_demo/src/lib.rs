pub struct Guess {
    value : i32,
}

impl Guess {

    pub fn new (value : i32) -> Guess {
        if value < 1 {
            panic!("Guess value is less than 1: got {}", value)
        } else if value > 100 {
            panic!("Guess value is bigger than 100: got {}", value)
        }
        Guess {value}
    }
    
}

#[cfg(test)]
mod test_dqr {

    use super::*;
    #[test]
    #[should_panic(expected = "less")] 
    fn test () {
        Guess::new(0);
    }

    #[test]
    #[ignore]
    fn test1 () -> Result<(), String> {
        if 2 + 2 != 4 {
            Ok(())
        } else {
            Err(String::from("Error"))
        }
    }

}