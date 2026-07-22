package models

type Endpoint struct {
	URL    string `yaml:"url" json:"url"`
	APIKey string `yaml:"api_key" json:"api_key"`
}

type Config struct {
	MCPServer Endpoint `yaml:"mcp_server" json:"mcp_server"`
	AIAgent   Endpoint `yaml:"ai_agent" json:"ai_agent"`
}
