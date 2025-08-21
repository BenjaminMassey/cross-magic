URL=https://huggingface.co/unsloth/Qwen3-14B-GGUF/resolve/main/Qwen3-14B-UD-IQ2_M.gguf?download=true
DIR="$(cd "$(dirname "$0")" && pwd)"
curl -L "$URL" -o "$DIR/Qwen3-14B-UD-IQ2_M.gguf"