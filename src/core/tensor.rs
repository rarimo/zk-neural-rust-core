use image::{EncodableLayout, imageops::FilterType};
use num_traits::{Float, FromBytes, PrimInt, ToBytes, ToPrimitive};
use tflitec::{
    interpreter::Interpreter,
    model::Model,
    tensor::{DataType, Shape},
};

use crate::{ZKNeuralError, core::face_detection::FaceDetector};
use serde::{Deserialize, Serialize};
use serde_json::Value;

pub struct TensorInvoker {
    pub model_data: Vec<u8>,
    pub input_shape: Shape,
    pub input_data_type: DataType,
    pub should_process: bool,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct BionettaGenericInputs {
    pub ultra_groth: String,
    pub address: String,
    pub threshold: String,
    pub nonce: String,
    pub features: Vec<String>,
    pub image: Vec<String>,
    pub rand: String,
}

#[repr(C)]
pub enum ImagePreprocessing {
    None,
    FaceRecognition,
}

const NEURAL_SIGNAL_MULTIPLIER: f64 = 32768.0;

impl TensorInvoker {
    pub fn new(model_data: &[u8], should_process: bool) -> Result<Self, ZKNeuralError> {
        let model = Model::from_bytes(&model_data)?;

        let interpreter = Interpreter::new(&model, None)?;

        interpreter.allocate_tensors()?;

        let input = interpreter.input(0)?;

        let input_shape = input.shape().clone();

        let input_data_type = input.data_type();

        Ok(TensorInvoker {
            model_data: model_data.to_vec(),
            input_shape,
            input_data_type,
            should_process,
        })
    }

    pub fn prepare_image_by_spec(
        &self,
        image_data: &[u8],
        image_preprocessing: ImagePreprocessing,
    ) -> Result<(Vec<u8>, Vec<String>), ZKNeuralError> {
        let preprocessed_image_data = match image_preprocessing {
            ImagePreprocessing::FaceRecognition => {
                FaceDetector::detect_face(image_data)?.as_bytes().to_vec()
            }
            ImagePreprocessing::None => image_data.to_vec(),
        };

        let input_dimentions = self.input_shape.dimensions();
        if input_dimentions.len() != 4 {
            return Err(ZKNeuralError::ModelNotFourDimensional);
        }

        let width = input_dimentions[1];
        let height = input_dimentions[2];
        let channels = input_dimentions[3];

        let loaded_image = image::load_from_memory(&preprocessed_image_data)?.resize_exact(
            width as u32,
            height as u32,
            FilterType::CatmullRom,
        );

        let prepared_image: Vec<u8> = match channels {
            1 => loaded_image.grayscale().as_bytes().to_vec(),
            3 => loaded_image.to_rgb8().as_bytes().to_vec(),
            _ => return Err(ZKNeuralError::InvalidModelChannel),
        };

        match self.input_data_type {
            DataType::Uint8 => Ok(prepare_data_by_type::<u8>(prepared_image)),
            DataType::Int16 => Ok(prepare_data_by_type::<i16>(prepared_image)),
            DataType::Int32 => Ok(prepare_data_by_type::<i32>(prepared_image)),
            DataType::Int64 => Ok(prepare_data_by_type::<i64>(prepared_image)),
            DataType::Float32 => Ok(prepare_data_by_float_type::<f32>(prepared_image)),
            DataType::Float64 => Ok(prepare_data_by_float_type::<f64>(prepared_image)),
            _ => Err(ZKNeuralError::InvalidModelDataType),
        }
    }

