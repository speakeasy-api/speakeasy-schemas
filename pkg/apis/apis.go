package apis

import (
	"github.com/SamuelTissot/sqltime"
)

// Api is the shared representation of an API between registry and client(Speakeasy SDK).
type Api struct {
	ID          uint         `gorm:"primarykey"`
	WorkspaceID string       `json: "workspace_id"` // Uniquely identifies the workspace this Api belongs to.
	Method      string       `json: "method"`       // HTTP verb.
	Path        string       `json: "path"`         // Path that handles this Api.
	CreatedAt   sqltime.Time `json: "created_at"`
	UpdatedAt   sqltime.Time `json: "updated_at"`
	DisplayName string       `json: "name"`                  // A human-friendly name.
	Description string       `json: "description,omitempty"` // A detailed description.
}
