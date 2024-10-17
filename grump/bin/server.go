package main

import (
	"net/http"
	"time"

	"github.com/bricef/grump/internal/rendering"

	"github.com/labstack/echo/v4"
	"github.com/labstack/gommon/log"
)

type LoginPage struct {
	Title    string // {{ .Title }}
	Username string // {{ .Username }}
	Password string // {{ .Password }}
}

type ErrorInfo struct {
	Code    int
	Message string
}

type DateInfo struct {
	Date time.Time
}

var CACHE_TEMPLATE = false

func main() {

	e := echo.New()

	e.Logger.SetLevel(log.DEBUG)

	e.Renderer = rendering.NewMustacheRenderer(rendering.MustacheRendererConfig{
		caching:           false,
		root_directory:    "./templates/",
		layouts_directory: "layouts/",
	})

	e.HTTPErrorHandler = func(err error, ctx echo.Context) {
		code := http.StatusInternalServerError

		if he, ok := err.(*echo.HTTPError); ok {
			code = he.Code
			ctx.Render(code, "pages/error", he)
		} else {
			ctx.Render(code, "pages/error", ErrorInfo{Code: http.StatusInternalServerError, Message: "Server encountered an internal error"})
		}
	}

	e.GET("/", func(c echo.Context) error {
		return c.String(http.StatusOK, "Hello, World!")
	})
	e.GET("/mustache", func(c echo.Context) error {
		data := map[string]string{"c": "from mustache"}
		return c.Render(http.StatusOK, "pages/mustache", data)
	})
	e.GET("/simple", func(c echo.Context) error {
		return c.Render(http.StatusOK, "pages/simple", nil)
	})
	e.GET("/date", func(c echo.Context) error {
		return c.Render(http.StatusOK, "partials/date", &DateInfo{time.Now()})
	})

	e.Logger.Fatal(e.Start(":1323"))
}
