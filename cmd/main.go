package main

import (
	"log/slog"
	"os"

	"github.com/brain-hol/tcp2npipe/internal"
)

func main() {
	logger := slog.New(slog.NewJSONHandler(os.Stdout, &slog.HandlerOptions{
		Level: slog.LevelDebug,
	}))
	slog.SetDefault(logger)

	address := "172.22.128.1:8080"           // Example TCP address, change as needed
	pipeName := `\\.\pipe\openssh-ssh-agent` // Example named pipe name, change as needed

	internal.StartListener(address, pipeName)
}
