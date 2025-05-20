import { useEffect, useState } from "react";
import { API_URL, GET_EXTENDED_OPERATING_SYSTEM_VERSIONS } from "../Constants";
import React from "react";
import CustomAutocomplete from "../components/CustomAutocomplete";
import { Box } from "@mui/material";
import { useSearchParams } from "react-router-dom";
import { ExecutableOperatingSystem } from "../types/executable_operating_system";
import { DeviceVersion } from "../types/device_versions";
import ExecutableEntitlements from "./ExecutableEntitlements";
import ExecutableFrameworks from "./ExecutableFrameworks";

const EXECUTABLE_ID_SEARCH_PARAM = "executable_id";

export const Executables = () => {
  const [searchParams, setSearchParams] = useSearchParams();

  const [devices, setDevices] = useState<DeviceVersion[]>([]);
  const [selectedDevice, setSelectedDevice] = useState<DeviceVersion | null>(
    null,
  );
  const [executables, setExecutables] = useState<ExecutableOperatingSystem[]>(
    [],
  );

  const executable_id = searchParams.get(EXECUTABLE_ID_SEARCH_PARAM);
  const [selectedExecutableId, setSelectedExecutableId] = useState<
    number | null
  >(executable_id ? parseInt(executable_id) : null);

  useEffect(() => {
    fetch(GET_EXTENDED_OPERATING_SYSTEM_VERSIONS)
      .then((response) => response.json())
      .then((data) => setDevices(data))
      .catch((error) => console.log(error));
  }, []);

  useEffect(() => {
    if (selectedDevice) {
      fetch(
        `${API_URL}/operating_system_versions/${selectedDevice.id}/executables`,
      )
        .then((response) => response.json())
        .then((data) => setExecutables(data))
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
        disabled={executables.length === 0}
        options={executables.map((executable) => executable.full_path)}
        inputLabel="Available executables"
        onChange={(event, newValue) => {
          const selectedExecutable = executables.find(
            (executables) => executables.full_path === newValue,
          );
          const selectedExecutableId =
            selectedExecutable?.executable_operating_system_id;

          if (selectedExecutableId) {
            setSelectedExecutableId(selectedExecutableId ?? null);
            searchParams.set(
              EXECUTABLE_ID_SEARCH_PARAM,
              selectedExecutableId.toString(),
            );
            setSearchParams(searchParams);
          }
        }}
      />

      {selectedExecutableId && (
        <Box
          sx={{
            display: "flex",
            flexDirection: "column",
            gap: "2rem",
          }}
        >
          <ExecutableEntitlements
            executable_operating_system_id={selectedExecutableId}
          />

          <ExecutableFrameworks
            executable_operating_system_id={selectedExecutableId}
          />
        </Box>
      )}
    </div>
  );
};
