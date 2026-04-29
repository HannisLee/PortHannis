package tray

import (
	_ "embed"

	"fyne.io/systray"
)

//go:embed icon.png
var iconData []byte

type Callbacks struct {
	OnShow func()
	OnHide func()
	OnQuit func()
}

func Run(cb Callbacks) {
	systray.Run(func() {
		systray.SetTitle("PH")
		systray.SetTooltip("PortHannis - 端口转发工具")
		systray.SetIcon(iconData)

		mShow := systray.AddMenuItem("显示窗口", "显示主窗口")
		systray.AddSeparator()
		mQuit := systray.AddMenuItem("退出", "退出 PortHannis")

		go func() {
			for {
				select {
				case <-mShow.ClickedCh:
					if cb.OnShow != nil {
						cb.OnShow()
					}
				case <-mQuit.ClickedCh:
					if cb.OnQuit != nil {
						cb.OnQuit()
					}
					systray.Quit()
					return
				}
			}
		}()
	}, func() {
		if cb.OnQuit != nil {
			cb.OnQuit()
		}
	})
}

func Quit() {
	systray.Quit()
}
