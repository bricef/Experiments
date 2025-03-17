package chatroom

import (
	"fmt"
	"sync"

	"github.com/gorilla/websocket"
	"github.com/sym01/htmlsanitizer"
)

type MessageType string

const (
	MessageTypeChat    MessageType = "chat"
	MessageTypeSystem  MessageType = "system"
	MessageTypeChannel MessageType = "channel"
)

type Message struct {
	Type    MessageType `json:"type"`
	Content string      `json:"content"`
	Nick    string      `json:"nick"`
	Channel string      `json:"channel"`
}

type Client struct {
	Conn    *websocket.Conn
	Nick    string
	Channel string
}

func NewClient(conn *websocket.Conn) *Client {
	return &Client{
		Conn: conn,
	}
}

func (c *Client) send(data Message) error {
	return c.Conn.WriteJSON(data)
}

type Channel struct {
	name    string
	clients map[*Client]bool
	mu      sync.RWMutex
}

func NewChannel(name string) *Channel {
	return &Channel{
		name:    name,
		clients: make(map[*Client]bool),
	}
}

func (ch *Channel) AddClient(c *Client) {
	ch.mu.Lock()
	defer ch.mu.Unlock()
	ch.clients[c] = true
	c.Channel = ch.name
}

func (ch *Channel) RemoveClient(c *Client) {
	ch.mu.Lock()
	defer ch.mu.Unlock()
	delete(ch.clients, c)
}

func (ch *Channel) CloseClient(c *Client) {
	ch.mu.Lock()
	defer ch.mu.Unlock()
	delete(ch.clients, c)
	_ = c.Conn.Close()
}

func (ch *Channel) Broadcast(msg Message) {
	ch.mu.RLock()
	defer ch.mu.RUnlock()

	for c := range ch.clients {
		err := c.send(msg)
		if err != nil {
			fmt.Printf("Error sending message: %v\n", err)
			ch.CloseClient(c)
		}
	}
}

func (ch *Channel) IsEmpty() bool {
	ch.mu.RLock()
	defer ch.mu.RUnlock()
	return len(ch.clients) == 0
}

type Dispatcher struct {
	channels map[string]*Channel
	in       chan Message
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
	channel, err := htmlsanitizer.SanitizeString(msg.Channel)
	if err != nil {
		return Message{}, err
	}
	return Message{
		Type:    msg.Type,
		Content: content,
		Nick:    nick,
		Channel: channel,
	}, nil
}

func NewDispatcher() *Dispatcher {
	d := &Dispatcher{
		channels: make(map[string]*Channel),
		in:       make(chan Message),
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
			d.SendToChannel(sanitisedMessage)
		}
	}()
	return d
}

func (d *Dispatcher) AddClient(c *Client, channelName string) error {
	// Get or create channel
	ch, exists := d.channels[channelName]
	if !exists {
		ch = NewChannel(channelName)
		d.channels[channelName] = ch
		// Broadcast new channel to all clients
		d.broadcastChannelUpdate()
	}
	ch.AddClient(c)

	// Send current channels list to the new client
	c.send(Message{
		Type:    MessageTypeChannel,
		Content: "channels",
		Channel: "system",
	})

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

			// Handle system messages
			if msg.Type == MessageTypeSystem && msg.Content == "switch_channel" {
				d.SwitchClientChannel(c, msg.Channel)
				continue
			}

			msg.Channel = c.Channel // Ensure message has correct channel
			d.in <- msg
		}
	}()
	return nil
}

func (d *Dispatcher) RemoveClient(c *Client) {
	if ch, exists := d.channels[c.Channel]; exists {
		ch.CloseClient(c)
		if ch.IsEmpty() && c.Channel != "general" {
			delete(d.channels, c.Channel)
			// Broadcast channel removal to all clients
			d.broadcastChannelUpdate()
		}
	}
}

func (d *Dispatcher) SendToChannel(data Message) {
	if ch, exists := d.channels[data.Channel]; exists {
		ch.Broadcast(data)
	}
}

func (d *Dispatcher) broadcastChannelUpdate() {
	msg := Message{
		Type:    MessageTypeChannel,
		Content: "channels",
		Channel: "system",
	}
	// Broadcast to all clients in all channels
	for _, ch := range d.channels {
		ch.Broadcast(msg)
	}
}

func (d *Dispatcher) GetChannels() []string {
	channels := make([]string, 0, len(d.channels))
	for name := range d.channels {
		channels = append(channels, name)
	}
	return channels
}

func (d *Dispatcher) SwitchClientChannel(c *Client, newChannel string) {
	// Remove from old channel
	if oldCh, exists := d.channels[c.Channel]; exists {
		oldCh.CloseClient(c)
		if oldCh.IsEmpty() && c.Channel != "general" {
			delete(d.channels, c.Channel)
			d.broadcastChannelUpdate()
		}
	}

	// Add to new channel
	ch, exists := d.channels[newChannel]
	if !exists {
		ch = NewChannel(newChannel)
		d.channels[newChannel] = ch
		d.broadcastChannelUpdate()
	}
	ch.AddClient(c)
}
