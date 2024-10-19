#[derive(Debug)]
pub enum StorageError {
    OutOfBounds,
}

pub trait FiniteStorage {
    fn size(&self) -> usize;
}

// T: access (word) size
pub trait Storage<T>: ReadableStorage<T> + WritableStorage<T>
where
    T: Copy,
{
}

pub trait ReadableStorage<T>: FiniteStorage
where
    T: Copy,
{
    fn get(&self, address: usize) -> Result<T, StorageError>;
}

pub trait WritableStorage<T>: FiniteStorage
where
    T: Copy,
{
    fn set(&mut self, address: usize, value: T) -> Result<(), StorageError>;
}

pub struct ROM<T>
where
    T: Copy,
{
    data: Vec<T>,
}

impl<T> ROM<T>
where
    T: Copy + Default,
{
    pub fn new(size: usize) -> Self {
        return Self {
            data: vec![T::default(); size],
        };
    }
}

impl<T> From<Vec<T>> for ROM<T>
where
    T: Copy,
{
    fn from(value: Vec<T>) -> Self {
        return Self { data: value };
    }
}

impl<T> FiniteStorage for ROM<T>
where
    T: Copy,
{
    fn size(&self) -> usize {
        return self.data.len();
    }
}

impl<T> ReadableStorage<T> for ROM<T>
where
    T: Copy,
{
    fn get(&self, address: usize) -> Result<T, StorageError> {
        if let Some(value) = self.data.get(address) {
            return Ok(*value);
        }

        return Err(StorageError::OutOfBounds);
    }
}

pub struct RAM<T>
where
    T: Copy,
{
    data: Vec<T>,
}

impl<T> RAM<T>
where
    T: Copy + Default,
{
    pub fn new(ram_size: usize) -> Self {
        return Self {
            data: vec![T::default(); ram_size],
        };
    }

    pub fn resize(&mut self, new_ram_size: usize) {
        self.data.resize(new_ram_size, T::default());
    }
}

impl<T> FiniteStorage for RAM<T>
where
    T: Copy,
{
    fn size(&self) -> usize {
        return self.data.len();
    }
}

impl<T> ReadableStorage<T> for RAM<T>
where
    T: Copy,
{
    fn get(&self, address: usize) -> Result<T, StorageError> {
        if let Some(value) = self.data.get(address) {
            return Ok(*value);
        }

        return Err(StorageError::OutOfBounds);
    }
}

impl<T> WritableStorage<T> for RAM<T>
where
    T: Copy,
{
    fn set(&mut self, address: usize, value: T) -> Result<(), StorageError> {
        if let Some(destination) = self.data.get_mut(address) {
            *destination = value;
            return Ok(());
        }

        return Err(StorageError::OutOfBounds);
    }
}

impl<T> From<Vec<T>> for RAM<T>
where
    T: Copy,
{
    fn from(value: Vec<T>) -> Self {
        return Self { data: value };
    }
}
