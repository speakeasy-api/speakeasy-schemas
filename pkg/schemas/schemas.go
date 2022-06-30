package schemas

type FileInfo struct {
	MimeType  string // The type of the file.
	Size      int64  // The size of the file in bytes.
	Hash      string // The MD5 hash of the file.
	SourceURI string // The GCS URI of the file.
}

// Schema is the shared representation of a Schema between registry and client(Speakeasy SDK).
type Schema struct {
	FileInfo
	ID          uint
	ApiID       string `json:"api_id"`
	VersionID   string `json:"version_id"`
	RevisionID  string `json:"revision_id"`
	Filename    string `json:"filename"`
	Description string `json:"description"`
}
