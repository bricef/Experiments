FROM docker.io/golang:1.23 as build

WORKDIR /go/src/app
COPY . .

RUN go mod download
RUN CGO_ENABLED=0 go build -o chat ./cmd/server/server.go

# Now copy it into our base image.
FROM gcr.io/distroless/static-debian12
COPY --from=build /go/src/app/chat /chat
COPY ./public /public 
CMD ["/chat"]