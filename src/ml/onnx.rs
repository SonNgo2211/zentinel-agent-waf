//! Tiny ONNX Model Inference for Fast Path
//!
//! Provides high-performance execution of compressed ML models
//! for real-time attack detection.

use anyhow::Result;
use tracing::{debug, info};

#[cfg(feature = "ml-onnx")]
use tract_onnx::prelude::*;

pub struct TinyModel {
    #[cfg(feature = "ml-onnx")]
    model: RunnableModel<TypedFact, Box<dyn TypedOp>, Graph<TypedFact, Box<dyn TypedOp>>>,
}

impl TinyModel {
    #[cfg(feature = "ml-onnx")]
    pub fn load(path: &str) -> Result<Self> {
        info!(path = path, "Loading tiny ONNX model for fast path");
        let model = tract_onnx::onnx()
            .model_for_path(path)?
            .with_input_fact(0, f32::fact(&[1, 128]).into())? // Example input shape
            .into_optimized()?
            .into_runnable()?;
        
        Ok(Self { model })
    }

    #[cfg(not(feature = "ml-onnx"))]
    pub fn load(_path: &str) -> Result<Self> {
        anyhow::bail!("ML-ONNX feature is not enabled")
    }

    pub fn predict(&self, _features: &[f32]) -> Result<f32> {
        #[cfg(feature = "ml-onnx")]
        {
            let tensor = tract_ndarray::Array2::from_shape_vec((1, 128), _features.to_vec())?.into();
            let result = self.model.run(tvec!(tensor))?;
            let output = result[0].to_array_view::<f32>()?;
            Ok(output[[0, 0]]) // Return first output as confidence
        }
        #[cfg(not(feature = "ml-onnx"))]
        {
            Ok(0.0)
        }
    }
}
