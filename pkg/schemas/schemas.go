package schemas

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
	ApiID       string `json:"api_id"`
	VersionID   string `json:"version_id"`
	RevisionID  string `json:"revision_id"`
	Filename    string `json:"filename"`
	Description string `json:"description"`
}
