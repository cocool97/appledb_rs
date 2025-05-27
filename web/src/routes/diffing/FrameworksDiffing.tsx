import React, { useEffect, useState } from "react";
import {
  API_URL,
  GET_EXTENDED_OPERATING_SYSTEM_VERSIONS,
} from "../../Constants";
import CustomAutocomplete from "../../components/CustomAutocomplete";
import DiffResults from "../../components/DiffResults";
import { Diff } from "../../types/diff";
import { Box } from "@mui/material";
import { DeviceVersion } from "../../types/device_versions";
import { ExecutableOperatingSystem } from "../../types/executable_operating_system";

const FrameworksDiffing = () => {
  const [devices, setDevices] = useState<DeviceVersion[]>([]);

  const [fromDeviceId, setFromDeviceId] = useState<number | null>(null);
  const [fromExecutables, setFromExecutables] = useState<
    ExecutableOperatingSystem[]
  >([]);
  const [fromExecutableId, setFromExecutableId] = useState<number | null>(null);

  const [toDeviceId, setToDeviceId] = useState<number | null>(null);
  const [toExecutables, setToExecutables] = useState<
    ExecutableOperatingSystem[]
  >([]);
  const [toExecutableId, setToExecutableId] = useState<number | null>(null);

  const [diff, setDiff] = useState<Diff | null>(null);

  useEffect(() => {
    fetch(GET_EXTENDED_OPERATING_SYSTEM_VERSIONS)
      .then((response) => response.json())
      .then((data) => setDevices(data))
      .catch((error) => console.log(error));
  }, []);

  useEffect(() => {
    if (fromDeviceId) {
      fetch(`${API_URL}/operating_system_versions/${fromDeviceId}/executables`)
        .then((response) => response.json())
        .then((data) => setFromExecutables(data))
        .catch((error) => console.log(error));
    }
  }, [fromDeviceId]);

  useEffect(() => {
    if (toDeviceId) {
      fetch(`${API_URL}/operating_system_versions/${toDeviceId}/executables`)
        .then((response) => response.json())
        .then((data) => setToExecutables(data))
        .catch((error) => console.log(error));
    }
  }, [toDeviceId]);

  useEffect(() => {
    if (fromExecutableId && toExecutableId) {
      fetch(`${API_URL}/frameworks/diff/${fromExecutableId}/${toExecutableId}`)
        .then((response) => response.json())
        .then((data) => setDiff(data))
        .catch((error) => console.log(error));
    }
  }, [fromExecutableId, toExecutableId]);

  const displayVersionChoice = (version) =>
    (version.display_name ?? "Unknown") +
    " - " +
    version.model_code +
    " - " +
    version.version;

  return (
    <div style={{ display: "flex", flexDirection: "column", gap: "1rem" }}>
      <Box sx={{ display: "flex", flexDirection: "row", gap: "1rem" }}>
        <CustomAutocomplete
          options={devices.map((device) => displayVersionChoice(device))}
          inputLabel="From version"
          onChange={(event, newValue) => {
            const selectedVersion = devices.find(
              (version) => displayVersionChoice(version) === newValue,
            );
            setFromDeviceId(selectedVersion?.id || null);
          }}
        />

        <CustomAutocomplete
          disabled={!fromDeviceId}
          options={fromExecutables.map((executable) => executable.full_path)}
          inputLabel="From executable"
          onChange={(event, newValue) => {
            const selectedExecutable = fromExecutables.find(
              (exec) => exec.full_path === newValue,
            );
            setFromExecutableId(
              selectedExecutable?.executable_operating_system_id || null,
            );
          }}
        />
      </Box>

      <Box sx={{ display: "flex", flexDirection: "row", gap: "1rem" }}>
        <CustomAutocomplete
          options={devices.map((device) => displayVersionChoice(device))}
          inputLabel="To version"
          onChange={(event, newValue) => {
            const selectedVersion = devices.find(
              (version) => displayVersionChoice(version) === newValue,
            );
            setToDeviceId(selectedVersion?.id || null);
          }}
        />

        <CustomAutocomplete
          disabled={!toDeviceId}
          options={toExecutables.map((executable) => executable.full_path)}
          inputLabel="Executable"
          onChange={(event, newValue) => {
            const selectedExecutable = toExecutables.find(
              (exec) => exec.full_path === newValue,
            );
            setToExecutableId(
              selectedExecutable?.executable_operating_system_id || null,
            );
          }}
        />
      </Box>

      <DiffResults
        diff={diff}
        mainCellLabel="Framework name"
        mainCellLabelGetter={(item) => item.full_path}
      />
    </div>
  );
};

export default FrameworksDiffing;
