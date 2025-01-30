package main

import (
	"net/http"
	"os"
	"time"

	"github.com/bricef/grump/rendering"
	"github.com/rs/zerolog"

	"github.com/labstack/echo/v4"
	"github.com/labstack/echo/v4/middleware"
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

func main() {

	e := echo.New()

	logger := zerolog.New(os.Stdout)
	e.Use(middleware.Recover())
	e.Use(middleware.RequestLoggerWithConfig(middleware.RequestLoggerConfig{
		LogURI:    true,
		LogStatus: true,
		LogValuesFunc: func(c echo.Context, v middleware.RequestLoggerValues) error {
			logger.Info().
				Str("URI", v.URI).
				Int("status", v.Status).
				Msg("request")

			return nil
		},
	}))

	mustache := rendering.NewMustacheRenderer(rendering.MustacheRendererConfig{
		Caching:       false,
		Layouts:       "templates/layouts/",
		DefaultLayout: "default",
	})
	markdown := rendering.NewMarkdownRenderer()
	mdx := rendering.NewMdxRenderer()

	e.Renderer = rendering.NewMetaRenderer().
		Root("./templates").
		Register([]string{".mustache", ".html"}, mustache).
		Register([]string{".md"}, markdown).
		Register([]string{".mdx"}, mdx)
		// Fallback(mustache)

	e.HTTPErrorHandler = func(err error, ctx echo.Context) {
		code := http.StatusInternalServerError
		if he, ok := err.(*echo.HTTPError); ok {
			code = he.Code
			ctx.Render(code, "pages/error.html", he)
		} else {
			e.Logger.Error(err)
			ctx.Render(code, "pages/error.html", ErrorInfo{Code: http.StatusInternalServerError, Message: "Server encountered an internal error"})
		}
	}

	e.GET("/", func(c echo.Context) error {
		return c.Render(http.StatusOK, "pages/index.md", interface{}(nil))
	})
	e.GET("/mustache", func(c echo.Context) error {
		data := map[string]string{"c": "from mustache"}
		return c.Render(http.StatusOK, "pages/mustache.mustache", data)
	})
	e.GET("/simple", func(c echo.Context) error {
		return c.Render(http.StatusOK, "pages/simple.mustache", nil)
	})
	e.GET("/date", func(c echo.Context) error {
		return c.Render(http.StatusOK, "partials/date.mustache", &DateInfo{time.Now()})
	})
	e.GET("/md", func(c echo.Context) error {
		return c.Render(http.StatusOK, "simple.md", nil)
	})
	e.GET("/mdx", func(c echo.Context) error {
		return c.Render(http.StatusOK, "simple.mdx", nil)
	})

	e.Logger.Fatal(e.Start(":1323"))
}
