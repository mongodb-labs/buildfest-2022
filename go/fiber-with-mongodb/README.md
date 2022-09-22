
# Moar Fiber - Fiber (Go) and MongoDB REST API

An API built with Fiber and MongoDB.

## Prerequisites

- [Go](https://golang.org/dl/)
- [MongoDB](https://www.mongodb.com/try/download/community)
- [Fiber](https://gofiber.io/)

## Installation

```bash
go get github.com/gofiber/fiber/v2
go get go.mongodb.org/mongo-driver/mongo
```

# Environment Variables

To run this project, you will need to add the following environment variables to your .env file

`MONGO_URI`= Your MongoDB URI

`DB`=sample_analytics

`PORT`=3001

`APP_ENV`=development

<br>

## Usage

```bash
go run main.go
```

## License

[MIT](https://choosealicense.com/licenses/mit/)



## API Usage

#### Get all customers data

```http
  GET /customers
```

| Optional Parameter | Type     | Description                                        | Example       |
| :----------------- | :------- | :------------------------------------------------- | :------------ |
| `s`                | `string` | Search Customer                  | ?s=example_text |
| `page`             | `int`	| Page number. Default: 1                            | ?page=2       |
| `limit`	     | `int`    | Limit number of customers data per page. Default: 10 | ?limit=20     |

#### Get customer data

```http
  GET /customers/:id
```

| Parameter | Type     | Description                       |
| :-------- | :------- | :-------------------------------- |
| `id`      | `string` | **Required**. Id of customer to fetch |


#### Add customer data

```http
  POST /customers
```


```json
{
    "name":"Alec Dorsey",
    "address":"Downtown, NYC",
    "email":"alec.dorsey@example.com"
}
```

#### Update customer data

```http
  PUT /customers/:id
```

| Parameter | Type     | Description                       |
| :-------- | :------- | :-------------------------------- |
| `id`      | `string` | **Required**. Id of customer to update |


```json
{
    "name":"Alec Dorsey",
    "address":"13th Ave SW Seattle",
    "email":"alec.dorsey@example.com"
}

// All three fields are optional when updating
```

#### Remove customer

```http
  DELETE /customers/:id
```

| Parameter | Type     | Description                       |
| :-------- | :------- | :-------------------------------- |
| `id`      | `string` | **Required**. Id of customer to delete |



## Contributor

- Kushagra Kesav
