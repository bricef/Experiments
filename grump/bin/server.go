package main

import (
	"io"
	"net/http"

	"github.com/a-h/templ"
	"github.com/bricef/grump/templates"
	"github.com/cbroglie/mustache"
	"github.com/labstack/echo/v4"
)

type LoginPage struct {
	Title    string // {{ .Title }}
	Username string // {{ .Username }}
	Password string // {{ .Password }}
}

type MustacheRenderer map[string]string

func (mr MustacheRenderer) Render(w io.Writer, name string, data interface{}, ctx echo.Context) error {
	template, err := mustache.Render(mr[name], data)
	if err != nil {
		return err
	}
	_, err = w.Write([]byte(template))
	return err
}

func render(ctx echo.Context, status int, t templ.Component) error {
	ctx.Response().Writer.WriteHeader(status)

	err := t.Render(ctx.Request().Context(), ctx.Response().Writer)
	if err != nil {
		return ctx.String(http.StatusInternalServerError, "failed to render response template")
	}

	return nil
}

func customHTTPErrorHandler(err error, ctx echo.Context) {
	code := http.StatusInternalServerError
	if he, ok := err.(*echo.HTTPError); ok {
		code = he.Code
	}
	render(ctx, code, templates.ErrorPage(code))
}

func main() {

	e := echo.New()
	e.Renderer = MustacheRenderer{
		"a": "I am template A {{data}}",
		"b": "I am template B {{data}}",
	}

	e.GET("/", func(c echo.Context) error {
		return c.String(http.StatusOK, "Hello, World!")
	})
	e.GET("/mustache", func(c echo.Context) error {
		data, err := mustache.Render("hello {{c}}", map[string]string{"c": "from mustache"})
		if err != nil {
			return echo.NewHTTPError(http.StatusInternalServerError, "failed to render template")
		}
		return c.String(http.StatusOK, data)
	})
	e.GET("/a", func(c echo.Context) error {
		return c.Render(http.StatusOK, "a", map[string]string{"data": "from a"})
	})
	e.GET("/b", func(c echo.Context) error {
		return c.Render(http.StatusOK, "b", map[string]string{"data": "from b"})
	})

	e.HTTPErrorHandler = customHTTPErrorHandler
	e.Logger.Fatal(e.Start(":1323"))
}
