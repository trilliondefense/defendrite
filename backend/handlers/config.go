package handlers

import (
	"net/http"

	"github.com/gin-gonic/gin"

	"github.com/trilliondefense/defendrite/backend/config"
)

type Request struct {
	URL    string `json:"url" binding:"required,url"`
	APIKey string `json:"api_key" binding:"required"`
}

func GetConfig(c *gin.Context) {

	cfg, err := config.Load()
	if err != nil {
		c.JSON(http.StatusInternalServerError, gin.H{
			"error": err.Error(),
		})
		return
	}

	c.JSON(http.StatusOK, cfg)
}

func UpdateMCP(c *gin.Context) {

	var req Request

	if err := c.ShouldBindJSON(&req); err != nil {
		c.JSON(http.StatusBadRequest, gin.H{"error": err.Error()})
		return
	}

	cfg, _ := config.Load()

	cfg.MCPServer.URL = req.URL
	cfg.MCPServer.APIKey = req.APIKey

	config.Save(cfg)

	c.JSON(http.StatusOK, gin.H{
		"message": "updated",
	})
}

func UpdateAIAgent(c *gin.Context) {

	var req Request

	if err := c.ShouldBindJSON(&req); err != nil {
		c.JSON(http.StatusBadRequest, gin.H{
			"error": err.Error(),
		})
		return
	}

	cfg, _ := config.Load()

	cfg.AIAgent.URL = req.URL
	cfg.AIAgent.APIKey = req.APIKey

	config.Save(cfg)

	c.JSON(http.StatusOK, gin.H{
		"message": "updated",
	})
}
