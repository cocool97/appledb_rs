export const DRAWER_WIDTH = 240;

export const API_URL = import.meta.env.PROD ? "" : "http://127.0.0.1:4000"

export const GET_ALL_DEVICES_ENDPOINT = `${API_URL}/api/v1/devices/all`

export const GET_ALL_OPERATING_SYSTEM_VERSIONS_ENDPOINT = `${API_URL}/api/v1/operating_system_versions/all`

export const GET_EXTENDED_OPERATING_SYSTEM_VERSIONS = `${API_URL}/api/v1/operating_system_versions/extended`

export const GET_ALL_EXECUTABLES_ENDPOINT = `${API_URL}/api/v1/executables/all`

// ROUTES
export const MAIN_ROUTE = "/"
export const STATS_ROUTE = "/stats"
export const ENTITLEMENTS_DIFF_ROUTE = "/entitlements/diff"
export const EXECUTABLES_DIFF_ROUTE = "/executables/diff"
export const MODELS = "/model/:modelId"
export const ENTITLEMENTS_VERSION = "/model/:modelId/version/:versionId"