package handlers_test

import (
	"net/http"
	"net/http/httptest"
	"testing"

	"github.com/trilliondefense/defendrite/backend/handlers" // Replace "backend" with your module name

	"github.com/gin-gonic/gin"
)

func TestHealth(t *testing.T) {
	gin.SetMode(gin.TestMode)

	r := gin.New()

	api := r.Group("/api")
	{
		api.GET("/health", handlers.Health)
	}

	req, err := http.NewRequest(http.MethodGet, "/api/health", nil)
	if err != nil {
		t.Fatalf("failed to create request: %v", err)
	}

	w := httptest.NewRecorder()

	r.ServeHTTP(w, req)

	if w.Code != http.StatusOK {
		t.Fatalf("expected %d, got %d", http.StatusOK, w.Code)
	}

	expected := "OK"

	if w.Body.String() != expected {
		t.Fatalf("expected body %q, got %q", expected, w.Body.String())
	}
}