package rendering

import (
	"io"
	"path/filepath"

	"github.com/cbroglie/mustache"
	"github.com/labstack/echo/v4"
)

type MustacheRendererConfig struct {
	Caching       bool
	Root          string
	Layouts       string
	DefaultLayout string
}

type MustacheRenderer struct {
	config  MustacheRendererConfig
	layouts map[string]*mustache.Template
	cache   map[string]*mustache.Template
}

func NewMustacheRenderer(c MustacheRendererConfig) *MustacheRenderer {
	r := &MustacheRenderer{
		config:  c,
		layouts: make(map[string]*mustache.Template),
		cache:   make(map[string]*mustache.Template),
	}
	return r
}

func (r *MustacheRenderer) Get(name string) (*mustache.Template, error) {
	// Return from cache if available
	t, ok := r.cache[name]
	if ok {
		return t, nil
	}

	dir, filename := filepath.Split(name)

	var err error

	for _, ext := range []string{"", ".mustache", ".stache", ".html"} {
		t, err := mustache.ParseFile(filepath.Join(r.config.Root, dir, filename+ext))

		if err == nil {
			if r.config.Caching {
				r.cache[name] = t
			}
			return t, nil
		}
	}

	return nil, err

}

func (r *MustacheRenderer) GetLayout(name string) (*mustache.Template, error) {
	// Return from cache if available
	t, ok := r.layouts[name]
	if ok {
		return t, nil
	}

	dir, filename := filepath.Split(name)

	var err error

	for _, ext := range []string{"", ".mustache", ".stache", ".html"} {
		t, err := mustache.ParseFile(filepath.Join(r.config.Root, r.config.Layouts, dir, filename+ext))

		if err == nil {
			if r.config.Caching {
				r.layouts[name] = t
			}
			return t, nil
		}
	}

	return nil, err
}

func (r *MustacheRenderer) Render(w io.Writer, name string, data interface{}, ctx echo.Context) error {

	template, err := r.Get(name)
	if err != nil {
		return err
	}

	defaultLayout, err := r.GetLayout(r.config.DefaultLayout)
	if err != nil {
		err = template.FRender(w, data)
		return err
	}

	err = template.FRenderInLayout(w, defaultLayout, data)
	return err
}
