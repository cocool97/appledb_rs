import { useEffect, useState } from "react";
import { API_URL, GET_EXTENDED_OPERATING_SYSTEM_VERSIONS } from "../Constants";
import React from "react";
import CustomAutocomplete from "../components/CustomAutocomplete";
import { Box } from "@mui/material";
import { DeviceVersion } from "../types/device_versions";
import { Framework } from "../types/framework";
import FrameworksLinking from "./FrameworksLinking";

export const Frameworks = () => {
  const [devices, setDevices] = useState<DeviceVersion[]>([]);
  const [selectedDevice, setSelectedDevice] = useState<DeviceVersion | null>(
    null,
  );
  const [frameworks, setFrameworks] = useState<Framework[]>([]);

  const [selectedFramework, setSelectedFramework] = useState<Framework | null>(
    null,
  );

  useEffect(() => {
    fetch(GET_EXTENDED_OPERATING_SYSTEM_VERSIONS)
      .then((response) => response.json())
      .then((data) => setDevices(data))
      .catch((error) => console.log(error));
  }, []);

  useEffect(() => {
    if (selectedDevice) {
      fetch(
        `${API_URL}/operating_system_versions/${selectedDevice.id}/frameworks`,
      )
        .then((response) => response.json())
        .then((data) => setFrameworks(data))
        .catch((error) => console.log(error));
    }
  }, [selectedDevice]);

  const displayVersionChoice = (version) =>
    (version.display_name ?? "Unknown") +
    " - " +
    version.model_code +
    " - " +
    version.version;

  return (
    <div style={{ display: "flex", flexDirection: "column", gap: "1rem" }}>
      <CustomAutocomplete
        options={devices.map((device) => ({
          id: device.id,
          label: displayVersionChoice(device),
        }))}
        inputLabel="Operating system"
        onChange={(event, newValue) => {
          const selectedDevice = devices.find(
            (device) => device.id === newValue.id,
          );
          setSelectedDevice(selectedDevice || null);
        }}
      />

      <CustomAutocomplete
        disabled={frameworks.length === 0}
        options={frameworks.map((framework) => framework.full_path)}
        inputLabel="Available frameworks"
        onChange={(event, newValue) => {
          const selectedFramework = frameworks.find(
            (framework) => framework.full_path === newValue,
          );
          setSelectedFramework(selectedFramework || null);
        }}
      />

      {selectedDevice && selectedFramework && (
        <Box
          sx={{
            display: "flex",
            flexDirection: "column",
            gap: "2rem",
          }}
        >
          <FrameworksLinking
            framework_id={selectedFramework.id}
            operating_system_version_id={selectedDevice.id}
          />
        </Box>
      )}
    </div>
  );
};
