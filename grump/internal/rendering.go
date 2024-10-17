package rendering

import (
	"io"
	"path/filepath"

	"github.com/cbroglie/mustache"
	"github.com/labstack/echo"
)

type MustacheRendererConfig struct {
	caching           bool
	root_directory    string
	layouts_directory string
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
	r.load_layouts()
	r.load_partials()
	return r
}

func (r *MustacheRenderer) load_layouts()  {}
func (r *MustacheRenderer) load_partials() {}

func (r *MustacheRenderer) Get(name string) (*mustache.Template, error) {
	// Return from cache if available
	t, ok := r.cache[name]
	if ok {
		return t, nil
	}

	dir, filename := filepath.Split(name)

	var err error

	for _, ext := range []string{"", ".mustache", ".stache", ".html"} {
		t, err := mustache.ParseFile(filepath.Join(r.config.root_directory, dir, filename+ext))
		if err == nil {
			if r.config.caching {
				r.cache[name] = t
			}
			return t, nil
		}
	}

	return nil, err

}

func (mr *MustacheRenderer) Render(w io.Writer, name string, data interface{}, ctx echo.Context) error {
	template, err := mr.Get(name)
	if err != nil {
		return err
	}
	err = template.FRender(w, data)
	return err
}
