use std::cmp::min;
use std::collections::HashMap;
use std::hash::Hash;

/// Insert an element in an hashmap that contains a vec
///
/// # Arguments
/// * `hashmap` - The hashmap
/// * `key` - The key of the value that is added
/// * `value` - The value that will be inserted
///
pub fn insert_in_hashmap_vec<K: Eq + Hash, T>(hashmap: &mut HashMap<K, Vec<T>>, key: K, value: T) {
    if let Some(vec) = hashmap.get_mut(&key) {
        vec.push(value);
    } else {
        hashmap.insert(key, vec![value]);
    }
}

/// Copies as many `T` as possible from `src` into `dst`, returning the number of `T` copied. This
/// function is short form for `dst.copy_from_slice(src)`, but accounts for if their lengths are
/// unequal to avoid panics.
///
/// With the `nightly` feature, `[u8]` is specialized to use [`Read`], which is implemented
/// specially for small slices.
///
/// [`Read`]: https://doc.rust-lang.org/std/primitive.slice.html#impl-Read
///
/// # Examples
///
/// ```
/// use slice_copy::copy;
///
/// let mut l = vec![1 as u8, 2, 3, 4, 5];
/// let r = vec![10, 11, 12];
///
/// let n = copy(&mut l, &r);
///
/// assert_eq!(n, 3);
/// assert_eq!(l, vec![10, 11, 12, 4, 5]);
/// ```
#[inline]
pub fn copy<T>(dst: &mut [T], src: &[T]) -> usize
where
    T: Copy,
{
    #[cfg(feature = "nightly")]
    {
        dst.copy(src)
    }
    #[cfg(not(feature = "nightly"))]
    {
        let len = min(src.len(), dst.len());
        (&mut dst[..len]).copy_from_slice(&src[..len]);
        len
    }
}

/// Encode an object as bytes into a byte array
///
/// # Arguments
/// * `bytes` - The bytes buffer that will be written
/// * `offset` - An offset
/// * `size` - Ths size of the obj that will be written
/// * `obj` - The object that will be written
///
pub fn encode_into_bytes<T>(bytes: &mut [u8], offset: usize, size: usize, obj: T) {
    let buffer = &mut bytes[offset..(offset + size)];

    let encoded = unsafe {
        std::slice::from_raw_parts((&obj as *const T) as *const u8, std::mem::size_of::<T>())
    };

    copy(buffer, encoded);
}
