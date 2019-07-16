//! # Digital Signal Processing
//!
//! Signals can be processed in Time or Frequency domain
//!

pub mod generators;
pub mod fft;
pub mod spectrums;
pub mod windows;
pub mod signalops;
mod vectors;

use num_complex::Complex32;


/// Time domain data buffer. Uses Real number
pub type RealBuffer = Vec<f32>;

/// Frequency domain data buffer based on complex numbers
pub type ComplexBuffer = Vec<Complex32>;

/// Node which produces signal
pub trait SourceNode {
    type Buffer;
    
    fn next_batch(&mut self) -> &Self::Buffer;
}

// signal transformation
pub trait ProcessingNode {
    type InBuffer;
    type OutBuffer;
    
    fn process(&mut self, input: &Self::InBuffer) -> &Self::OutBuffer;
}

// Consume signal
pub trait SinkNode {
    type Buffer;
    
    fn consume(&mut self, input: &Self::Buffer);
}

pub struct RealToComplexNode {
    output: ComplexBuffer,
}

impl RealToComplexNode {
    pub fn new(size: usize) -> RealToComplexNode {
        RealToComplexNode {output: vec![Complex32::new(0.0, 0.0); size]}
    }
}

impl ProcessingNode for RealToComplexNode {
    type InBuffer = RealBuffer;
    type OutBuffer = ComplexBuffer;
    
    fn process(&mut self, input: &RealBuffer) -> &ComplexBuffer {
        let n = usize::min(input.len(), self.output.len());
        for i in 0..n {
            self.output[i] = Complex32::new(input[i], 0.0);
        }
        &self.output
    }
}

pub struct ComplexToRealNode {
    output: RealBuffer,
}

impl ComplexToRealNode {
    pub fn new(size: usize) -> ComplexToRealNode {
        ComplexToRealNode {output: vec![0.0; size]}
    }
}

impl ProcessingNode for ComplexToRealNode {
    type InBuffer = ComplexBuffer;
    type OutBuffer = RealBuffer;
    
    fn process(&mut self, input: &ComplexBuffer) ->&RealBuffer {
        let n = usize::min(input.len(), self.output.len());
        for i in 0..n {
            self.output[i] = input[i].re;
        }
        &self.output
    }
}

/// ------------------------------------------------------------------------------------------------
/// Module unit tests
/// ------------------------------------------------------------------------------------------------
#[cfg(test)]
mod tests {
    use num_complex::Complex32;
    use crate::generators::ImpulseGen;
    use super::*;

    #[test]
    fn test_real_to_complex() {
        let real = vec![1.0, 2.0, 3.0, 4.0];
        let expected = vec![Complex32::new(1.0, 0.0), 
                            Complex32::new(2.0, 0.0),
                            Complex32::new(3.0, 0.0),
                            Complex32::new(4.0, 0.0)];
        let mut rtc = RealToComplexNode::new(4);
        let complex = rtc.process(&real);
        assert_eq!(complex, &expected);
    }

    #[test]
    fn test_complex_to_real() {
        let complex = vec![Complex32::new(1.0, 0.0), 
                           Complex32::new(2.0, 0.0),
                           Complex32::new(3.0, 0.0),
                           Complex32::new(4.0, 0.0)];
        let expected = vec![1.0, 2.0, 3.0, 4.0];
        let mut ctr = ComplexToRealNode::new(4);
        let real = ctr.process(&complex);
        assert_eq!(real, &expected);
    }

}
