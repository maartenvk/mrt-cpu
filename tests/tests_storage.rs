#[cfg(test)]
mod tests {
    use mrt_cpu::machine::storage::*;

    #[test]
    fn ram_new_size() {
        let storage: RAM<u8> = RAM::new(1);
        assert_eq!(storage.size(), 1);
    }

    #[test]
    fn ram_from_vec() {
        let vector: Vec<u8> = [1, 2, 3].to_vec();
        let storage: RAM<u8> = RAM::from(vector.clone());

        assert_eq!(storage.size(), vector.len());
        for (i, n) in vector.iter().enumerate() {
            assert_eq!(storage.get(i).unwrap(), *n);
        }
    }

    #[test]
    fn ram_get() {
        let vector: Vec<u8> = [1, 2, 3].to_vec();
        let storage: RAM<u8> = RAM::from(vector);

        assert_eq!(storage.get(1).unwrap(), 2);
    }

    #[test]
    fn ram_set() {
        let vector: Vec<u8> = [1, 2, 3].to_vec();
        let mut storage: RAM<u8> = RAM::from(vector);

        let result = storage.set(1, 42);

        assert!(result.is_ok());
        assert_eq!(storage.get(1).unwrap(), 42);
    }

    #[test]
    fn ram_get_out_of_bounds() {
        let vector: Vec<u8> = [1, 2, 3].to_vec();
        let storage: RAM<u8> = RAM::from(vector);

        let result = storage.get(42);

        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), StorageError::OutOfBounds));
    }

    #[test]
    fn ram_set_out_of_bounds() {
        let vector: Vec<u8> = [1, 2, 3].to_vec();
        let mut storage: RAM<u8> = RAM::from(vector);

        let result = storage.set(42, 1);

        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), StorageError::OutOfBounds));
    }
}
