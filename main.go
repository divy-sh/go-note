package main

import (
	"io"

	"fyne.io/fyne/v2"
	"fyne.io/fyne/v2/app"
	"fyne.io/fyne/v2/container"
	"fyne.io/fyne/v2/dialog"
	"fyne.io/fyne/v2/widget"
)

func main() {
	a := app.New()
	w := a.NewWindow("Notepad Clone")

	content := widget.NewMultiLineEntry()

	openFile := func() {
		fd := dialog.NewFileOpen(
			func(r fyne.URIReadCloser, err error) {
				if err != nil || r == nil {
					return
				}
				data, err := io.ReadAll(r)
				if err != nil {
					dialog.ShowError(err, w)
					return
				}
				content.SetText(string(data))
			}, w)
		fd.Show()
	}

	saveFile := func() {
		fd := dialog.NewFileSave(
			func(r fyne.URIWriteCloser, err error) {
				if err != nil || r == nil {
					return
				}
				_, err = r.Write([]byte(content.Text))
				if err != nil {
					dialog.ShowError(err, w)
					return
				}
				r.Close()
			}, w)
		fd.Show()
	}

	newFile := func() {
		content.SetText("")
	}

	menu := fyne.NewMainMenu(
		fyne.NewMenu("File",
			fyne.NewMenuItem("New", func() { newFile() }),
			fyne.NewMenuItem("Open...", func() { openFile() }),
			fyne.NewMenuItem("Save As...", func() { saveFile() }),
			fyne.NewMenuItem("Quit", func() { a.Quit() }),
		),
	)

	w.SetMainMenu(menu)
	w.SetContent(container.NewStack(content))
	w.Resize(fyne.NewSize(800, 600))
	w.ShowAndRun()
}
