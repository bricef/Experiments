package main

import (
	"github.com/bricef/Experiments/web-chat/chatroom"

	"github.com/gorilla/sessions"
	"github.com/gorilla/websocket"
	"github.com/labstack/echo-contrib/session"
	"github.com/labstack/echo/v4"
	"github.com/labstack/echo/v4/middleware"
)

var (
	upgrader = websocket.Upgrader{}
)

func main() {
	dispatcher := chatroom.NewDispatcher()

	e := echo.New()
	e.Use(middleware.Logger())
	e.Use(middleware.Recover())
	e.Use(session.Middleware(sessions.NewCookieStore([]byte("lknosauhfuhdsaa7aayfsdad09w7fshdj"))))

	e.Static("/", "./public")

	e.GET("/chatroom", func(c echo.Context) error {
		// upgrade the connection to a websocket
		ws, err := upgrader.Upgrade(c.Response(), c.Request(), nil)
		if err != nil {
			return err
		}
		// add the client to the dispatcher
		return dispatcher.AddClient(chatroom.NewClient(ws))
	})

	// go func() {
	// 	for {
	// 		time.Sleep(3 * time.Second)
	// 		dispatcher.sendAll(Message{
	// 			Content: time.Now().Format(time.RFC3339),
	// 			Nick:    "Server",
	// 		})
	// 	}
	// }()

	e.Logger.Fatal(e.Start(":1323"))
}
