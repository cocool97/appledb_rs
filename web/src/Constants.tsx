export const DRAWER_WIDTH = 240;

//////////////////////////
// BACKEND ENDPOINTS
const SERVER_URL = import.meta.env.PROD ? "" : "http://127.0.0.1:4000";
const API_VERSION = "/api/v1";
export const API_URL = `${SERVER_URL}${API_VERSION}`;

export const GET_ALL_DEVICES_ENDPOINT = `${API_URL}/devices/all`;
export const GET_ALL_OPERATING_SYSTEM_VERSIONS_ENDPOINT = `${API_URL}/operating_system_versions/all`;
export const GET_EXTENDED_OPERATING_SYSTEM_VERSIONS = `${API_URL}/operating_system_versions/extended`;
export const GET_ALL_EXECUTABLES_ENDPOINT = `${API_URL}/executables/all`;
export const GET_ALL_FRAMEWORKS_ENDPOINT = `${API_URL}/frameworks/all`;
export const GET_RUNNING_TASKS = `${API_URL}/tasks/running`;
//////////////////////////

//////////////////////////
// LOCAL WEB-UI ROUTES
export const HOME_ROUTE = "/";
export const EXECUTABLES_ROUTE = "/executables";
export const FRAMEWORKS_ROUTE = "/frameworks";
export const EXECUTABLE_ENTITLEMENTS_ROUTE =
  "/executable/:executable_operating_system_id/entitlements";
export const EXECUTABLE_FRAMEWORKS_ROUTE =
  "/executable/:executable_operating_system_id/frameworks";
export const FRAMEWORK_OPERATING_SYSTEM_VERSION_EXECUTABLES =
  "/operating_system_version/:operating_system_version_id/framework/:framework_id";
export const ENTITLEMENTS_SEARCH_ROUTE = "/entitlements/search";
export const ENTITLEMENTS_DIFF_ROUTE = "/entitlements/diff";
export const FRAMEWORKS_DIFF_ROUTE = "/frameworks/diff";
export const EXECUTABLES_DIFF_ROUTE = "/executables/diff";
export const TASKS_ROUTE = "/tasks";
//////////////////////////
