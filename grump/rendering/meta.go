package rendering

import (
	"bytes"
	"fmt"
	"io"
	"path/filepath"

	"github.com/labstack/echo/v4"
)

type GrumpRenderer interface {
	Render(w io.Writer, name string, data interface{}, ctx echo.Context) error
	Transform(string) (string, error)
}

type MetaRenderer struct {
	root     string
	dispatch map[string]echo.Renderer
	fallback echo.Renderer
}

func NewMetaRenderer() *MetaRenderer {
	return &MetaRenderer{
		root:     "./",
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

func (m *MetaRenderer) Root(path string) *MetaRenderer {
	m.root = path
	return m
}

func (m *MetaRenderer) GetRendererFor(name string) (echo.Renderer, error) {
	ext := filepath.Ext(name)
	if ext == "" {
		// fmt.Printf("File does not have an extension\n")
		return nil, echo.NewHTTPError(500, fmt.Sprintf("file '%v' did not have an extension ", name))
	}

	rc, ok := m.dispatch[ext]
	if ok {
		fmt.Printf("Found Renderer for ext %v\n", ext)
		return rc, nil
	}

	fmt.Printf("No Renderer found for ext %v\n", ext)
	if m.fallback != nil {
		return m.fallback, nil
	}
	return nil, fmt.Errorf("could not handle file %v as template", name)

}

func (m *MetaRenderer) Render(w io.Writer, name string, data interface{}, ctx echo.Context) error {
	fullname := filepath.Join(m.root, name)
	ir, err := m.GetRendererFor(name)
	if err != nil {
		return err
	}

	return ir.Render(w, fullname, data, ctx)
}

type WrappedRenderer struct {
	outerTemplate string
	inner         echo.Renderer
	outer         echo.Renderer
}

func (m *WrappedRenderer) Render(w io.Writer, name string, data interface{}, ctx echo.Context) error {
	// Create a string writer
	wi := bytes.NewBufferString("")
	m.inner.Render(wi, name, data, ctx)

	wo := bytes.NewBufferString("")
	m.outer.Render(wo, m.outerTemplate, struct{ Content string }{
		Content: wi.String(),
	}, ctx)

	w.Write(wo.Bytes())
	return nil
}

func Wrap(inner echo.Renderer, outer echo.Renderer, outerTemplate string) echo.Renderer {
	return &WrappedRenderer{
		outerTemplate: outerTemplate,
		inner:         inner,
		outer:         outer,
	}
}
