import React, { useEffect, useMemo, useState } from "react";
import { API_URL, EXECUTABLE_ID_SEARCH_PARAM } from "../Constants";
import CustomAutocomplete from "../components/CustomAutocomplete";
import { Box } from "@mui/material";
import { useSearchParams } from "react-router-dom";
import { ExecutableOperatingSystem } from "../types/executable_operating_system";
import ExecutableEntitlements from "./ExecutableEntitlements";
import ExecutableFrameworks from "./ExecutableFrameworks";
import DeviceVersionSearch from "../components/DeviceVersionSearch";
import { CustomAccordion } from "../components/CustomAccordion";

export const Executables = () => {
  const [searchParams, setSearchParams] = useSearchParams();

  const [selectedDeviceVersionId, setSelectedDeviceVersionId] = useState<
    number | null
  >(null);

  const [executables, setExecutables] = useState<ExecutableOperatingSystem[]>(
    [],
  );

  const executable_id = searchParams.get(EXECUTABLE_ID_SEARCH_PARAM);
  const [selectedExecutableId, setSelectedExecutableId] = useState<
    number | null
  >(executable_id ? parseInt(executable_id) : null);

  const selectedExecutableOption = useMemo(() => {
    const executable = executables.find(
      (e) => e.executable_operating_system_id === selectedExecutableId,
    );
    return executable
      ? {
          id: executable.executable_operating_system_id,
          label: executable.full_path,
        }
      : null;
  }, [executables, selectedExecutableId]);

  useEffect(() => {
    if (selectedDeviceVersionId) {
      fetch(
        `${API_URL}/operating_system_versions/${selectedDeviceVersionId}/executables`,
      )
        .then((response) => response.json())
        .then((data) => setExecutables(data))
        .catch((error) => console.log(error));
    }
  }, [selectedDeviceVersionId]);

  return (
    <div
      style={{
        display: "flex",
        flexDirection: "column",
      }}
    >
      <Box
        sx={{
          marginBottom: "3rem",
          display: "flex",
          flexDirection: "column",
          gap: "1rem",
        }}
      >
        <DeviceVersionSearch
          setSelectedDeviceVersionId={setSelectedDeviceVersionId}
        />

        <CustomAutocomplete
          disabled={executables.length === 0}
          options={executables.map((executable) => executable.full_path)}
          inputLabel="Available executables"
          value={selectedExecutableOption}
          onChange={(event, newValue) => {
            const selectedExecutable = executables.find(
              (executables) => executables.full_path === newValue,
            );
            const selectedExecutableId =
              selectedExecutable?.executable_operating_system_id;

            if (selectedExecutableId) {
              setSelectedExecutableId(selectedExecutableId ?? null);
              setSearchParams((searchParams) => {
                searchParams.set(
                  EXECUTABLE_ID_SEARCH_PARAM,
                  selectedExecutableId.toString(),
                );
                return searchParams;
              });
            }
          }}
        />
      </Box>

      {selectedExecutableId && (
        <Box
          sx={{
            display: "flex",
            flexDirection: "column",
          }}
        >
          <CustomAccordion
            members={[
              {
                title: "Entitlements",
                summary: "What entitlements does this executable have ?",
                component: (
                  <ExecutableEntitlements
                    executable_operating_system_id={selectedExecutableId}
                  />
                ),
              },
              {
                title: "Frameworks",
                summary: "What frameworks does this executable depend on?",
                component: (
                  <ExecutableFrameworks
                    executable_operating_system_id={selectedExecutableId}
                  />
                ),
              },
            ]}
          />
        </Box>
      )}
    </div>
  );
};
