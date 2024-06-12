package internal

import (
	"io"
	"log/slog"
	"net"
	"strings"
	"sync"

	"github.com/Microsoft/go-winio"
)

func StartListener(address, pipeName string) {
	listener, err := net.Listen("tcp", address)
	if err != nil {
		slog.Error("Error starting TCP listener", slog.Any("error", err))
		return
	}
	defer listener.Close()

	slog.Debug("TCP listener started", "localAddr", address)

	for {
		conn, err := listener.Accept()
		if err != nil {
			slog.Error("Error accepting connection", slog.Any("error", err))
			continue
		}

		slog.Debug("Accepted connection", "remoteAddr", conn.RemoteAddr())

		go handleConnection(conn, pipeName)
	}
}

func handleConnection(conn net.Conn, pipeName string) {
	defer conn.Close()

	pipe, err := winio.DialPipe(pipeName, nil)
	if err != nil {
		slog.Error("Error opening named pipe", slog.Any("error", err))
		return
	}
	defer pipe.Close()

	var once sync.Once
	var wg sync.WaitGroup
	wg.Add(1)

	go func() {
		_, err := io.Copy(pipe, conn)
		if err != nil && !strings.Contains(err.Error(), "file has already been closed") {
			slog.Error("Error copying from TCP to pipe", slog.Any("error", err))
		}
		once.Do(func() { wg.Done() })
	}()

	go func() {
		_, err := io.Copy(conn, pipe)
		if err != nil && !strings.Contains(err.Error(), "file has already been closed") {
			slog.Error("Error copying from pipe to TCP:", slog.Any("error", err))
		}
		once.Do(func() { wg.Done() })
	}()

	wg.Wait()
	slog.Debug("Finished handling connection", "remoteAddr", conn.RemoteAddr())
}