    pub fn fire(&self, data: &[u8]) -> Result<Vec<u8>, ZKNeuralError> {
        let model = Model::from_bytes(&self.model_data)?;

        let interpreter = Interpreter::new(&model, None)?;

        interpreter.allocate_tensors()?;

        interpreter.copy(data, 0)?;

        interpreter.invoke()?;

        let output_tensor = interpreter.output(0)?;

        let output_data = output_tensor.data::<u8>().to_vec();

        match output_tensor.data_type() {
            DataType::Uint8 => {
                let collected = collect_processed_data_to::<u8>(output_data);

                return Ok(serde_json::to_vec(&collected)?);
            }
            DataType::Int16 => {
                let collected = collect_processed_data_to::<i16>(output_data);

                return Ok(serde_json::to_vec(&collected)?);
            }
            DataType::Int32 => {
                let collected = collect_processed_data_to::<i32>(output_data);

                return Ok(serde_json::to_vec(&collected)?);
            }
            DataType::Int64 => {
                let collected = collect_processed_data_to::<i64>(output_data);

                return Ok(serde_json::to_vec(&collected)?);
            }
            DataType::Float32 => {
                let collected =
                    collect_processed_data_to_float::<f32>(output_data, self.should_process);

                return Ok(serde_json::to_vec(&collected)?);
            }
            DataType::Float64 => {
                let collected =
                    collect_processed_data_to_float::<f64>(output_data, self.should_process);

                return Ok(serde_json::to_vec(&collected)?);
            }
            _ => {
                return Err(ZKNeuralError::InvalidModelDataType);
            }
        }
    }

