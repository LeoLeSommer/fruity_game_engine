/// A sliced vector
#[derive(Debug, Clone, Default)]
pub struct SlicedVec<T> {
    // The actual data
    pub(crate) data: Vec<T>,
    // The indices of the slices
    pub(crate) indices: Vec<usize>,
    /// The lengths of the slices
    pub(crate) lengths: Vec<usize>,
}

impl<T> SlicedVec<T> {
    /// Creates a new SlicedVec
    pub fn new() -> Self {
        Self {
            data: Vec::new(),
            indices: Vec::new(),
            lengths: Vec::new(),
        }
    }

    /// Returns the total number of elements
    pub fn len(&self) -> usize {
        self.data.len()
    }

    /// Returns the number of slices
    pub fn slice_len(&self, slice_index: usize) -> usize {
        self.lengths[slice_index]
    }

    /// Returns the slice at the given index
    pub fn get_slice(&self, slice_index: usize) -> Option<&[T]> {
        let start = *self.indices.get(slice_index)?;
        let end = start + self.lengths.get(slice_index)?;

        Some(&self.data[start..end])
    }

    /// Returns the slice at the given index
    pub fn get_slice_mut(&mut self, slice_index: usize) -> Option<&mut [T]> {
        let start = *self.indices.get(slice_index)?;
        let end = start + self.lengths.get(slice_index)?;

        Some(&mut self.data[start..end])
    }

    /// Returns the slice at the given index
    pub fn get_unchecked_slice(&self, slice_index: usize) -> &[T] {
        let start = self.indices[slice_index];
        let end = start + self.lengths[slice_index];

        &self.data[start..end]
    }

    /// Returns the slice at the given index
    pub fn get_unchecked_mut_slice(&mut self, slice_index: usize) -> &mut [T] {
        let start = self.indices[slice_index];
        let end = start + self.lengths[slice_index];

        &mut self.data[start..end]
    }

    /// Returns the slice at the given index
    pub fn push_slice(&mut self, slice: Vec<T>) {
        self.indices.push(self.data.len());
        self.lengths.push(slice.len());
        self.data.extend(slice);
    }

    /// Removes the slice at the given index
    pub fn remove_slice(&mut self, slice_index: usize) -> Vec<T> {
        let start = self.indices[slice_index];
        let end = start + self.lengths[slice_index];

        let slice = self.data.drain(start..end).collect();

        for i in slice_index + 1..self.len() {
            self.indices[i] -= self.lengths[slice_index];
        }

        self.indices.remove(slice_index);
        self.lengths.remove(slice_index);

        slice
    }

    /// Insert a new item at the given index in a slice
    /// This will move all the items after the given index
    pub fn insert_in_slice(&mut self, slice_index: usize, item: T) {
        let start = self.indices[slice_index];
        let end = start + self.lengths[slice_index];

        self.data.insert(end, item);
        self.lengths[slice_index] += 1;

        for i in slice_index + 1..self.len() {
            self.indices[i] += 1;
        }
    }

    /// Append the contents of another SlicedVec to this one
    pub fn append(&mut self, other: &mut Self) {
        self.data.append(&mut other.data);
        self.indices.append(&mut other.indices);
        self.lengths.append(&mut other.lengths);
    }

    /// Remove an item at the given index in a slice
    /// This will move all the items after the given index
    pub fn remove_in_slice(&mut self, slice_index: usize, index: usize) -> T {
        let start = self.indices[slice_index];

        let item = self.data.remove(start + index);
        self.lengths[slice_index] -= 1;

        for i in slice_index + 1..self.len() {
            self.indices[i] -= 1;
        }

        item
    }

    /// Clears the SlicedVec
    pub fn clear(&mut self) {
        self.data.clear();
        self.indices.clear();
        self.lengths.clear();
    }

    /// Reserve capacity for at least `additional` more elements to be inserted in the storage
    pub fn reserve(&mut self, additional: usize) {
        self.data.reserve(additional);
    }

    /// Returns the capacity of the storage
    pub fn capacity(&self) -> usize {
        self.data.capacity()
    }
}
