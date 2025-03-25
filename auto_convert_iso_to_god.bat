@echo off
setlocal EnableDelayedExpansion

:: Use current directory as base path
set "BASE_PATH=%CD%"



:: Or (optionally) set the base path as a variable
:: set "BASE_PATH=D:\path_to_folder_with_iso_s"



:: Iterate through all .iso files in the current directory
set "ISO_COUNT=0"
for %%F in (*.iso) do set /a "ISO_COUNT+=1"

:: Initialize progress counter
set "PROCESSED_COUNT=0"

:: Iterate through all .iso files in the current directory
for %%F in (*.iso) do (
    set /a "PROCESSED_COUNT+=1"
    
    echo Processing %%F... ^(!PROCESSED_COUNT!/%ISO_COUNT%^)
    
    :: Run dry-run and capture output to temp file named after the ISO
    iso2god-x86_64-windows.exe --dry-run --trim "%%F" "%BASE_PATH%" > "%%~nF_temp.txt"
    
    :: Parse temp file for Name and Type explicitly
    for /f "tokens=1,* delims=:" %%a in ('findstr "Name:" "%%~nF_temp.txt"') do set "NAME=%%b"
    for /f "tokens=1,* delims=:" %%a in ('findstr "Type:" "%%~nF_temp.txt"') do set "TYPE=%%b"
    
    :: Trim leading spaces from variables
    set "NAME=!NAME:~1!"
    set "TYPE=!TYPE:~1!"
    
    :: If NAME is "(unknown)", use the ISO filename without extension
    if "!NAME!"=="(unknown)" (
        set "NAME=%%~nF"
    )
    
    :: Debug: Print parsed values
    echo Parsed NAME: !NAME!
    echo Parsed TYPE: !TYPE!
    
    :: Create target folder path with spaces
    set "TARGET_FOLDER=%BASE_PATH%\!NAME! !TYPE!"
    
    :: Print the target folder path
    echo Target folder: "!TARGET_FOLDER!"
    
    :: Run the actual command with parsed values and spaces
    iso2god-x86_64-windows.exe --trim "%%F" "!TARGET_FOLDER!"
    
	
	
    :: (optionally) Delete temp file
    :: del "%%~nF_temp.txt"
	
	
)

echo Done. Processed !PROCESSED_COUNT! files.