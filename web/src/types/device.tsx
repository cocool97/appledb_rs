import { OperatingSystemVersion } from "./operating_system_version";

export interface Device {
  id: number;
  display_name: string;
  model_code: string;
  versions: [OperatingSystemVersion];
}
