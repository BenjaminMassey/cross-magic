@echo off
set URL=https://huggingface.co/unsloth/Qwen3-14B-GGUF/resolve/main/Qwen3-14B-UD-IQ2_M.gguf?download=true
set SCRIPT_DIR=%~dp0
curl -L %URL% -o "%SCRIPT_DIR%Qwen3-14B-UD-IQ2_M.gguf"
echo Download complete: %SCRIPT_DIR%Qwen3-14B-UD-IQ2_M.gguf
pause