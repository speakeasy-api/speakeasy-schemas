package apis

// Api is the shared representation of an API between registry and client(Speakeasy SDK).
type Api struct {
	WorkspaceID string `json:"workspace_id" db:"workspace_id"`         // Uniquely identifies the workspace this Api belongs to.
	ApiID       string `json:"api_id" db:"api_id"`                     // Uniquely identifies an api.
	Method      string `json:"method" db:"method"`                     // HTTP verb.
	Path        string `json:"path" db:"path"`                         // Path that handles this Api.
	DisplayName string `json:"name" db:"name"`                         // A human-friendly name.
	Description string `json:"description,omitempty" db:"description"` // A detailed description.
}
