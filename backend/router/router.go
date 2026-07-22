package router

import (
	"os"

	"github.com/gin-gonic/gin"

	"github.com/trilliondefense/defendrite/backend/handlers"
)

func Setup() *gin.Engine {

	r := gin.Default()

	api := r.Group("/api")
	{
		api.GET("/health", handlers.Health)

		api.GET("/config", handlers.GetConfig)

		api.POST("/mcp-server", handlers.UpdateMCP)

		api.POST("/ai-agent", handlers.UpdateAIAgent)
	}

	r.NoRoute(func(c *gin.Context) {

		path := "./frontend/build" + c.Request.URL.Path

		if _, err := os.Stat(path); os.IsNotExist(err) {
			c.File("./frontend/build/index.html")
			return
		}

		c.File(path)
	})

	return r
}
