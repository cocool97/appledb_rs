import { useEffect, useState } from "react";
import { API_URL } from "../Constants";
import React from "react";
import CustomAutocomplete from "../components/CustomAutocomplete";
import { Box } from "@mui/material";
import { DeviceVersion } from "../types/device_versions";
import { Framework } from "../types/framework";
import FrameworksLinking from "./FrameworksLinking";
import DeviceVersionSearch from "../components/DeviceVersionSearch";
import { CustomAccordion } from "../components/CustomAccordion";

export const Frameworks = () => {
  const [selectedDeviceVersion, setSelectedDeviceVersion] =
    useState<DeviceVersion | null>(null);

  const [frameworks, setFrameworks] = useState<Framework[]>([]);
  const [selectedFramework, setSelectedFramework] = useState<Framework | null>(
    null,
  );

  useEffect(() => {
    if (selectedDeviceVersion) {
      fetch(
        `${API_URL}/operating_system_versions/${selectedDeviceVersion.id}/frameworks`,
      )
        .then((response) => response.json())
        .then((data) => setFrameworks(data))
        .catch((error) => console.log(error));
    }
  }, [selectedDeviceVersion]);

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
          setSelectedDeviceVersion={setSelectedDeviceVersion}
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
      </Box>

      {selectedDeviceVersion && selectedFramework && (
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
                    framework_id={selectedFramework.id}
                    operating_system_version_id={selectedDeviceVersion.id}
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
