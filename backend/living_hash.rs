use sha3::{Digest, KeccakF1600};
use serde::{Serialize, Deserialize};
use std::fmt;

/// Represents the phase of the sponge operation
#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum SpongePhase {
    Absorb,
    Squeeze,
}

/// Represents the state of the Keccak sponge at a specific step
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SpongeState {
    pub step_index: usize,       // Index of the current step
    pub phase: SpongePhase,      // Absorb or Squeeze phase
    pub input: Vec<u8>,          // Input data for this step
    pub state: [u64; 25],        // Internal state (1600 bits)
    pub diffs: [u64; 25],        // XOR differences from the previous state
}

impl fmt::Display for SpongeState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "Step {}: {:?}", self.step_index, self.phase)?;
        writeln!(f, "Input: {:?}", self.input)?;
        writeln!(f, "State: {:?}", self.state)?;
        writeln!(f, "Diffs: {:?}", self.diffs)?;
        Ok(())
    }
}

/// The Living Hash Engine, representing a Keccak sponge with traceability
pub struct LivingHash {
    trace: Vec<SpongeState>,    // Trace log of all states
    current_state: [u64; 25],   // Current state of the sponge
    rate: usize,                // Absorption/squeezing rate (bits)
    capacity: usize,            // Cryptographic capacity (bits)
}

impl LivingHash {
    /// Creates a new Living Hash Engine instance
    pub fn new(rate: usize, capacity: usize) -> Self {
        assert_eq!(rate + capacity, 1600, "Rate and capacity must sum to 1600 bits!");
        Self {
            trace: Vec::new(),
            current_state: [0u64; 25],
            rate,
            capacity,
        }
    }

    /// Absorbs input into the sponge and records the state
    pub fn absorb(&mut self, input: &[u8]) -> Result<(), String> {
        let mut keccak = KeccakF1600::default();
        keccak.set_state(&self.current_state);

        let mut padded_input = input.to_vec();
        self.apply_padding(&mut padded_input)?;

        let prev_state = self.current_state;
        keccak.absorb(&padded_input);

        self.current_state = keccak.clone().state();
        let diffs = Self::calculate_diffs(&prev_state, &self.current_state);

        self.trace.push(SpongeState {
            step_index: self.trace.len(),
            phase: SpongePhase::Absorb,
            input: padded_input,
            state: self.current_state,
            diffs,
        });

        Ok(())
    }

    /// Squeezes output from the sponge and records the state
    pub fn squeeze(&mut self, output_length: usize) -> Result<Vec<u8>, String> {
        let mut keccak = KeccakF1600::default();
        keccak.set_state(&self.current_state);

        let prev_state = self.current_state;
        let output = keccak.squeeze(output_length);

        self.current_state = keccak.clone().state();
        let diffs = Self::calculate_diffs(&prev_state, &self.current_state);

        self.trace.push(SpongeState {
            step_index: self.trace.len(),
            phase: SpongePhase::Squeeze,
            input: output.clone(),
            state: self.current_state,
            diffs,
        });

        Ok(output)
    }

    /// Retrieves the full trace log of the sponge's operation
    pub fn get_trace(&self) -> &Vec<SpongeState> {
        &self.trace
    }

    /// Retrieves a specific trace step by index
    pub fn get_trace_step(&self, index: usize) -> Option<&SpongeState> {
        self.trace.get(index)
    }

    /// Applies Keccak padding to the input
    fn apply_padding(&self, input: &mut Vec<u8>) -> Result<(), String> {
        let rate_in_bytes = self.rate / 8;
        let mut pad_len = rate_in_bytes - (input.len() % rate_in_bytes);
        if pad_len == 0 {
            pad_len = rate_in_bytes;
        }

        input.push(0x01); // Padding start
        for _ in 1..pad_len - 1 {
            input.push(0x00); // Zero padding
        }
        input.push(0x80); // Padding end

        if input.len() % rate_in_bytes != 0 {
            return Err("Padding failed to align input to rate size".to_string());
        }

        Ok(())
    }

    /// Calculates XOR differences between two states
    fn calculate_diffs(prev_state: &[u64; 25], current_state: &[u64; 25]) -> [u64; 25] {
        let mut diffs = [0u64; 25];
        for i in 0..25 {
            diffs[i] = prev_state[i] ^ current_state[i];
        }
        diffs
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_absorb_and_squeeze() {
        let mut engine = LivingHash::new(1088, 512);
        let input = b"hello world";

        // Absorb input
        assert!(engine.absorb(input).is_ok());

        // Squeeze output
        let output = engine.squeeze(32);
        assert!(output.is_ok());
        let output = output.unwrap();

        // Verify trace
        let trace = engine.get_trace();
        assert_eq!(trace.len(), 2); // One absorb and one squeeze step
        assert_eq!(trace[0].phase, SpongePhase::Absorb);
        assert_eq!(trace[1].phase, SpongePhase::Squeeze);

        // Verify output length
        assert_eq!(output.len(), 32);
    }

    #[test]
    fn test_padding() {
        let mut engine = LivingHash::new(1088, 512);
        let mut input = vec![0x00; 136]; // 136 bytes == 1088 bits
        assert!(engine.apply_padding(&mut input).is_ok());
        assert_eq!(input.len(), 144); // Should be padded to next multiple of 136
    }

    #[test]
    #[should_panic]
    fn test_invalid_rate_capacity() {
        LivingHash::new(1024, 512); // This should panic because rate + capacity != 1600
    }
}