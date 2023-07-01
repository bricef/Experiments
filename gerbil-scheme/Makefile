

build: Dockerfile
		# docker build -t schemebox:latest --progress=plain --no-cache .
		docker build -t schemebox:latest .

run:
	docker run --rm -t schemebox:latest

try: build run

app:
	gxc -v -g -static -exe -o app src/entrypoint.ss

.PHONY: build run try