package chatroom

import (
	"fmt"
)

type ReceivedMessage struct {
	Message MessageInbound
	Client  *Client
}

type Dispatcher struct {
	clients  map[*Client]bool
	in       chan ReceivedMessage
	channels map[string]*Channel
}

func NewDispatcher() *Dispatcher {
	d := &Dispatcher{
		clients:  make(map[*Client]bool),
		in:       make(chan ReceivedMessage),
		channels: make(map[string]*Channel),
	}

	go func() {
		for {
			msg := <-d.in
			// fmt.Printf("Received message: %v\n", msg)
			sanitisedMessage, err := msg.Message.Sanitise()
			fmt.Printf("Sanitised message: %v\n", sanitisedMessage)
			if err != nil {
				fmt.Printf("Error sanitising message: %v\n", err)
				continue
			}
			switch sanitisedMessage.Type {
			case "message":
				d.DispatchMessage(sanitisedMessage, msg.Client)
			case "join":
				d.AddClientToChannel(sanitisedMessage.Channel, msg.Client)
			case "leave":
				d.RemoveClientFromChannel(sanitisedMessage, msg.Client)
			}
		}
	}()
	return d
}

func (d *Dispatcher) mkChannel(channelName string) *Channel {
	channel, exists := d.channels[channelName]
	if !exists {
		channel = NewChannel(channelName)
		d.channels[channelName] = channel
	}
	d.sendAll(ChannelsUpdateMessage{
		Type:     "channelsUpdate",
		Channels: d.GetChannels(),
	})
	return channel
}

func (d *Dispatcher) sendAll(msg interface{}) {
	for _, channel := range d.channels {
		channel.Send(msg)
	}
}

func (d *Dispatcher) DispatchMessage(msg MessageInbound, client *Client) {
	channel, exists := d.channels[msg.Channel]
	if !exists {
		// Create new channel if it doesn't exist
		channel = d.mkChannel(msg.Channel)
	}
	channel.Send(msg)
}

func (d *Dispatcher) AddClientToChannel(channelName string, client *Client) {
	channel, exists := d.channels[channelName]
	if !exists {
		channel = NewChannel(channelName)
		d.channels[channelName] = channel
	}
	channel.AddClient(client)
}

func (d *Dispatcher) RemoveClientFromChannel(msg MessageInbound, client *Client) {
	channel, exists := d.channels[msg.Channel]
	if !exists {
		// Nothing to do if channel doesn't exist
		return
	}
	channel.RemoveClient(client)
}

func (d *Dispatcher) AddClient(c *Client) error {
	d.clients[c] = true
	d.AddClientToChannel("general", c)
	// create listening goroutine
	go func() {
		for {
			msg := MessageInbound{}
			err := c.Conn.ReadJSON(&msg)
			if err != nil {
				fmt.Printf("Error reading message: %v\n", err)
				d.RemoveClient(c)
				break
			}
			d.in <- ReceivedMessage{
				Message: msg,
				Client:  c,
			}
		}
	}()
	return nil
}

func (d *Dispatcher) RemoveClient(c *Client) {
	_ = c.Conn.Close()
	delete(d.clients, c)
}

func (d *Dispatcher) GetChannels() []string {
	channels := make([]string, 0, len(d.channels))
	for channel := range d.channels {
		if d.channels[channel].IsActive() {
			channels = append(channels, channel)
		}
	}
	return channels
}
