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
		Root:          "./templates/",
		Layouts:       "layouts/",
		DefaultLayout: "default",
	})

	e.Renderer = rendering.NewMetaRenderer().
		Register([]string{".mustache", ".html"}, mustache).
		Fallback(mustache)

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
