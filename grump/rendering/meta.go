package rendering

import (
	"fmt"
	"io"
	"path/filepath"

	"github.com/labstack/echo/v4"
)

type MetaRenderer struct {
	dispatch map[string]echo.Renderer
	fallback echo.Renderer
}

func NewMetaRenderer() *MetaRenderer {
	return &MetaRenderer{
		dispatch: make(map[string]echo.Renderer),
		fallback: nil,
	}
}

func (m *MetaRenderer) Register(extensions []string, r echo.Renderer) *MetaRenderer {
	for _, ext := range extensions {
		m.dispatch[ext] = r
	}
	return m
}
func (m *MetaRenderer) Fallback(r echo.Renderer) *MetaRenderer {
	m.fallback = r
	return m
}

func (m *MetaRenderer) GetRendererFor(name string) (echo.Renderer, error) {
	ext := filepath.Ext(name)
	rc, ok := m.dispatch[ext]
	if ok {
		return rc, nil
	}
	if m.fallback != nil {
		return m.fallback, nil
	}
	return nil, fmt.Errorf("could not handle file %v as template", name)

}

func (m *MetaRenderer) Render(w io.Writer, name string, data interface{}, ctx echo.Context) error {
	ir, err := m.GetRendererFor(name)
	if err != nil {
		return err
	}

	return ir.Render(w, name, data, ctx)
}
