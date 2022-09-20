FROM golang:1.19-alpine

WORKDIR /go/src/app
COPY ./gosrc ./gosrc
COPY ./go.mod .
COPY ./go.sum .

RUN go build -v -o buildfest ./gosrc

CMD ["./buildfest"]