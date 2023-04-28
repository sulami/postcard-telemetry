//! Generic ring buffers

/// A ring buffer that holds N elements of type T. Once the buffer is full,
/// the oldest element gets overwritten. The buffer is aware of how
/// full it is, so [`Ring::len`] and [`Ring::is_empty`] will report
/// `0` and `true` for a freshly constructed buffer.
///
/// ```
/// # use embedded_imu::ring::Ring;
/// // Keep up to 64 f32s.
/// let mut buf: Ring<f32, 64> = Ring::new();
///
/// buf.push(3.14);
/// buf.push(6.28);
///
/// let mut iter = buf.into_iter();
/// assert_eq!(iter.next(), Some(3.14));
/// assert_eq!(iter.next(), Some(6.28));
/// assert_eq!(iter.next(), None);
/// ```
#[derive(Clone)]
pub struct Ring<T: Copy + Default, const N: usize> {
    buf: [T; N],
    head: usize,
    filled: bool,
}

impl<T: Copy + Default, const N: usize> Ring<T, N> {
    /// Constructs a new, empty ring buffer.
    pub fn new() -> Self {
        Self::default()
    }

    /// Pushes a new item to the ring buffer.
    pub fn push(&mut self, item: T) {
        if N == 0 {
            return;
        }
        self.buf[self.head] = item;
        self.head = (self.head + 1) % N;
        if self.head == 0 {
            self.filled = true;
        }
    }

    /// Returns `true` if the buffer has been filled completely.
    pub fn is_saturated(&self) -> bool {
        self.filled
    }

    /// Returns the length of the ring buffer. Partially filled
    /// buffers have a length < `N`, while buffers always have lenth
    /// `N` once they are filled.
    pub fn len(&self) -> usize {
        if self.filled {
            N
        } else {
            self.head
        }
    }

    /// Returns whether the buffer is empty. A buffer can only be
    /// empty if it is freshly constructed.
    pub fn is_empty(&self) -> bool {
        !self.filled && self.head == 0
    }
}

impl<T: Copy + Default, const N: usize> Default for Ring<T, N> {
    fn default() -> Self {
        Self {
            buf: [T::default(); N],
            head: 0,
            filled: false,
        }
    }
}

impl<T: Copy + Default, const N: usize> IntoIterator for &Ring<T, N> {
    type Item = T;
    type IntoIter = RingIter<T, N>;

    /// Creates an iterator over the items in the ring buffer, from
    /// least recently inserted to most recently inserted.
    fn into_iter(self) -> Self::IntoIter {
        RingIter {
            buf: self.buf,
            left: if self.filled { self.head } else { 0 },
            right: if N == 0 { 0 } else { (N + self.head - 1) % N },
            finished: self.is_empty(),
        }
    }
}

/// An iterator over the items in the ring buffer, from least recently
/// inserted to most recently inserted.
pub struct RingIter<T: Copy + Default, const N: usize> {
    buf: [T; N],
    // NB Left is the oldest element, right the newest.
    left: usize,
    right: usize,
    finished: bool,
}

impl<T: Copy + Default, const N: usize> Iterator for RingIter<T, N> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.finished {
            None
        } else {
            let item = self.buf[self.left];
            if self.left == self.right {
                self.finished = true;
            }
            self.left = (self.left + 1) % N;
            Some(item)
        }
    }
}

