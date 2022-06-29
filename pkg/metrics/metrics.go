package metrics

type ApiData struct {
	ApiKey      string      `json:"api_key"`
	ApiServerId string      `json:"api_server_id"`
	Handlers    HandlerInfo `json:"handlers"`
}

type HandlerInfo struct {
	ApiStatsById map[uint]*ApiStats
}

type ApiStats struct {
	NumCalls           int `json:"number_of_calls"`
	NumErrors          int `json:"number_of_errors"`
	NumUniqueCustomers int `json:"number_of_unique_customers"`
}
