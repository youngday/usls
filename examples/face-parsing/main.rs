use usls::{models::YOLO, Annotator, DataLoader, Options};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // build model
    let options = Options::default()
        .with_model("../models/face-parsing-dyn.onnx")
        .with_i00((1, 1, 4).into())
        .with_i02((416, 640, 800).into())
        .with_i03((416, 640, 800).into())
        // .with_trt(0)
        // .with_fp16(true)
        // .with_dry_run(10)
        .with_confs(&[0.5]);
    let mut model = YOLO::new(&options)?;

    // load image
    let x = vec![DataLoader::try_read("./assets/nini.png")?];

    // run
    let y = model.run(&x)?;

    // annotate
    let annotator = Annotator::default()
        .without_conf(true)
        .without_name(true)
        .without_polygons(false)
        .without_bboxes(true)
        .with_masks_name(false)
        .with_saveout("Face-Parsing");
    annotator.annotate(&x, &y);

    Ok(())
}