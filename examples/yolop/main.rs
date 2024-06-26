use usls::{models::YOLOPv2, Annotator, DataLoader, Options};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // build model
    let options = Options::default()
        .with_model("yolopv2-dyn-480x800.onnx")?
        .with_i00((1, 1, 8).into())
        .with_confs(&[0.3]);
    let mut model = YOLOPv2::new(options)?;

    // load image
    let x = vec![DataLoader::try_read("./assets/car.jpg")?];

    // run
    let y = model.run(&x)?;

    // annotate
    let annotator = Annotator::default()
        .with_polygons_name(true)
        .with_saveout("YOLOPv2");
    annotator.annotate(&x, &y);

    Ok(())
}
