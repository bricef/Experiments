package main

import (
	"fmt"

	"github.com/gorilla/sessions"
	"github.com/gorilla/websocket"
	"github.com/labstack/echo-contrib/session"
	"github.com/labstack/echo/v4"
	"github.com/labstack/echo/v4/middleware"
)

var (
	upgrader = websocket.Upgrader{}
)

type Message struct {
	Content string `json:"content"`
	Nick    string `json:"nick"`
}

type Client struct {
	Conn *websocket.Conn
	Nick string
}

func NewClient(conn *websocket.Conn) *Client {
	return &Client{
		Conn: conn,
	}
}

func (c *Client) send(data Message) error {
	return c.Conn.WriteJSON(data)
}

type Dispatcher struct {
	clients map[*Client]bool
	in      chan Message
}

func newDispatcher() *Dispatcher {
	d := &Dispatcher{
		clients: make(map[*Client]bool),
		in:      make(chan Message),
	}
	go func() {
		for {
			msg := <-d.in
			fmt.Printf("Received message: %v\n", msg)
			d.sendAll(msg)
		}
	}()
	return d
}

func (d *Dispatcher) addClient(c *Client) error {
	d.clients[c] = true
	// create listening goroutine
	go func() {
		for {
			msg := Message{}
			err := c.Conn.ReadJSON(&msg)
			if err != nil {
				fmt.Printf("Error reading message: %v\n", err)
				d.removeClient(c)
				break
			}
			d.in <- msg
		}
	}()
	return nil
}

func (d *Dispatcher) removeClient(c *Client) {
	_ = c.Conn.Close()
	delete(d.clients, c)
}

func (d *Dispatcher) sendAll(data Message) {
	for c := range d.clients {
		err := c.send(data)
		if err != nil {
			fmt.Printf("Error sending message: %v\n", err)
			d.removeClient(c)
		}
	}
}

func main() {
	dispatcher := newDispatcher()

	e := echo.New()
	e.Use(middleware.Logger())
	// e.Use(middleware.Recover())
	e.Use(session.Middleware(sessions.NewCookieStore([]byte("lknosauhfuhdsaa7aayfsdad09w7fshdj"))))

	e.Static("/", "./public")

	e.GET("/chatroom", func(c echo.Context) error {
		// upgrade the connection to a websocket
		ws, err := upgrader.Upgrade(c.Response(), c.Request(), nil)
		if err != nil {
			return err
		}
		// add the client to the dispatcher
		return dispatcher.addClient(NewClient(ws))
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
