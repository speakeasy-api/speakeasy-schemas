package schemas

import (
	"github.com/SamuelTissot/sqltime"
)

type FileInfo struct {
	MimeType  string // The type of the file.
	Size      int64  // The size of the file in bytes.
	Hash      string // The MD5 hash of the file.
	SourceURI string // The GCS URI of the file.
}

// unfortunately we end up with the duplication of fields in models (instead of embedding structs) due to bugs with gorm handling embedded structs
type Schema struct {
	FileInfo
	ID          uint   `gorm:"primarykey"`
	ApiId       string `json:"api_id"`
	VersionId   string `json:"version_id"`
	RevisionId  string `json:"revision_id"`
	Filename    string `json:"filename"`
	Description string `json:"description"`
	// needing to use a third pary sqltime.Time package to avoid a bug in gorm where it want handle time.Time resolution correctly
	CreatedAt sqltime.Time `json:"created_at"`
	UpdatedAt sqltime.Time `json:"updated_at"`
}
