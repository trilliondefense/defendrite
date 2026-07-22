package config

import (
	"os"

	"gopkg.in/yaml.v3"

	"github.com/trilliondefense/defendrite/backend/models"
)

var File = "config/config.yml"

func Load() (*models.Config, error) {
	cfg := &models.Config{}

	data, err := os.ReadFile(File)
	if err != nil {
		return nil, err
	}

	if err := yaml.Unmarshal(data, cfg); err != nil {
		return nil, err
	}

	return cfg, nil
}

func Save(cfg *models.Config) error {
	data, err := yaml.Marshal(cfg)
	if err != nil {
		return err
	}

	return os.WriteFile(File, data, 0644)
}
