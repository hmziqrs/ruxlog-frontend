@REM deploy-prod.bat
@echo off
setlocal enabledelayedexpansion

:: Store the original directory
set "ORIGINAL_DIR=%CD%"
set "BASE_DIR=%CD%"
set SERVICES=client admin
set PROJECT=rux_prod

echo Starting production deployment process...

:: Create network if it doesn't exist
docker network create %PROJECT%_network 2>nul || echo Network already exists

for %%s in (%SERVICES%) do (
    echo.
    echo Processing %%s...

    if exist "%BASE_DIR%\%%s" (
        cd "%BASE_DIR%\%%s"

        :: Down the container
        echo Stopping %%s containers...
        docker compose -f docker-compose.prod.yml down

        :: Rebuild
        echo Rebuilding %%s...
        docker compose -f docker-compose.prod.yml build

        :: Start
        echo Starting %%s...
        set "PROJECT=%PROJECT%" && docker compose -f docker-compose.prod.yml up -d

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
