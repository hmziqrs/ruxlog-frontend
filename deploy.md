# Production:
# Deployment Instructions

## Windows Production Deployment

Set environment variable and run:
```powershell
$env:PROJECT="rux_prod"
.\deploy-prod.bat
```

Or run directly:
```cmd
deploy-prod.bat
```

## Unix Production Deployment

Make the script executable:
```bash
chmod +x deploy-prod.sh
```

Run the deployment:
```bash
./deploy-prod.sh
```

Or with sudo if needed:
```bash
sudo ./deploy-prod.sh
```

## Manual Docker Compose Commands

Windows:
```powershell
$env:PROJECT="rux_prod"; docker compose -f docker-compose.prod.yml up -d
```

Unix:
```bash
export PROJECT="rux_prod" && docker compose -f docker-compose.prod.yml up -d
```



# Development:
windows:

set env
```bash
$env:PROJECT="rux_local";
```

```bash
$env:PROJECT="rux_local"; docker compose up -d
```

Unix:

```bash
export PROJECT="rux_local" && docker compose up -d
```

```bash
chmod +x rebuild-docker.sh
```

```bash
sudo ./deploy.sh
```
