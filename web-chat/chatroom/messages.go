package chatroom

import "github.com/sym01/htmlsanitizer"

type MessageInbound struct {
	Type    string `json:"type"`
	Content string `json:"content"`
	Nick    string `json:"nick"`
	Channel string `json:"channel"`
}

func (m MessageInbound) Sanitise() (MessageInbound, error) {

	content, err := htmlsanitizer.SanitizeString(m.Content)
	if err != nil {
		return MessageInbound{}, err
	}

	nick, err := htmlsanitizer.SanitizeString(m.Nick)
	if err != nil {
		return MessageInbound{}, err
	}

	if m.Type == "" {
		m.Type = "message"
	}
	mType, err := htmlsanitizer.SanitizeString(m.Type)
	if err != nil {
		return MessageInbound{}, err
	}
	if m.Channel == "" {
		m.Channel = "general"
	}
	channel, err := htmlsanitizer.SanitizeString(m.Channel)
	if err != nil {
		return MessageInbound{}, err
	}

	return MessageInbound{
		Type:    mType,
		Content: content,
		Nick:    nick,
		Channel: channel,
	}, nil
}

type ChannelsUpdateMessage struct {
	Type     string   `json:"type"`
	Channels []string `json:"channels"`
}

func NewChannelsUpdateMessage(channels []string) ChannelsUpdateMessage {
	return ChannelsUpdateMessage{
		Type:     "channelsUpdate",
		Channels: channels,
	}
}
