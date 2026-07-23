# ===================================
# Stage 1: Build frontend
# ===================================
FROM node:22-alpine AS frontend-builder

WORKDIR /app/frontend

RUN corepack enable

COPY frontend/package*.json frontend/pnpm-lock.yaml ./

RUN pnpm install

COPY frontend/ .

RUN pnpm build


# ===================================
# Stage 2: Build backend
# ===================================
FROM golang:1.26-alpine AS backend-builder

WORKDIR /app/backend

COPY backend/go.mod backend/go.sum ./

RUN go mod download

COPY backend/ .

# Copy Svelte build into Go project
COPY --from=frontend-builder /app/frontend/build ./frontend/build

RUN go build -o defendrite ./cmd/server


# ===================================
# Stage 3: Final image
# ===================================
FROM alpine:latest

WORKDIR /app

COPY --from=frontend-builder /app/frontend/build ./frontend/build
COPY --from=backend-builder /app/backend/defendrite .

EXPOSE 8080

CMD ["./defendrite"]