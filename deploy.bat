@echo off
setlocal enabledelayedexpansion

:: Store the original directory
set "ORIGINAL_DIR=%CD%"
set "BASE_DIR=%CD%"
set SERVICES=client admin

echo Starting deployment process...

for %%s in (%SERVICES%) do (
    echo.
    echo Processing %%s...

    if exist "%BASE_DIR%\%%s" (
        cd "%BASE_DIR%\%%s"

        :: Down the container
        echo Stopping %%s containers...
        docker-compose down

        :: Rebuild
        echo Rebuilding %%s...
        docker-compose build

        :: Start
        echo Starting %%s...
        docker-compose up -d

        :: Return to original directory
        cd "%ORIGINAL_DIR%"
    ) else (
        echo Directory not found: %BASE_DIR%\%%s
    )
)

echo.
echo All services rebuilt and running
docker ps

pause
