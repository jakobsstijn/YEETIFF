@echo off
echo ============================================================
echo YEET Installer - Quick Build
echo ============================================================
echo.

REM Check Python
python --version >nul 2>&1
if errorlevel 1 (
    echo ERROR: Python is not installed or not in PATH
    pause
    exit /b 1
)

echo [1/2] Installing dependencies...
python -m pip install -r requirements.txt

echo.
echo [2/2] Building installer EXE...
python build_installer.py

echo.
pause