impl<T: Copy + Default, const N: usize> DoubleEndedIterator for RingIter<T, N> {
    fn next_back(&mut self) -> Option<Self::Item> {
        if self.finished {
            None
        } else {
            let item = self.buf[self.right];
            if self.left == self.right {
                self.finished = true;
            }
            self.right = if self.right == 0 {
                N - 1
            } else {
                self.right - 1
            };
            Some(item)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_size() {
        let ring: Ring<i32, 64> = Ring::new();
        assert_eq!(ring.len(), 0);
    }

    #[test]
    fn test_is_empty() {
        let ring: Ring<i32, 64> = Ring::new();
        assert!(ring.is_empty());
    }

    #[test]
    fn test_push() {
        let mut ring: Ring<i32, 64> = Ring::new();
        ring.push(314);
        assert_eq!(ring.len(), 1);
    }

    #[test]
    fn test_push_on_zero_size() {
        let mut ring: Ring<i32, 0> = Ring::new();
        ring.push(314);
        assert_eq!(ring.len(), 0);
    }

    #[test]
    fn test_push_wrapping() {
        let mut ring: Ring<i32, 2> = Ring::new();
        ring.push(314);
        assert_eq!(ring.len(), 1);
        ring.push(628);
        assert_eq!(ring.len(), 2);
        ring.push(42);
        assert_eq!(ring.len(), 2);
    }

    #[test]
    fn test_into_iter_zero_size() {
        let ring: Ring<i32, 0> = Ring::new();
        let mut iter = ring.into_iter();
        assert!(iter.next().is_none());
        assert!(iter.next_back().is_none());
    }

    #[test]
    fn test_into_iter_empty() {
        let ring: Ring<i32, 3> = Ring::new();
        assert!(ring.into_iter().next().is_none());
    }

    #[test]
    fn test_into_iter_one_item() {
        let mut ring: Ring<i32, 3> = Ring::new();
        ring.push(1);
        let mut iter = ring.into_iter();
        assert_eq!(iter.next(), Some(1));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn test_into_iter_partially_filled() {
        let mut ring: Ring<i32, 3> = Ring::new();
        ring.push(1);
        ring.push(2);
        let mut iter = ring.into_iter();
        assert_eq!(iter.next(), Some(1));
        assert_eq!(iter.next(), Some(2));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn test_into_iter_exactly_filled() {
        let mut ring: Ring<i32, 3> = Ring::new();
        ring.push(1);
        ring.push(2);
        ring.push(3);
        let mut iter = ring.into_iter();
        assert_eq!(iter.next(), Some(1));
        assert_eq!(iter.next(), Some(2));
        assert_eq!(iter.next(), Some(3));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn test_into_iter_wrapped_around() {
        let mut ring: Ring<i32, 3> = Ring::new();
        ring.push(1);
        ring.push(2);
        ring.push(3);
        ring.push(4);
        let mut iter = ring.into_iter();
        assert_eq!(iter.next(), Some(2));
        assert_eq!(iter.next(), Some(3));
        assert_eq!(iter.next(), Some(4));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn test_into_iter_reverse_empty() {
        let ring: Ring<i32, 3> = Ring::new();
        let mut iter = ring.into_iter().rev();
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn test_into_iter_reverse_partially_filled() {
        let mut ring: Ring<i32, 3> = Ring::new();
        ring.push(1);
        ring.push(2);
        let mut iter = ring.into_iter().rev();
        assert_eq!(iter.next(), Some(2));
        assert_eq!(iter.next(), Some(1));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn test_into_iter_reverse_exactly_filled() {
        let mut ring: Ring<i32, 3> = Ring::new();
        ring.push(1);
        ring.push(2);
        ring.push(3);
        let mut iter = ring.into_iter().rev();
        assert_eq!(iter.next(), Some(3));
        assert_eq!(iter.next(), Some(2));
        assert_eq!(iter.next(), Some(1));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn test_into_iter_reverse_wrapped_around() {
        let mut ring: Ring<i32, 3> = Ring::new();
        ring.push(1);
        ring.push(2);
        ring.push(3);
        ring.push(4);
        let mut iter = ring.into_iter().rev();
        assert_eq!(iter.next(), Some(4));
        assert_eq!(iter.next(), Some(3));
        assert_eq!(iter.next(), Some(2));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn test_into_iter_meet_in_the_middle() {
        let mut ring: Ring<i32, 3> = Ring::new();
        ring.push(1);
        ring.push(2);
        ring.push(3);
        ring.push(4);
        let mut iter = ring.into_iter();
        assert_eq!(iter.next(), Some(2));
        assert_eq!(iter.next_back(), Some(4));
        assert_eq!(iter.next(), Some(3));
        assert_eq!(iter.next_back(), None);
    }

    #[test]
    fn test_is_saturated() {
        let mut ring: Ring<i32, 2> = Ring::new();
        assert!(!ring.is_saturated());
        ring.push(1);
        assert!(!ring.is_saturated());
        ring.push(2);
        assert!(ring.is_saturated());
        ring.push(3);
        assert!(ring.is_saturated());
    }
}
