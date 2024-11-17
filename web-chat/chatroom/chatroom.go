package chatroom

import (
	"fmt"

	"github.com/gorilla/websocket"
	"github.com/sym01/htmlsanitizer"
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

func sanitiseMessage(msg Message) (Message, error) {
	content, err := htmlsanitizer.SanitizeString(msg.Content)
	if err != nil {
		return Message{}, err
	}
	nick, err := htmlsanitizer.SanitizeString(msg.Nick)
	if err != nil {
		return Message{}, err
	}
	return Message{
		Content: content,
		Nick:    nick,
	}, nil
}

func NewDispatcher() *Dispatcher {
	d := &Dispatcher{
		clients: make(map[*Client]bool),
		in:      make(chan Message),
	}
	go func() {
		for {
			msg := <-d.in
			fmt.Printf("Received message: %v\n", msg)
			sanitisedMessage, err := sanitiseMessage(msg)
			if err != nil {
				fmt.Printf("Error sanitising message: %v\n", err)
				continue
			}
			d.SendAll(sanitisedMessage)
		}
	}()
	return d
}

func (d *Dispatcher) AddClient(c *Client) error {
	d.clients[c] = true
	// create listening goroutine
	go func() {
		for {
			msg := Message{}
			err := c.Conn.ReadJSON(&msg)
			if err != nil {
				fmt.Printf("Error reading message: %v\n", err)
				d.RemoveClient(c)
				break
			}
			d.in <- msg
		}
	}()
	return nil
}

func (d *Dispatcher) RemoveClient(c *Client) {
	_ = c.Conn.Close()
	delete(d.clients, c)
}

func (d *Dispatcher) SendAll(data Message) {
	for c := range d.clients {
		err := c.send(data)
		if err != nil {
			fmt.Printf("Error sending message: %v\n", err)
			d.RemoveClient(c)
		}
	}
}
