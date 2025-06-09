#[cfg(test)]
mod tests {
    use std::{fs::File, io::Read};
    use tflitec::{interpreter::Interpreter, model::Model};

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
}
