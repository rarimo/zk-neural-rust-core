use image::{EncodableLayout, imageops::FilterType};
use num_traits::{Float, FromBytes, PrimInt, ToBytes};
use tflitec::{
    interpreter::Interpreter,
    model::Model,
    tensor::{DataType, Shape},
};

use crate::ZKNeuralError;

// const MAX_FACE_SCORE: f32 = 0.9;

pub struct TensorInvoker {
    pub model_data: Vec<u8>,
    pub input_shape: Shape,
    pub input_data_type: DataType,
}

impl TensorInvoker {
    pub fn new(model_data: &[u8]) -> Result<Self, ZKNeuralError> {
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
        })
    }

    pub fn prepare_image_by_spec(&self, image_data: &[u8]) -> Result<Vec<u8>, ZKNeuralError> {
        let input_dimentions = self.input_shape.dimensions();
        if input_dimentions.len() != 4 {
            return Err(ZKNeuralError::ModelNotFourDimensional);
        }

        let width = input_dimentions[1];
        let height = input_dimentions[2];
        let channels = input_dimentions[3];

        let loaded_image = image::load_from_memory(image_data)?.resize_exact(
            width as u32,
            height as u32,
            FilterType::CatmullRom,
        );

        loaded_image.save("assets/processed_image.jpg")?;

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

    pub fn fire(&self, data: &[u8], should_process: bool) -> Result<Vec<u8>, ZKNeuralError> {
        let model = Model::from_bytes(&self.model_data)?;

        let interpreter = Interpreter::new(&model, None)?;

        interpreter.allocate_tensors()?;

        interpreter.copy(data, 0)?;

        interpreter.invoke()?;

        let output_tensor = interpreter.output(0)?;

        let output_data = output_tensor.data::<u8>().to_vec();

        match self.input_data_type {
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
                let collected = collect_processed_data_to_float::<f32>(output_data, should_process);

                return Ok(serde_json::to_vec(&collected)?);
            }
            DataType::Float64 => {
                let collected = collect_processed_data_to_float::<f64>(output_data, should_process);

                return Ok(serde_json::to_vec(&collected)?);
            }
            _ => {
                return Err(ZKNeuralError::InvalidModelDataType);
            }
        }
    }
}

fn prepare_data_by_float_type<T: Float + ToBytes>(data: Vec<u8>) -> Vec<u8> {
    let mut float_data: Vec<T> = vec![];
    for &byte in data.iter() {
        let float_value = T::from(byte).expect("Failed to convert byte to float type");

        let multiplier = T::from(255.0).expect("Failed to convert 255.0 to float type");

        float_data.push(float_value / multiplier);
    }

    float_data
        .into_iter()
        .map(|f| f.to_le_bytes().as_ref().to_vec())
        .flatten()
        .collect()
}

fn prepare_data_by_type<T: PrimInt + ToBytes>(data: Vec<u8>) -> Vec<u8> {
    let mut int_data: Vec<T> = vec![];
    for &byte in data.iter() {
        let int_value = T::from(byte).expect("Failed to convert byte to integer type");
        int_data.push(int_value);
    }

    int_data
        .into_iter()
        .map(|i| i.to_le_bytes().as_ref().to_vec())
        .flatten()
        .collect()
}

fn collect_processed_data_to_float<T>(data: Vec<u8>, should_process: bool) -> Vec<T>
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

fn collect_processed_data_to<T>(data: Vec<u8>) -> Vec<T>
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

#[cfg(test)]
mod tests {
    use image::{
        GenericImageView,
        imageops::{FilterType, crop_imm},
    };
    use std::{fs::File, io::Read};
    use tflitec::{interpreter::Interpreter, model::Model};

    use crate::core::{math::sigmoid, tensor::TensorInvoker};

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

        let invoker = TensorInvoker::new(&model_data).unwrap();

        let image_data = File::open("assets/face.jpeg")
            .unwrap()
            .bytes()
            .collect::<Result<Vec<_>, _>>()
            .unwrap();

        let prepared_image = invoker.prepare_image_by_spec(&image_data).unwrap();

        let result = invoker.fire(&prepared_image, true).unwrap();

        println!("Result: {:?}", String::from_utf8(result).unwrap());
    }

    #[test]
    fn test_tensor_find_face() {
        let mut file = File::open("assets/blaze_face_short_range.tflite").unwrap();
        let mut model_data = Vec::new();
        file.read_to_end(&mut model_data).unwrap();

        let image_data = File::open("assets/face3.jpg")
            .unwrap()
            .bytes()
            .collect::<Result<Vec<_>, _>>()
            .unwrap();

        let invoker = TensorInvoker::new(&model_data).unwrap();

        let prepared_image_data = invoker.prepare_image_by_spec(&image_data).unwrap();
        let result_raw = invoker.fire(&prepared_image_data, false).unwrap();

        let result: Vec<f32> = serde_json::from_slice(&result_raw).unwrap();

        println!("Result: {:?}", &result[0..4]);

        let x_center = result[0];
        let y_center = result[1];
        let width = result[2];
        let height = result[3];

        let primary_image = image::load_from_memory(&image_data).unwrap().resize_exact(
            128,
            128,
            FilterType::CatmullRom,
        );

        let cropped = crop_and_resize_face(&primary_image, x_center, y_center, width, height);

        // Save the result
        cropped.save("assets/face_cropped_resized.jpg").unwrap();
    }

    fn crop_and_resize_face(
        image: &image::DynamicImage,
        x_center_raw: f32,
        y_center_raw: f32,
        width: f32,
        height: f32,
    ) -> image::DynamicImage {
        let image_width = image.width() as f32;
        let image_height = image.height() as f32;

        let x_center = sigmoid(x_center_raw) * image_width;
        let y_center = sigmoid(y_center_raw) * image_height;

        println!(
            "x_center: {}, y_center: {}, width: {}, height: {}",
            x_center, y_center, width, height
        );

        let x = (x_center - width / 2.0).max(0.0) as u32;
        let y = y_center as u32;

        println!("Crop coordinates: x: {}, y: {}", x, y);

        let width = width as u32;
        let height = height as u32;

        // Crop the image
        image.crop_imm(x, y, width, height)
    }
}
