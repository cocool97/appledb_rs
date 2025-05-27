import React, { useCallback, useEffect, useMemo, useState } from "react";
import { BarLoader } from "react-spinners";
import { API_URL } from "../Constants";
import { CustomDataTable } from "../components/CustomDataTable";
import { Box } from "@mui/material";
import { CustomSearch } from "../components/CustomSearch";
import DeviceVersionSearch from "../components/DeviceVersionSearch";

const EntitlementsSearch = () => {
  const [selectedDeviceVersionId, setSelectedDeviceVersionId] = useState<
    number | null
  >(null);

  const [results, setResults] = useState({});
  const [isLoading, setLoading] = useState(false);

  const [executableInput, setExecutableInput] = useState("");
  const [entitlementKeyInput, setEntitlementKeyInput] = useState("");
  const [entitlementValueInput, setEntitlementValueInput] = useState("");

  useEffect(() => {
    if (selectedDeviceVersionId) {
      setLoading(true);
      fetch(
        `${API_URL}/operating_systems/${selectedDeviceVersionId}/executable_entitlements`,
      )
        .then((response) => response.json())
        .then((data) => setResults(data))
        .then(() => setLoading(false))
        .catch((error) => console.log(error));
    }
  }, [selectedDeviceVersionId]);

  const filterObject = useCallback(
    (obj) => {
      const result = {};
      const lowerExecutableInput = executableInput.toLowerCase();
      const lowerEntitlementKey = entitlementKeyInput.toLowerCase();
      const lowerEntitlementValue = entitlementValueInput.toLowerCase();

      for (const [key, value] of Object.entries(obj)) {
        if (!value.name.toLowerCase().includes(lowerExecutableInput)) continue;

        const filteredArray = value.entitlements.filter(
          (item) =>
            item.key.toLowerCase().includes(lowerEntitlementKey) &&
            item.value.toLowerCase().includes(lowerEntitlementValue),
        );

        if (filteredArray.length > 0) {
          result[key] = { entitlements: filteredArray, name: value.name };
        }
      }
      return result;
    },
    [executableInput, entitlementKeyInput, entitlementValueInput],
  );

  const filteredResults = useMemo(
    () => filterObject(results),
    [results, filterObject],
  );

  const renderDataTable = () => {
    if (isLoading) {
      return (
        <div
          style={{
            width: "inherit",
            height: "inherit",
            display: "flex",
            justifyContent: "center",
            alignItems: "center",
            backgroundColor: "inherit",
          }}
        >
          <BarLoader color="white" width="10rem" />
        </div>
      );
    }
    return (
      Object.keys(results).length !== 0 && (
        <CustomDataTable data={filteredResults} />
      )
    );
  };

  return (
    <Box>
      <Box
        style={{
          display: "flex",
          flexDirection: "column",
          justifyContent: "space-around",
          marginBottom: "3rem",
          gap: "1rem",
        }}
      >
        <DeviceVersionSearch
          setSelectedDeviceVersionId={setSelectedDeviceVersionId}
        />

        <Box
          display="flex"
          flexDirection="row"
          gap={4}
          justifyContent="center"
          sx={{ "& > *": { flex: 1 } }}
        >
          <CustomSearch
            disabled={Object.keys(results).length === 0}
            label="Filter by executable name"
            value={executableInput}
            onChange={(e) => setExecutableInput(e.target.value)}
          />
          <CustomSearch
            disabled={Object.keys(results).length === 0}
            label="Filter by entitlement key"
            value={entitlementKeyInput}
            onChange={(e) => setEntitlementKeyInput(e.target.value)}
          />
          <CustomSearch
            disabled={Object.keys(results).length === 0}
            label="Filter by entitlement value"
            value={entitlementValueInput}
            onChange={(e) => setEntitlementValueInput(e.target.value)}
          />
        </Box>
      </Box>

      {renderDataTable()}
    </Box>
  );
};

export default EntitlementsSearch;
