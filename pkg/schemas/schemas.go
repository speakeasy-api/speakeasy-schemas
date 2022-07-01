package schemas

import "github.com/SamuelTissot/sqltime"

// Schema is the shared representation of a Schema between registry and client(Speakeasy SDK).
type Schema struct {
	ApiID       string       `json:"api_id"`
	VersionID   string       `json:"version_id"`
	RevisionID  string       `json:"revision_id"`
	Filename    string       `json:"filename"`
	Description string       `json:"description"`
	CreatedAt   sqltime.Time `json:"created_at"`
}
