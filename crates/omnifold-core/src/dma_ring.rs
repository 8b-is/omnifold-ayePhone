//! dma_ring — zero-allocation fixed-slot DMA ring buffer for feeding `CrabCC`
//! symbol slices and bio-harmonic sensory frames into Apple Silicon Unified
//! Memory (`UMA`).
//!
//! Designed so the hot path performs zero heap allocations: storage is inline
//! (`[[u8; SLOT_BYTES]; CAP]`), and every frame carries an FNV-1a content
//! digest that can be committed directly to the `mem8` continuity ledger.

use crate::capability::fnv1a_step;

const FNV_OFFSET: u64 = 0xcbf2_9ce4_8422_2325;

/// Metadata and fixed-size payload of one DMA ring slot.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DmaFrame<const SLOT_BYTES: usize> {
    pub seq: u64,
    pub len: usize,
    pub digest: u64,
    pub bytes: [u8; SLOT_BYTES],
}

impl<const SLOT_BYTES: usize> DmaFrame<SLOT_BYTES> {
    pub const fn empty() -> Self {
        Self {
            seq: 0,
            len: 0,
            digest: FNV_OFFSET,
            bytes: [0u8; SLOT_BYTES],
        }
    }

    pub fn payload(&self) -> &[u8] {
        &self.bytes[..self.len]
    }
}

/// Errors returned by [`DmaRing`] operations.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DmaRingError {
    /// Input slice exceeds `SLOT_BYTES`.
    PayloadTooLarge { given: usize, max: usize },
    /// All `CAP` slots are currently unread.
    RingFull,
}

/// Fixed-capacity, zero-allocation ring buffer.
#[derive(Debug, Clone)]
pub struct DmaRing<const CAP: usize, const SLOT_BYTES: usize> {
    slots: [DmaFrame<SLOT_BYTES>; CAP],
    head: usize,
    tail: usize,
    count: usize,
    next_seq: u64,
    rolling_digest: u64,
}

impl<const CAP: usize, const SLOT_BYTES: usize> Default for DmaRing<CAP, SLOT_BYTES> {
    fn default() -> Self {
        Self::new()
    }
}

impl<const CAP: usize, const SLOT_BYTES: usize> DmaRing<CAP, SLOT_BYTES> {
    pub const fn new() -> Self {
        Self {
            slots: [DmaFrame::empty(); CAP],
            head: 0,
            tail: 0,
            count: 0,
            next_seq: 1,
            rolling_digest: FNV_OFFSET,
        }
    }

    pub const fn capacity(&self) -> usize {
        CAP
    }

    pub const fn slot_bytes(&self) -> usize {
        SLOT_BYTES
    }

    pub const fn len(&self) -> usize {
        self.count
    }

    pub const fn is_empty(&self) -> bool {
        self.count == 0
    }

    pub const fn is_full(&self) -> bool {
        self.count == CAP
    }

    pub const fn rolling_digest(&self) -> u64 {
        self.rolling_digest
    }

    /// Feed a frame into the ring without heap allocation. Returns `(seq, frame_digest)`.
    pub fn feed(&mut self, data: &[u8]) -> Result<(u64, u64), DmaRingError> {
        if data.len() > SLOT_BYTES {
            return Err(DmaRingError::PayloadTooLarge {
                given: data.len(),
                max: SLOT_BYTES,
            });
        }
        if self.count == CAP {
            return Err(DmaRingError::RingFull);
        }

        let seq = self.next_seq;
        self.next_seq += 1;
        let digest = fnv1a_step(FNV_OFFSET, data);

        let slot = &mut self.slots[self.tail];
        slot.seq = seq;
        slot.len = data.len();
        slot.digest = digest;
        slot.bytes[..data.len()].copy_from_slice(data);
        if data.len() < SLOT_BYTES {
            slot.bytes[data.len()..].fill(0);
        }

        self.tail = (self.tail + 1) % CAP;
        self.count += 1;
        self.rolling_digest = fnv1a_step(self.rolling_digest, &seq.to_le_bytes());
        self.rolling_digest = fnv1a_step(self.rolling_digest, &digest.to_le_bytes());

        Ok((seq, digest))
    }

    /// Consume the oldest unread frame from the ring.
    pub fn drain_one(&mut self) -> Option<DmaFrame<SLOT_BYTES>> {
        if self.count == 0 {
            return None;
        }
        let frame = self.slots[self.head];
        self.head = (self.head + 1) % CAP;
        self.count -= 1;
        Some(frame)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ring_feeds_drains_and_wraps_without_alloc() {
        let mut ring: DmaRing<4, 64> = DmaRing::new();
        for i in 0..4u8 {
            let (seq, _) = ring.feed(&[i; 8]).unwrap();
            assert_eq!(seq, u64::from(i) + 1);
        }
        assert!(ring.is_full());
        assert_eq!(ring.feed(b"overflow"), Err(DmaRingError::RingFull));

        let f0 = ring.drain_one().unwrap();
        assert_eq!(f0.seq, 1);
        assert_eq!(f0.payload(), &[0u8; 8]);

        let (seq5, _) = ring.feed(b"crabcc::symbol::Omnifold").unwrap();
        assert_eq!(seq5, 5);
        assert_eq!(ring.len(), 4);
    }

    #[test]
    fn oversized_payload_is_rejected() {
        let mut ring: DmaRing<2, 16> = DmaRing::new();
        assert_eq!(
            ring.feed(&[0u8; 17]),
            Err(DmaRingError::PayloadTooLarge { given: 17, max: 16 })
        );
    }
}
