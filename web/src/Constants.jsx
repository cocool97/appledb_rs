
export const API_URL = import.meta.env.PROD ? "" : "http://127.0.0.1:4000" 

export const GET_ALL_DEVICES_ENDPOINT = `${API_URL}/api/v1/devices/all`