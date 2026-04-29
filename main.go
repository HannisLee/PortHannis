package main

import (
	"context"
	"embed"

	"github.com/HannisLee/PortHannis/tray"
	"github.com/wailsapp/wails/v2"
	"github.com/wailsapp/wails/v2/pkg/options"
	"github.com/wailsapp/wails/v2/pkg/options/assetserver"
	"github.com/wailsapp/wails/v2/pkg/runtime"
)

//go:embed all:frontend/dist
var assets embed.FS

func main() {
	app := NewApp()

	go tray.Run(tray.Callbacks{
		OnShow: func() {
			if app.ctx != nil {
				runtime.WindowShow(app.ctx)
			}
		},
		OnQuit: func() {
			if app.ctx != nil {
				app.quitting = true
				runtime.Quit(app.ctx)
			}
		},
	})

	err := wails.Run(&options.App{
		Title:  "PortHannis",
		Width:  800,
		Height: 600,
		AssetServer: &assetserver.Options{
			Assets: assets,
		},
		BackgroundColour: &options.RGBA{R: 27, G: 38, B: 54, A: 1},
		OnStartup:  app.startup,
		OnShutdown: app.shutdown,
		OnBeforeClose: func(ctx context.Context) (prevent bool) {
			if app.quitting {
				return false
			}
			runtime.WindowHide(app.ctx)
			return true
		},
		Bind: []interface{}{
			app,
		},
	})

	tray.Quit()

	if err != nil {
		println("Error:", err.Error())
	}
}
