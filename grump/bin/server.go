package main

import (
	"net/http"

	"github.com/a-h/templ"
	"github.com/bricef/grump/templates"
	"github.com/labstack/echo/v4"
)

type LoginPage struct {
	Title    string // {{ .Title }}
	Username string // {{ .Username }}
	Password string // {{ .Password }}
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

	r := templates.ErrorPage(code)
	render(ctx, code, r)
}

func main() {
	e := echo.New()
	e.GET("/", func(c echo.Context) error {
		return c.String(http.StatusOK, "Hello, World!")
	})

	e.HTTPErrorHandler = customHTTPErrorHandler
	e.Logger.Fatal(e.Start(":1323"))
}
