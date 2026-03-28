package main

import (
	"embed"
	"fmt"
	"html/template"
	"log"
	"net/http"
	"sync/atomic"
	"time"
)

//go:embed templates
var templateFS embed.FS

var (
	startTime = time.Now()
	views     atomic.Int64
)

type pageData struct {
	ViewCount int64
	Duration  string
}

func formatDuration(d time.Duration) string {
	switch {
	case d < time.Hour:
		mins := int(d.Minutes())
		if mins < 1 {
			mins = 1
		}
		return fmt.Sprintf("%d minute%s", mins, pluralize(mins))
	case d < 24*time.Hour:
		hours := int(d.Hours())
		return fmt.Sprintf("%d hour%s", hours, pluralize(hours))
	default:
		days := int(d.Hours() / 24)
		return fmt.Sprintf("%d day%s", days, pluralize(days))
	}
}

func pluralize(n int) string {
	if n == 1 {
		return ""
	}
	return "s"
}

func main() {
	tmpl := template.Must(template.ParseFS(templateFS, "templates/index.html"))

	http.HandleFunc("/", func(w http.ResponseWriter, r *http.Request) {
		// counter resets on restart; persistence to be added later
		count := views.Add(1)
		elapsed := time.Since(startTime)
		data := pageData{
			ViewCount: count,
			Duration:  formatDuration(elapsed),
		}
		if err := tmpl.Execute(w, data); err != nil {
			http.Error(w, "template error", http.StatusInternalServerError)
		}
	})

	log.Println("listening on :8080")
	log.Fatal(http.ListenAndServe(":8080", nil))
}
