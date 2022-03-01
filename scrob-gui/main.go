package main 
/*
#cgo LDFLAGS: -L../target/debug -lscrob_gui
#include "../scrob-gui.h"
*/
import "C"
import "github.com/webview/webview"

func main() {
  go func() {
	C.scrob_run()
  } ()
  debug := true
  w := webview.New(debug)
  defer w.Destroy()
  w.SetTitle("Scrob")
  w.SetSize(400, 400, webview.HintNone)
  w.Navigate("http://localhost:8000")
  w.Run()

}


