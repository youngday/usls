mod annotator;
mod bbox;
mod dataloader;
mod device;
mod dynconf;
mod embedding;
mod engine;
mod keypoint;
mod logits_sampler;
mod metric;
mod min_opt_max;
pub mod models;
pub mod ops;
mod options;
mod point;
mod polygon;
mod rect;
mod rotated_rect;
mod tokenizer_stream;
mod utils;
mod ys;

pub use annotator::Annotator;
pub use bbox::Bbox;
pub use dataloader::DataLoader;
pub use device::Device;
pub use dynconf::DynConf;
pub use embedding::Embedding;
pub use engine::OrtEngine;
pub use keypoint::Keypoint;
pub use logits_sampler::LogitsSampler;
pub use metric::Metric;
pub use min_opt_max::MinOptMax;
pub use options::Options;
pub use point::Point;
pub use polygon::Polygon;
pub use rect::Rect;
pub use rotated_rect::RotatedRect;
pub use tokenizer_stream::TokenizerStream;
pub use utils::{auto_load, config_dir, download, string_now, COCO_NAMES_80, COCO_SKELETON_17};
pub use ys::Ys;

const GITHUB_ASSETS: &str = "https://github.com/jamjamjon/assets/releases/download/v0.0.1";
const CHECK_MARK: &str = "✅";
const CROSS_MARK: &str = "❌";
const SAFE_CROSS_MARK: &str = "❎";
