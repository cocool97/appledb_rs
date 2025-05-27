import { useEffect, useMemo, useState } from "react";
import { API_URL, FRAMEWORK_ID_SEARCH_PARAM } from "../Constants";
import React from "react";
import CustomAutocomplete from "../components/CustomAutocomplete";
import { Box } from "@mui/material";
import { Framework } from "../types/framework";
import FrameworksLinking from "./FrameworksLinking";
import DeviceVersionSearch from "../components/DeviceVersionSearch";
import { CustomAccordion } from "../components/CustomAccordion";
import { useSearchParams } from "react-router-dom";

export const Frameworks = () => {
  const [searchParams, setSearchParams] = useSearchParams();
  const [selectedDeviceVersionId, setSelectedDeviceVersionId] = useState<
    number | null
  >(null);

  const [frameworks, setFrameworks] = useState<Framework[]>([]);

  const framework_id = searchParams.get(FRAMEWORK_ID_SEARCH_PARAM);
  const [selectedFrameworkId, setSelectedFrameworkId] = useState<number | null>(
    framework_id ? parseInt(framework_id) : null,
  );

  useEffect(() => {
    if (selectedDeviceVersionId) {
      fetch(
        `${API_URL}/operating_system_versions/${selectedDeviceVersionId}/frameworks`,
      )
        .then((response) => response.json())
        .then((data) => setFrameworks(data))
        .catch((error) => console.log(error));
    }
  }, [selectedDeviceVersionId]);

  const selectedFrameworkOption = useMemo(() => {
    const framework = frameworks.find((f) => f.id === selectedFrameworkId);
    return framework
      ? {
          id: framework.id,
          label: framework.full_path,
        }
      : null;
  }, [frameworks, selectedFrameworkId]);

  return (
    <div style={{ display: "flex", flexDirection: "column" }}>
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
          disabled={frameworks.length === 0}
          options={frameworks.map((framework) => framework.full_path)}
          inputLabel="Available frameworks"
          value={selectedFrameworkOption}
          onChange={(event, newValue) => {
            const selectedFramework = frameworks.find(
              (framework) => framework.full_path === newValue,
            );
            const selectedFrameworkId = selectedFramework?.id;

            if (selectedFrameworkId) {
              setSearchParams((searchParams) => {
                searchParams.set(
                  FRAMEWORK_ID_SEARCH_PARAM,
                  selectedFrameworkId.toString(),
                );
                return searchParams;
              });
              setSelectedFrameworkId(selectedFrameworkId);
            }
          }}
        />
      </Box>

      {selectedDeviceVersionId && selectedFrameworkId && (
        <Box
          sx={{
            display: "flex",
            flexDirection: "column",
          }}
        >
          <CustomAccordion
            members={[
              {
                title: "Linking executables",
                summary: "What executables are using this framework ?",
                component: (
                  <FrameworksLinking
                    framework_id={selectedFrameworkId}
                    operating_system_version_id={selectedDeviceVersionId}
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
