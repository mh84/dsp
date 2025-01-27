
use crate::{RealBuffer, ProcessingNode};


/// Change signal amplitude
/// 
/// Example
/// 
/// ```
/// use assert_approx_eq::assert_approx_eq;    
/// use dsp::{ProcessingNode, SourceNode};
/// use dsp::generators::{SineGen, GenNode};
/// use dsp::signals::GainNode;
/// 
/// let mut gen = GenNode::new(Box::new(SineGen::new(1.0)), 4.0, 4);
/// let mut amplitude_node = GainNode::new(2.0, 4);
/// let signal = gen.next_frame();
/// let scaled_signal = amplitude_node.process(signal);
/// assert_approx_eq!(scaled_signal[0], 0.0, 1e-5f32);
/// assert_approx_eq!(scaled_signal[1], 2.0, 1e-5f32);
/// assert_approx_eq!(scaled_signal[2], 0.0, 1e-5f32);
/// assert_approx_eq!(scaled_signal[3], -2.0, 1e-5f32);
/// ```
pub struct GainNode {
    scale: f32,
    output: RealBuffer,
}

impl GainNode {
    pub fn new(scale: f32, frame_size: usize) -> GainNode {
        GainNode { scale, output: vec![0.0; frame_size] }
    }
}

impl ProcessingNode for GainNode {
    type InBuffer = RealBuffer;
    type OutBuffer = RealBuffer;
    
    fn process(&mut self, input: &Self::InBuffer) -> &RealBuffer {
        let n = usize::min(input.len(), self.output.len());
        for i in 0..n {
            self.output[i] = self.scale * input[i];
        }
        &self.output
    }

}


/// Sum several signals
pub struct SumNode {
    output: RealBuffer,
}

impl SumNode {
    pub fn new(frame_size: usize) -> SumNode {
        SumNode { output: vec![0.0; frame_size] }
    }
    
    pub fn process(&mut self, input1: &RealBuffer, input2: &RealBuffer) -> &RealBuffer {
        let n = usize::min(usize::min(input1.len(), input2.len()), self.output.len());
        for i in 0..n {
            self.output[i] = input1[i] + input2[i];
        }
        &self.output
    }

}


/// ------------------------------------------------------------------------------------------------
/// Module unit tests
/// ------------------------------------------------------------------------------------------------
#[cfg(test)]
mod tests {
    use assert_approx_eq::assert_approx_eq;    
    use crate::{ProcessingNode, SourceNode};
    use crate::generators::{SineGen, StepGen, GenNode};
    use super::*;

    #[test]
    fn test_gen_node() {
        let mut gen = GenNode::new(Box::new(SineGen::new(1.0)), 4.0, 4);
        let mut gain_node = GainNode::new(2.0, 4);
        let signal = gen.next_frame();
        let scaled_signal = gain_node.process(signal);

        assert_approx_eq!(scaled_signal[0], 0.0, 1e-5f32);
        assert_approx_eq!(scaled_signal[1], 2.0, 1e-5f32);
        assert_approx_eq!(scaled_signal[2], 0.0, 1e-5f32);
        assert_approx_eq!(scaled_signal[3], -2.0, 1e-5f32);
    }

    #[test]
    fn test_sum_node() {
        let mut sine_gen = GenNode::new(Box::new(SineGen::new(1.0)), 4.0, 4);
        let mut step_gen = GenNode::new(Box::new(StepGen::new(0.2)), 4.0, 4);
        let mut gain_node = GainNode::new(0.5, 4);
        let mut sum_node = SumNode::new(4);

        let sine_signal = sine_gen.next_frame();
        let frame1 = gain_node.process(sine_signal);
        let frame2 = step_gen.next_frame();
        let sum_signal = sum_node.process(frame1, frame2);
        
        assert_approx_eq!(sum_signal[0], 0.0, 1e-5f32);
        assert_approx_eq!(sum_signal[1], 1.5, 1e-5f32);
        assert_approx_eq!(sum_signal[2], 1.0, 1e-5f32);
        assert_approx_eq!(sum_signal[3], 0.5, 1e-5f32);
    }
}