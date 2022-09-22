package models

import (
	"time"
	"go.mongodb.org/mongo-driver/bson/primitive"
)

type Customer struct {
	ID        primitive.ObjectID `json:"_id,omitempty" bson:"_id,omitempty"`
	Name      string             `json:"name,omitempty" bson:"name,omitempty"`
	Address   string             `json:"address,omitempty" bson:"address,omitempty"`
	BirthDate time.Time          `json:"birthdate,omitempty" bson:"birthdate,omitempty"`
	Email     string             `json:"email,omitempty" bson:"email,omitempty"`
}
