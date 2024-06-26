use usls::{
    models::{YOLOVersion, YOLO},
    Annotator, DataLoader, Options,
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // build model
    let options = Options::default()
        .with_model("yolov10n-dyn.onnx")?
        .with_yolo_version(YOLOVersion::V10)
        .with_i00((1, 1, 4).into())
        .with_i02((416, 640, 800).into())
        .with_i03((416, 640, 800).into())
        .with_confs(&[0.4, 0.15]);
    let mut model = YOLO::new(options)?;

    // load image
    let x = vec![DataLoader::try_read("./assets/bus.jpg")?];

    // run
    let y = model.run(&x)?;

    // annotate
    let annotator = Annotator::default().with_saveout("YOLOv10");
    annotator.annotate(&x, &y);

    Ok(())
}
