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
