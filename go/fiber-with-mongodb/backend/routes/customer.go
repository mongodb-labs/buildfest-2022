package routes

import (
	"example.com/m/v2/controllers"
	"github.com/gofiber/fiber/v2"
)

func CustomerRoute(route fiber.Router) {
	route.Get("/", controllers.GetAllcustomers)
	route.Get("/:id", controllers.GetCustomer)
	route.Post("/", controllers.AddCustomer)
	route.Put("/:id", controllers.UpdateCustomer)
	route.Delete("/:id", controllers.DeleteCustomer)
}
