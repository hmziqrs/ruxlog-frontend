
FROM oven/bun:1.1.31-alpine AS builder

WORKDIR /app

COPY . .

RUN bun install

RUN bun run build

EXPOSE 3000

CMD ["bun", "run", "start"]