use super::face_anchors::BLAZE_FACE_SHORT_RANGE_ANCHORS;

const IMAGE_SCALE: u32 = 128;

struct FaceDetector {}

// x_center = raw_boxes[..., 0] / self.x_scale * anchors[:, 2] + anchors[:, 0]
//         y_center = raw_boxes[..., 1] / self.y_scale * anchors[:, 3] + anchors[:, 1]

//         print("scales:", self.x_scale, self.y_scale, self.w_scale, self.h_scale)

//         w = raw_boxes[..., 2] / self.w_scale * anchors[:, 2]
//         h = raw_boxes[..., 3] / self.h_scale * anchors[:, 3]

//         boxes[..., 0] = y_center - h / 2.  # ymin
//         boxes[..., 1] = x_center - w / 2.  # xmin
//         boxes[..., 2] = y_center + h / 2.  # ymax
//         boxes[..., 3] = x_center + w / 2.  # xmax

impl FaceDetector {
    fn decode_boxes(boxes: Vec<Vec<f32>>) -> Vec<Vec<f32>> {
        let mut decoded_boxes: Vec<Vec<f32>> = vec![];
        for (element, anchor) in boxes.iter().zip(BLAZE_FACE_SHORT_RANGE_ANCHORS.iter()) {
            let decoded_box = vec![
                element[0] / IMAGE_SCALE as f32 * anchor[2] + anchor[0],
                element[1] / IMAGE_SCALE as f32 * anchor[3] + anchor[1],
                element[2] / IMAGE_SCALE as f32 * anchor[2],
                element[3] / IMAGE_SCALE as f32 * anchor[3],
            ];

            decoded_boxes.push(decoded_box);
        }

        return vec![];
    }
}
