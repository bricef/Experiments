package rendering

import (
	"io"

	"github.com/labstack/echo/v4"
	"github.com/zbysir/gojsx"
)

type MdxRenderer struct{}

func NewMdxRenderer() *MdxRenderer {
	return &MdxRenderer{}
}

func (r *MdxRenderer) Render(w io.Writer, name string, data interface{}, ctx echo.Context) error {

	mdr, err := gojsx.NewJsx(gojsx.Option{Debug: true})
	if err != nil {
		return err
	}
	htmls, err := mdr.Render(name, data)
	if err != nil {
		return err
	}
	_, err = w.Write([]byte(htmls))
	if err != nil {
		return err
	}
	return nil
}
