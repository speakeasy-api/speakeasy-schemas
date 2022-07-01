package apis

import "time"

// Api is the shared representation of an API between registry and client(Speakeasy SDK).
type Api struct {
	WorkspaceID string    `json: "workspace_id"`          // Uniquely identifies the workspace this Api belongs to.
	Method      string    `json: "method"`                // HTTP verb.
	Path        string    `json: "path"`                  // Path that handles this Api.
	DisplayName string    `json: "name"`                  // A human-friendly name.
	Description string    `json: "description,omitempty"` // A detailed description.
	CreatedAt   time.Time `json:"created_at"`
	UpdatedAt   time.Time `json:"updated_at"`
}
