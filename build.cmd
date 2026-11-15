@echo off
setlocal
cd /d "%~dp0"
cargo build --release
if errorlevel 1 exit /b 1
copy /y "target\release\car-flipper-mod-tool.exe" "Car Flipper Save Editor.exe" >nul
if errorlevel 1 (
  echo Failed to replace "Car Flipper Save Editor.exe". Close it and run build.cmd again.
  exit /b 1
)
copy /y "target\release\cf-save.exe" "cf-save.exe" >nul
if errorlevel 1 (
  echo Failed to replace "cf-save.exe". Close it and run build.cmd again.
  exit /b 1
)
echo.
echo Built:
echo   %CD%\Car Flipper Save Editor.exe   (GUI)
echo   %CD%\cf-save.exe                   (CLI)
echo.
