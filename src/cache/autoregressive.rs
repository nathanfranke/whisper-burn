//use alloc::vec;

use super::{CacheState, TensorCache};
use burn::tensor::backend::Backend;
use burn::tensor::Tensor;

impl<B: Backend, const D: usize> TensorCache<B, D> {
    pub fn forward_autoregressive<F>(
        &mut self,
        tensor: Tensor<B, 3>,
        dim_cat: usize,
        func: F,
    ) -> Tensor<B, D>
    where
        F: Fn(Tensor<B, 3>) -> Tensor<B, D>,
    {
        /*let mut tensor_old = CacheState::Empty;
        core::mem::swap(&mut self.state, &mut tensor_old);

        let tensor_new = match tensor_old {
            CacheState::Value(tensor_old) => {
                let [batch_size, seq_length, d_model] = tensor.dims();
                let next_seq_token =
                    tensor.slice([0..batch_size, (seq_length - 1)..seq_length, 0..d_model]);
                let next_seq_token = func(next_seq_token);

                Tensor::cat(vec![tensor_old, next_seq_token], dim_cat)
            }
            _ => func(tensor),
        };

        self.state = CacheState::Value(tensor_new.clone());
        tensor_new*/

        let t = func(tensor);
        //self.state = CacheState::Value(t.clone());
        return t;
    }

    pub fn forward_full<F>(&mut self, tensor: Tensor<B, 3>, func: F) -> Tensor<B, D>
    where
        F: Fn(Tensor<B, 3>) -> Tensor<B, D>,
    {
        /*let new_state = match self.state.as_ref() {
            Some(t) => t.clone(), 
            None => func(tensor)
        };

        self.state = Some( new_state.clone() );

        new_state*/
        

        let mut tensor_old = CacheState::Empty;
        core::mem::swap(&mut self.state, &mut tensor_old);

        let tensor_new = match tensor_old {
            CacheState::Value(tensor_old) => tensor_old,
            _ => func(tensor),
        };

        self.state = CacheState::Value(tensor_new.clone());
        tensor_new
    }
}