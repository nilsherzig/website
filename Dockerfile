FROM golang:1.24-alpine AS builder

WORKDIR /app
COPY go.mod ./
RUN go mod download
COPY . .
RUN CGO_ENABLED=0 GOOS=linux go build -o website .

FROM alpine:3.21

RUN adduser -D -u 1000 appuser
USER appuser
WORKDIR /app
COPY --from=builder /app/website .

EXPOSE 8080
CMD ["./website"]
