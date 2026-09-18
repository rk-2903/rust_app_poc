#!/bin/sh
# Downloads the Moonshine Tiny ONNX weights (encoder + decoder) needed to
# build the transcription module. Not committed to git (~109MB) — run this
# once after cloning, from anywhere:
#
#   ./scripts/download_models.sh
#
# The float (fp32) export is used deliberately, not the smaller quantized
# one: the quantized export uses DynamicQuantizeLinear/QLinearConv, which
# burn-onnx doesn't support yet (see SUPPORTED-ONNX-OPS.md in tracel-ai/
# burn-onnx). Revisit once/if that lands.
set -eu

cd "$(dirname "$0")/.."
mkdir -p models/moonshine-tiny
cd models/moonshine-tiny

BASE_URL="https://huggingface.co/moonshine-ai/moonshine/resolve/main/onnx/merged/tiny/float"

echo "Downloading Moonshine Tiny encoder (~31MB)..."
curl -sL -o encoder_model.onnx "$BASE_URL/encoder_model.onnx"

echo "Downloading Moonshine Tiny decoder (~78MB)..."
curl -sL -o decoder_model_merged.onnx "$BASE_URL/decoder_model_merged.onnx"

echo "Done: $(pwd)"
ls -la
