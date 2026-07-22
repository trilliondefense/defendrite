.PHONY: install backend frontend dev build test clean

install:
	cd backend && go mod tidy
	cd frontend && npm install

backend:
	cd backend && go run ./cmd/server

frontend:
	cd frontend && npm run dev

dev:
	@echo "Start the backend in one terminal:"
	@echo "  make backend"
	@echo ""
	@echo "Start the frontend in another terminal:"
	@echo "  make frontend"

build:
	cd backend && go build -o defendrite ./cmd/server
	cd frontend && npm run build

test:
	cd backend && go test ./...
	cd frontend && npm run check

clean:
	rm -f backend/defendrite
	rm -rf frontend/build
	rm -rf frontend/.svelte-kit