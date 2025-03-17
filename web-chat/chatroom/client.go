package chatroom

import "github.com/gorilla/websocket"

type Client struct {
	Conn *websocket.Conn
}

func NewClient(conn *websocket.Conn) *Client {
	return &Client{
		Conn: conn,
	}
}

func (c *Client) send(data interface{}) error {
	return c.Conn.WriteJSON(data)
}
