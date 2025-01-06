pub mod storage;
pub mod buffer;


pub fn write_to_page() -> bool {
    
	return true;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_page_write() {
        let result = write_to_page();
        assert_eq!(result, true);
    }
}
