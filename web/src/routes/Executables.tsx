import { useEffect, useState } from "react";
import { API_URL } from "../Constants";
import React from "react";
import CustomAutocomplete from "../components/CustomAutocomplete";
import {
  Accordion,
  AccordionDetails,
  AccordionSummary,
  Box,
  Typography,
} from "@mui/material";
import { useSearchParams } from "react-router-dom";
import { ExecutableOperatingSystem } from "../types/executable_operating_system";
import { DeviceVersion } from "../types/device_versions";
import ExecutableEntitlements from "./ExecutableEntitlements";
import ExecutableFrameworks from "./ExecutableFrameworks";
import DeviceVersionSearch from "../components/DeviceVersionSearch";
import { CustomAccordion } from "../components/CustomAccordion";

const EXECUTABLE_ID_SEARCH_PARAM = "executable_id";

export const Executables = () => {
  const [expanded, setExpanded] = React.useState("");

  const handleChange = (panel) => (event, isExpanded) => {
    setExpanded(isExpanded ? panel : false);
  };

  const [searchParams, setSearchParams] = useSearchParams();

  const [selectedDeviceVersion, setSelectedDeviceVersion] =
    useState<DeviceVersion | null>(null);

  const [executables, setExecutables] = useState<ExecutableOperatingSystem[]>(
    [],
  );

  const executable_id = searchParams.get(EXECUTABLE_ID_SEARCH_PARAM);
  const [selectedExecutableId, setSelectedExecutableId] = useState<
    number | null
  >(executable_id ? parseInt(executable_id) : null);

  useEffect(() => {
    if (selectedDeviceVersion) {
      fetch(
        `${API_URL}/operating_system_versions/${selectedDeviceVersion.id}/executables`,
      )
        .then((response) => response.json())
        .then((data) => setExecutables(data))
        .catch((error) => console.log(error));
    }
  }, [selectedDeviceVersion]);

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
          setSelectedDeviceVersion={setSelectedDeviceVersion}
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
