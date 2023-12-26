#![no_std]

//! A convenient implementation of a Bitmap over 
//! a vector u64

use alloc::vec::Vec;

#[macro_use]
extern crate alloc;


const MIN_BITMAP_SIZE: usize  = 64;
pub struct Bitmap {

    rounded_size: usize,
    mask        : usize,
    log_sz      : usize,


    bitmap      : Vec<u64>,

    /// This is the start-bit within the Bitmap
    /// The space before this bit is skipped
    /// by the `find_free_slot` function 
    start_bit   : usize,    
}

impl Bitmap {

    fn roundup_pow_of_two(x: usize) -> usize {
        let mut x = x - 1;
        x |= x >> 1;
        x |= x >> 2;
        x |= x >> 4;
        x |= x >> 8;
        x |= x >> 16;
        x = x+1;
        return x;
    }

    /// Returns a new instance of the Bitmap
    /// the size argument is in `bits`
    pub fn new(size: usize) -> Self {
        let size = if size < MIN_BITMAP_SIZE {
            MIN_BITMAP_SIZE
        } else {
            size
        };

        let rounded_size = Bitmap::roundup_pow_of_two(size);

        // These are fixed
        let log_sz: usize = 6;    // 2**6 = 64
        let mask: usize   = 0x3f; // 64 - 1

        // Calculate how many 0u64 are needed 
        // for the requested size
        let mut bitmap_size = rounded_size >> log_sz;
        if (rounded_size & mask) != 0 {
            bitmap_size += 1;
        }

        Self {
            rounded_size: rounded_size,
            mask:         mask, 
            log_sz:       log_sz,    
            start_bit:    0,
            bitmap:       vec![0u64; bitmap_size]
        }        
    }

    pub fn new_with_reserved(size: usize, reserved_space: usize) -> Self {
        assert!(reserved_space < size);

        let mut instance = Self::new(size);
        instance.start_bit = reserved_space;
        instance
    }

    pub fn set_bit(&mut self, bit: usize) {
        assert!(bit < self.rounded_size);

        let selected_mask   = bit >> self.log_sz;
        let mask_bit        = bit & self.mask;

        self.bitmap[selected_mask] |= 1u64 << mask_bit;
    }

    pub fn unset_bit(&mut self, bit: usize) {
        assert!(bit < self.rounded_size);

        let selected_mask   = bit >> self.log_sz;
        let mask_bit        = bit & self.mask;

        self.bitmap[selected_mask] &= !(1u64 << mask_bit);
    }

    pub fn is_set(&self, bit: usize) -> bool {
        assert!(bit < self.rounded_size);

        let selected_mask   = bit >> self.log_sz;
        let mask_bit        = bit & self.mask;

        ((self.bitmap[selected_mask] >> mask_bit) & 1) == 1
    }

    /// Finds the first unused bit in the bitmap
    pub fn find_free_slot(&self) -> Option<usize> {
        for bit in self.start_bit..self.rounded_size {
            let selected_mask   = bit >> self.log_sz;
            let mask_bit        = bit & self.mask;

            // If the bit is not set, return it
            if ((self.bitmap[selected_mask] >> mask_bit) & 1) == 0 {
                return Some(bit);
            }
        }
        None
    }

    /// Returns the space of the bitmap in bits
    pub fn bit_size(&self) -> usize {
        self.rounded_size
    }

    /// Returns the actual consumed space of the Bitmap
    pub fn size(&self) -> usize {
        self.bitmap.len()
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    

    /// Test the simplest case of a single u64
    #[test]
    fn test_bitmap1() {
        let mut bitmap = Bitmap::new(64);
        assert_eq!(bitmap.bit_size(), 64);
        assert_eq!(bitmap.size(), 1);

        bitmap.set_bit(0);

        assert_eq!(bitmap.find_free_slot(), Some(1usize));
    }

    /// Test the roundup function
    #[test]
    fn test_bitmap2() {
        let bitmap = Bitmap::new(20);
        assert_eq!(bitmap.bit_size(), 64);
        assert_eq!(bitmap.size(), 1);
    }

    /// Test the roundup function2
    #[test]
    fn test_bitmap3() {
        let bitmap = Bitmap::new(65);
        assert_eq!(bitmap.bit_size(), 128);
        assert_eq!(bitmap.size(), 2);
    }

    /// Test reserved space feature
    #[test]
    fn test_bitmap4() {
        let mut bitmap = Bitmap::new_with_reserved(65, 20);
        assert_eq!(bitmap.bit_size(), 128);
        assert_eq!(bitmap.size(), 2);

        bitmap.set_bit(0);

        assert_eq!(bitmap.find_free_slot(), Some(20usize));
    }

    /// Test set / unset
    #[test]
    fn test_bitmap5() {
        let mut bitmap = Bitmap::new(65);
        assert_eq!(bitmap.bit_size(), 128);
        assert_eq!(bitmap.size(), 2);

        bitmap.set_bit(0);
        assert_eq!(bitmap.find_free_slot(), Some(1usize));
        bitmap.set_bit(1);
        assert_eq!(bitmap.find_free_slot(), Some(2usize));
        bitmap.unset_bit(0);
        assert_eq!(bitmap.find_free_slot(), Some(0usize));
    }
}
