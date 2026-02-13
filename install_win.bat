@echo off
set "HOST_NAME=com.focussentinel"
set "TARGET_DIR=%~dp0host\target\release"
set "MANIFEST_PATH=%~dp0host\com.focussentinel.json"
set "EXE_PATH=%TARGET_DIR%\focus_sentinel_host.exe"

:: Escape backslashes for JSON
set "JSON_EXE_PATH=%EXE_PATH:\=\\%"

echo Generatig Manifest...
(
echo {
echo   "name": "%HOST_NAME%",
echo   "description": "FocusSentinel Native Host",
echo   "path": "%JSON_EXE_PATH%",
echo   "type": "stdio",
echo   "allowed_origins": [
echo     "chrome-extension://jbclhaqbgpcmilmjjpldpcknneegmfgk/"
echo   ]
echo }
) > "%MANIFEST_PATH%"

echo Registering Native Host...
REG ADD "HKCU\Software\Google\Chrome\NativeMessagingHosts\%HOST_NAME%" /ve /t REG_SZ /d "%MANIFEST_PATH%" /f

echo Done! Make sure to update the 'allowed_origins' in 'host/com.focussentinel.json' with your actual Extension ID after loading it in Chrome.
pause
