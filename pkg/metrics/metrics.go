package metrics

type ApiData struct {
	ApiKey       string               `json:"api_key"`
	ApiServerID  string               `json:"api_server_id"`
	ApiStatsByID map[string]*ApiStats `json:"api_stats_by_id"`
}

type ApiStats struct {
	NumCalls  int `json:"num_calls"`
	NumErrors int `json:"num_errors"`
}
