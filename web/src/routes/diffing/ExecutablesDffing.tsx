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

const ExecutablesDiffing = () => {
  const [devices, setDevices] = useState<DeviceVersion[]>([]);

  const [fromDeviceId, setFromDeviceId] = useState<number | null>(null);

  const [toDeviceId, setToDeviceId] = useState<number | null>(null);

  const [diff, setDiff] = useState<Diff | null>(null);

  useEffect(() => {
    fetch(GET_EXTENDED_OPERATING_SYSTEM_VERSIONS)
      .then((response) => response.json())
      .then((data) => setDevices(data))
      .catch((error) => console.log(error));
  }, []);

  useEffect(() => {
    if (fromDeviceId && toDeviceId) {
      fetch(`${API_URL}/executables/diff/${fromDeviceId}/${toDeviceId}`)
        .then((response) => response.json())
        .then((data) => setDiff(data))
        .catch((error) => console.log(error));
    }
  }, [fromDeviceId, toDeviceId]);

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
      </Box>

      <DiffResults
        diff={diff}
        mainCellLabel="Executable name"
        secondaryCellLabel="Executable full path"
        mainCellLabelGetter={(item) => item.name}
        secondaryCellLabelGetter={(item) => item.full_path}
      />
    </div>
  );
};

export default ExecutablesDiffing;
