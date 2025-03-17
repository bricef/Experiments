package chatroom

type Channel struct {
	Name        string
	Subscribers []*Client
}

func NewChannel(name string) *Channel {
	return &Channel{
		Name:        name,
		Subscribers: make([]*Client, 0),
	}
}

func (c *Channel) Send(msg interface{}) {
	for _, client := range c.Subscribers {
		client.send(msg)
	}
}

func (c *Channel) AddClient(client *Client) {
	c.Subscribers = append(c.Subscribers, client)
}

func (c *Channel) RemoveClient(client *Client) {
	for i, cl := range c.Subscribers {
		if cl == client {
			c.Subscribers = append(c.Subscribers[:i], c.Subscribers[i+1:]...)
		}
	}
}

func (c *Channel) IsActive() bool {
	if c.Name == "general" {
		return true
	}
	return len(c.Subscribers) != 0
}
