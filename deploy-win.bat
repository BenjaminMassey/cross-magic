set YEAR=%DATE:~10,4%
set MONTH=%DATE:~4,2%
set DAY=%DATE:~7,2%
set HOUR=%TIME:~0,2%
set MINUTE=%TIME:~3,2%
set SECOND=%TIME:~6,2%
set DATETIME=%YEAR%-%MONTH%-%DAY%_%HOUR%-%MINUTE%-%SECOND%
cargo build --release
set BUILDDIR=deployments\windows\build_%DATETIME%
mkdir %BUILDDIR%
xcopy "ollama-win" "%BUILDDIR%\ollama-win\" /s /e
xcopy "ollama-model" "%BUILDDIR%\ollama-model\" /s /e
copy "target\release\*.exe" "%BUILDDIR%"