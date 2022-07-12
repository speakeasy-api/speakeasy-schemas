package apimetadata

type ApiMetadata struct {
	WorkspaceID string `json:"workspace_id" db:"workspace_id"` // Uniquely identifies the workspace this Api belongs to.
	ApiID       string `json:"api_id" db:"api_id"`
	VersionID   string `json:"version_id" db:"version_id"`
	Key         string `json:"meta_key" db:"meta_key"`
	Value       string `json:"meta_value" db:"meta_value"`
}
