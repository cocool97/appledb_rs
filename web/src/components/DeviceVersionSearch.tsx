import { Box } from "@mui/material";
import React, { useEffect, useMemo, useState } from "react";
import CustomAutocomplete from "./CustomAutocomplete";
import { Device } from "../types/device";
import {
  API_URL,
  DEVICE_ID_SEARCH_PARAM,
  DEVICE_VERSION_ID_SEARCH_PARAM,
  GET_ALL_DEVICES_ENDPOINT,
} from "../Constants";
import { DeviceVersion } from "../types/device_versions";
import { useSearchParams } from "react-router-dom";

interface Props {
  setSelectedDeviceVersionId: (selectedDeviceVersionId: number | null) => void;
}

export const DeviceVersionSearch: React.FC<Props> = ({
  setSelectedDeviceVersionId,
}) => {
  const [searchParams, setSearchParams] = useSearchParams();

  const [devices, setDevices] = useState<Device[]>([]);

  const selectedDeviceIdParam = searchParams.get(DEVICE_ID_SEARCH_PARAM);
  const [selectedDeviceId, setSelectedDeviceId] = useState<number | null>(
    selectedDeviceIdParam ? parseInt(selectedDeviceIdParam) : null,
  );

  const [deviceVersions, setDeviceVersions] = useState<DeviceVersion[]>([]);

  const selectedDeviceVersionIdParam = searchParams.get(
    DEVICE_VERSION_ID_SEARCH_PARAM,
  );
  const [selectedDeviceVersionId, setInnerSelectedDeviceVersionId] = useState<
    number | null
  >(
    selectedDeviceVersionIdParam
      ? parseInt(selectedDeviceVersionIdParam)
      : null,
  );

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
    if (selectedDeviceId) {
      fetch(`${API_URL}/device/${selectedDeviceId}/operating_system_versions`)
        .then((response) => response.json())
        .then((data) => setDeviceVersions(data))
        .catch((error) => console.log(error));
    }
  }, [selectedDeviceId]);

  const selectedDeviceOption = useMemo(() => {
    const device = devices.find((d) => d.id === selectedDeviceId);
    return device
      ? {
          id: device.id,
          label: displayDeviceChoice(device),
        }
      : null;
  }, [devices, selectedDeviceId]);

  const selectedDeviceVersionOption = useMemo(() => {
    const version = deviceVersions.find(
      (v) => v.id === selectedDeviceVersionId,
    );
    return version
      ? {
          id: version.id,
          label: displayVersionChoice(version),
        }
      : null;
  }, [deviceVersions, selectedDeviceVersionId]);

  useEffect(() => {
    setSelectedDeviceVersionId(selectedDeviceVersionId);
  }, [selectedDeviceVersionId]);

  return (
    <Box sx={{ display: "flex", flexDirection: "row", gap: 4 }}>
      <CustomAutocomplete
        options={devices.map((device) => ({
          id: device.id,
          label: displayDeviceChoice(device),
        }))}
        inputLabel="Device"
        value={selectedDeviceOption}
        onChange={(event, newValue) => {
          const selectedDevice = devices.find(
            (device) => device.id === newValue.id,
          );

          const selectedDeviceId = selectedDevice?.id;
          if (selectedDeviceId) {
            setSearchParams((searchParams) => {
              searchParams.set(
                DEVICE_ID_SEARCH_PARAM,
                selectedDeviceId.toString(),
              );
              return searchParams;
            });
            setSelectedDeviceId(selectedDeviceId);
          }
        }}
      />

      <CustomAutocomplete
        disabled={deviceVersions.length === 0}
        options={deviceVersions.map((deviceVersion) => ({
          id: deviceVersion.id,
          label: displayVersionChoice(deviceVersion),
        }))}
        value={selectedDeviceVersionOption}
        inputLabel="Device version"
        onChange={(event, newValue) => {
          const selectedDeviceVersion = deviceVersions.find(
            (deviceVersion) => deviceVersion.id === newValue.id,
          );

          const selectedDeviceVersionId = selectedDeviceVersion?.id;
          if (selectedDeviceVersionId) {
            setSearchParams((searchParams) => {
              searchParams.set(
                DEVICE_VERSION_ID_SEARCH_PARAM,
                selectedDeviceVersionId.toString(),
              );
              return searchParams;
            });
            setInnerSelectedDeviceVersionId(selectedDeviceVersionId);
          }
        }}
      />
    </Box>
  );
};

export default DeviceVersionSearch;
