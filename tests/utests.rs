
#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    // use super::*;
    
    #[test]
    fn main() {
        assert_ne!(4, 3);
    }
    
    #[test]
    fn read_file(){
        assert_eq!(4, 4);
    }
}