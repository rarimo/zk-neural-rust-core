use image::{DynamicImage, imageops::FilterType};
use tflitec::{interpreter::Interpreter, model::Model};

use crate::{
    ZKNeuralError,
    core::{
        math::sigmoid,
        tensor::{collect_processed_data_to_float, prepare_data_by_float_type},
    },
};

use super::face_anchors::BLAZE_FACE_SHORT_RANGE_ANCHORS;

const IMAGE_SCALE: u32 = 128;

const FACE_DETECTION_MIN_SCORE: f32 = 0.9;

const BLAZE_FACE_MODEL_BYTES: &[u8] = include_bytes!("../../assets/blaze_face_short_range.tflite");

pub struct FaceDetector {}

impl FaceDetector {
    pub fn detect_face(image_data: &[u8]) -> Result<DynamicImage, ZKNeuralError> {
        let loaded_rescaled_image = image::load_from_memory(image_data)?.resize_exact(
            IMAGE_SCALE,
            IMAGE_SCALE,
            FilterType::CatmullRom,
        );

        let rgb_image_data: Vec<u8> = loaded_rescaled_image.to_rgb8().to_vec();

        let (prepared_image_data, _) = prepare_data_by_float_type::<f32>(rgb_image_data);

        let model = Model::from_bytes(BLAZE_FACE_MODEL_BYTES)?;

        let interpreter = Interpreter::new(&model, None)?;

        interpreter.allocate_tensors()?;

        interpreter.copy(&prepared_image_data, 0)?;

        interpreter.invoke()?;

        let face_detections_tensor = interpreter.output(0)?;
        let face_scores_tensor = interpreter.output(1)?;

        let face_detections =
            collect_processed_data_to_float::<f32>(face_detections_tensor.data().to_vec(), false);
        let raw_face_scores =
            collect_processed_data_to_float::<f32>(face_scores_tensor.data().to_vec(), false);

        let face_scores: Vec<f32> = raw_face_scores
            .iter()
            .map(|&score| sigmoid(score))
            .collect();

        let boxes = face_detections
            .chunks(16)
            .map(|chunk| chunk.to_vec())
            .collect::<Vec<Vec<f32>>>();

        let decoded_boxes = Self::decode_boxes(boxes);

        let mut best_score_index: usize = 0;
        for (i, &score) in face_scores.iter().enumerate() {
            if score > face_scores[best_score_index] {
                best_score_index = i;
            }
        }

        if face_scores[best_score_index] < FACE_DETECTION_MIN_SCORE {
            return Err(ZKNeuralError::FaceNotFound);
        }

        Ok(Self::resize_original_image(
            image_data,
            &decoded_boxes[best_score_index],
        ))
    }

    fn resize_original_image(image_data: &[u8], best_box: &[f32]) -> DynamicImage {
        let loaded_image = image::load_from_memory(&image_data).unwrap();

        let x_min = loaded_image.width() as f32 * best_box[0];
        let y_min = loaded_image.height() as f32 * best_box[1];
        let x_max = loaded_image.width() as f32 * best_box[2];
        let y_max = loaded_image.height() as f32 * best_box[3];

        loaded_image.crop_imm(
            x_min as u32,
            y_min as u32,
            (x_max - x_min) as u32,
            (y_max - y_min) as u32,
        )
    }

    fn decode_boxes(boxes: Vec<Vec<f32>>) -> Vec<Vec<f32>> {
        let mut decoded_boxes: Vec<Vec<f32>> = vec![];
        for (element, anchor) in boxes.iter().zip(BLAZE_FACE_SHORT_RANGE_ANCHORS.iter()) {
            let x_center = element[0] / IMAGE_SCALE as f32 * anchor[2] + anchor[0];
            let y_center = element[1] / IMAGE_SCALE as f32 * anchor[3] + anchor[1];
            let width = element[2] / IMAGE_SCALE as f32 * anchor[2];
            let height = element[3] / IMAGE_SCALE as f32 * anchor[3];

            let decoded_box = vec![
                x_center - width / 2.0,
                y_center - height / 2.0,
                x_center + width / 2.0,
                y_center + height / 2.0,
            ];

            decoded_boxes.push(decoded_box);
        }

        return decoded_boxes;
    }
}

#[cfg(test)]
mod tests {
    use std::{fs::File, io::Read};

    use crate::core::face_detection::FaceDetector;

    #[test]
    fn test_face_detection() {
        let image_data = File::open("assets/face3.jpg")
            .unwrap()
            .bytes()
            .collect::<Result<Vec<_>, _>>()
            .unwrap();

        let _ = FaceDetector::detect_face(&image_data).expect("Face detection failed");
    }
}
