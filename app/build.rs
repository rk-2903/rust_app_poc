use burn_onnx::ModelGen;

fn main() {
    println!("cargo:rerun-if-changed=models/moonshine-tiny/encoder_model.onnx");
    println!("cargo:rerun-if-changed=models/moonshine-tiny/decoder_model_merged.onnx");

    ModelGen::new()
        .input("models/moonshine-tiny/encoder_model.onnx")
        .input("models/moonshine-tiny/decoder_model_merged.onnx")
        .out_dir("moonshine/")
        .run_from_script();
}
