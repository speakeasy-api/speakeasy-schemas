package schemas

// Schema is the shared representation of a Schema between registry and client(Speakeasy SDK).
type Schema struct {
	ApiID       string `json:"api_id" db:"api_id"`
	VersionID   string `json:"version_id" db:"version_id"`
	SchemaID    string `json:"schema_id" db:"schema_id"`
	RevisionID  string `json:"revision_id" db:"revision_id"`
	Filename    string `json:"filename" db:"filename"`
	Description string `json:"description" db:"description"`
}
