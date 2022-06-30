package metrics

// ApiData represents data relating to a particular server and the API calls it handles
type ApiData struct {
	ApiServerID  string               `json:"api_server_id"`
	ApiStatsByID map[string]*ApiStats `json:"api_stats_by_id"`
}

// ApiStats represents stats for a particular API call
type ApiStats struct {
	NumCalls  int `json:"num_calls"`
	NumErrors int `json:"num_errors"`
}
