import { Box } from "@mui/material";
import React, { useEffect, useState } from "react";
import CustomAutocomplete from "./CustomAutocomplete";
import { Device } from "../types/device";
import { API_URL, GET_ALL_DEVICES_ENDPOINT } from "../Constants";
import { DeviceVersion } from "../types/device_versions";

const DeviceVersionSearch = (props) => {
  const [devices, setDevices] = useState<Device[]>([]);
  const [selectedDevice, setSelectedDevice] = useState<Device | null>(null);

  const [deviceVersions, setDeviceVersions] = useState<DeviceVersion[]>([]);

  const displayDeviceChoice = (device: Device) =>
    (device.display_name ?? "Unknown") + " - " + device.model_code;

  const displayVersionChoice = (version) => version.version;

  useEffect(() => {
    fetch(GET_ALL_DEVICES_ENDPOINT)
      .then((response) => response.json())
      .then((data) => setDevices(data))
      .catch((error) => console.log(error));
  }, []);

  useEffect(() => {
    if (selectedDevice) {
      fetch(`${API_URL}/device/${selectedDevice.id}/operating_system_versions`)
        .then((response) => response.json())
        .then((data) => setDeviceVersions(data))
        .catch((error) => console.log(error));
    }
  }, [selectedDevice]);

  return (
    <Box sx={{ display: "flex", flexDirection: "row", gap: "1rem" }}>
      <CustomAutocomplete
        options={devices.map((device) => ({
          id: device.id,
          label: displayDeviceChoice(device),
        }))}
        inputLabel="Device"
        onChange={(event, newValue) => {
          const selectedDevice = devices.find(
            (device) => device.id === newValue.id,
          );
          setSelectedDevice(selectedDevice || null);
        }}
      />

      <CustomAutocomplete
        disabled={deviceVersions.length === 0}
        options={deviceVersions.map((deviceVersion) => ({
          id: deviceVersion.id,
          label: displayVersionChoice(deviceVersion),
        }))}
        inputLabel="Device version"
        onChange={(event, newValue) => {
          const selectedDeviceVersion = deviceVersions.find(
            (deviceVersion) => deviceVersion.id === newValue.id,
          );
          props.setSelectedDeviceVersion(selectedDeviceVersion || null);
        }}
      />
    </Box>
  );
};

export default DeviceVersionSearch;