    pub fn drain_generic_inputs(
        &self,
        address: String,
        threshold: String,
        nonce: String,
        image_data: &[u8],
        image_preprocessing: ImagePreprocessing,
    ) -> Result<Vec<u8>, ZKNeuralError> {
        let (data, signal_data) = self.prepare_image_by_spec(image_data, image_preprocessing)?;

        let serialized_features = self.fire(&data)?;

        let features = parse_json_numbers_to_strings_unchecked(&serialized_features);

        let inputs = BionettaGenericInputs {
            ultra_groth: "1".to_string(),
            address,
            threshold,
            nonce,
            features: features,
            image: signal_data,
            rand: "0".to_string(),
        };

        Ok(serde_json::to_vec(&inputs)?)
    }
}

pub fn prepare_data_by_float_type<T: Float + ToBytes>(data: Vec<u8>) -> (Vec<u8>, Vec<String>) {
    let mut float_data: Vec<T> = vec![];
    for &byte in data.iter() {
        let float_value = T::from(byte).expect("Failed to convert byte to float type");

        let multiplier = T::from(255.0).expect("Failed to convert 255.0 to float type");

        float_data.push(float_value / multiplier);
    }

    let result_data = float_data
        .clone()
        .into_iter()
        .map(|f| f.to_le_bytes().as_ref().to_vec())
        .flatten()
        .collect();

    let result_signal_data: Vec<String> = float_data
        .into_iter()
        .map(|f| {
            (f.to_f64().unwrap() * NEURAL_SIGNAL_MULTIPLIER)
                .to_i64()
                .unwrap()
                .to_string()
        })
        .collect();

    (result_data, result_signal_data)
}

pub fn prepare_data_by_type<T: PrimInt + ToBytes + ToString>(
    data: Vec<u8>,
) -> (Vec<u8>, Vec<String>) {
    let mut int_data: Vec<T> = vec![];
    for &byte in data.iter() {
        let int_value = T::from(byte).expect("Failed to convert byte to integer type");
        int_data.push(int_value);
    }

    let result_data: Vec<u8> = int_data
        .clone()
        .into_iter()
        .map(|i| i.to_le_bytes().as_ref().to_vec())
        .flatten()
        .collect();

    let result_signals: Vec<String> = int_data.into_iter().map(|i| i.to_string()).collect();

    (result_data, result_signals)
}

pub fn collect_processed_data_to_float<T>(data: Vec<u8>, should_process: bool) -> Vec<T>
where
    T: Float + FromBytes + std::fmt::Debug,
    for<'a> &'a [u8]: TryInto<&'a T::Bytes>,
{
    let floats: Vec<T> = data
        .chunks_exact(std::mem::size_of::<T>())
        .map(TryInto::<&T::Bytes>::try_into)
        .map(|x| x.unwrap_or_else(|_| panic!("could not convert slice to array reference!")))
        .map(T::from_le_bytes)
        .collect();

    if !should_process {
        return floats;
    }

    let sum_of_square = floats.iter().fold(T::zero(), |acc, &x| acc + x * x);

    floats
        .into_iter()
        .map(|x| x / sum_of_square.sqrt())
        .collect()
}

pub fn collect_processed_data_to<T>(data: Vec<u8>) -> Vec<T>
where
    T: num_traits::FromBytes,
    for<'a> &'a [u8]: TryInto<&'a T::Bytes>,
{
    data.chunks_exact(std::mem::size_of::<T>())
        .map(TryInto::<&T::Bytes>::try_into)
        .map(|x| x.unwrap_or_else(|_| panic!("could not convert slice to array reference!")))
        .map(T::from_le_bytes)
        .collect()
}

pub fn parse_json_numbers_to_strings_unchecked(json_bytes: &[u8]) -> Vec<String> {
    let value: Value = serde_json::from_slice(json_bytes).expect("Failed to parse JSON");

    let Value::Array(arr) = value else {
        panic!("Expected a JSON array");
    };

    arr.iter()
        .map(|v| {
            let Value::Number(number) = v else {
                panic!("Expected a JSON number");
            };

            if let Some(float_value) = number.as_f64() {
                let signal = (float_value / 255.0) * NEURAL_SIGNAL_MULTIPLIER;

                return signal.to_string();
            }

            number.to_string()
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use std::{fs::File, io::Read};
    use tflitec::{interpreter::Interpreter, model::Model};

    use crate::core::tensor::{ImagePreprocessing, TensorInvoker};

    #[test]
    fn compute() {
        let model_data = File::open("assets/mnist.tflite")
            .unwrap()
            .bytes()
            .collect::<Result<Vec<_>, _>>()
            .unwrap();

        let mut test_image = image::open("assets/mnist_test_image.jpg").unwrap();
        test_image = test_image.grayscale();

        let test_image_bytes = test_image
            .as_bytes()
            .iter()
            .map(|&b| b as f32 / 255.0)
            .collect::<Vec<f32>>();

        let model = Model::from_bytes(&model_data).unwrap();

        let interpreter = Interpreter::new(&model, None).unwrap();

        interpreter.allocate_tensors().unwrap();

        interpreter.copy(&test_image_bytes, 0).unwrap();

        interpreter.invoke().unwrap();

        let output_tensor = interpreter.output(0).unwrap();

        let output_data = output_tensor.data::<f32>();

        println!("Output tensor shape: {:?}", output_data);
    }

    #[test]
    fn test_tensor_invoker() {
        let mut file = File::open("assets/arcface.tflite").unwrap();
        let mut model_data = Vec::new();
        file.read_to_end(&mut model_data).unwrap();

        let invoker = TensorInvoker::new(&model_data, true).unwrap();

        let image_data = File::open("assets/face.jpeg")
            .unwrap()
            .bytes()
            .collect::<Result<Vec<_>, _>>()
            .unwrap();

        let (prepared_image, _) = invoker
            .prepare_image_by_spec(&image_data, ImagePreprocessing::None)
            .unwrap();

        let result = invoker.fire(&prepared_image).unwrap();

        println!("Result: {:?}", String::from_utf8(result).unwrap());
    }

    #[test]
    fn test_inputs_drain() {
        let mut file = File::open("assets/arcface.tflite").unwrap();
        let mut model_data = Vec::new();
        file.read_to_end(&mut model_data).unwrap();

        let invoker = TensorInvoker::new(&model_data, true).unwrap();

        let image_data = File::open("assets/face.jpeg")
            .unwrap()
            .bytes()
            .collect::<Result<Vec<_>, _>>()
            .unwrap();

        let result = invoker
            .drain_generic_inputs(
                "3123123".to_string(),
                "1".to_string(),
                "1".to_string(),
                &image_data,
                ImagePreprocessing::None,
            )
            .unwrap();

        println!("Result: {:?}", String::from_utf8(result).unwrap());
    }
}
