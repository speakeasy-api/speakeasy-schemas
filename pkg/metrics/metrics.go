package metrics

type ApiData struct {
	ApiKey      string      `json:"api_key"`
	ApiServerID string      `json:"api_server_id"`
	Handlers    HandlerInfo `json:"handlers"`
}

type HandlerInfo struct {
	ApiStatsByID map[uint]*ApiStats `json:"api_stats_by_id"`
}

type ApiStats struct {
	NumCalls  int `json:"number_of_calls"`
	NumErrors int `json:"number_of_errors"`
}
