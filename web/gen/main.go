package main

import (
	"fmt"
	"os"
	"path/filepath"
	"strings"

	"github.com/gofiber/template/django"
)

func main() {

	templateDir := os.Getenv("TEMPLATE_DIR")
	if templateDir == "" {
		templateDir = "./templates"
	}
	engine := django.New(templateDir, ".html")

	err := filepath.Walk(templateDir,
		func(path string, info os.FileInfo, err error) error {
			if strings.HasSuffix(path, ".stage.html") || strings.HasSuffix(path, ".stage.tmpl") {
				base := strings.TrimSuffix(path, ".stage.html")
				base = strings.TrimSuffix(base, ".stage.tmpl")
				base = strings.TrimPrefix(base, fmt.Sprintf("%s/", templateDir))

				distPath := filepath.Join("dist", fmt.Sprintf("%s.html", base))
				os.MkdirAll(filepath.Dir(distPath), 0o755)
				os.Remove(distPath)
				f, err := os.OpenFile(distPath, os.O_RDWR|os.O_CREATE, 0644)
				if err != nil {
					panic(err)
				}
				defer f.Close()
				err = engine.Render(f, fmt.Sprintf("%s.stage", base), map[string]interface{}{})
				if err != nil {
					panic(err)
				}
				fmt.Println(path)
			}

			return nil
		})
	if err != nil {
		panic(err)
	}

}
