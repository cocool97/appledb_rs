import React, { useCallback, useEffect, useMemo, useState } from "react";
import { BarLoader } from "react-spinners";
import { API_URL, GET_ALL_DEVICES_ENDPOINT } from "../Constants";
import {
  Box,
  Table,
  TableBody,
  TableCell,
  TableContainer,
  TableHead,
  TableRow,
  Typography,
} from "@mui/material";
import { CustomSearch } from "../components/CustomSearch";
import { Entitlement } from "../types/entitlement";

const ExecutableEntitlements = (props) => {
  const { executable_operating_system_id } = props;
  const [results, setResults] = useState<Entitlement[]>([]);
  const [isLoading, setLoading] = useState(false);

  const [entitlementKeyInput, setEntitlementKeyInput] = useState("");
  const [entitlementValueInput, setEntitlementValueInput] = useState("");

  useEffect(() => {
    if (!executable_operating_system_id) {
      setResults([]);
      return;
    }

    setLoading(true);

    fetch(
      `${API_URL}/executable/${executable_operating_system_id}/entitlements`,
    )
      .then((response) => {
        if (!response.ok) {
          throw new Error("Failed to fetch entitlements");
        }
        return response.json();
      })
      .then((data) => {
        setResults(data);
      })
      .catch((error) => {
        console.error(error);
        setResults([]);
      })
      .finally(() => setLoading(false));
  }, [executable_operating_system_id]);

  const filterObject = useCallback(
    (obj: Entitlement[]) => {
      const lowerEntitlementKey = entitlementKeyInput.toLowerCase();
      const lowerEntitlementValue = entitlementValueInput.toLowerCase();
      return obj.filter((elem) => {
        return (
          elem.key.toLowerCase().includes(lowerEntitlementKey) &&
          elem.value.toLowerCase().includes(lowerEntitlementValue)
        );
      });
    },
    [entitlementKeyInput, entitlementValueInput],
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
      results.length !== 0 && (
        <TableContainer>
          <Table size="small" sx={{ tableLayout: "fixed" }}>
            <TableHead>
              <TableRow>
                <TableCell>
                  <Typography variant="h6" sx={{ color: "white" }}>
                    Entitlement key
                  </Typography>
                </TableCell>
                <TableCell>
                  <Typography variant="h6" sx={{ color: "white" }}>
                    Entitlement value
                  </Typography>
                </TableCell>
              </TableRow>
            </TableHead>
            <TableBody>
              {filteredResults.map((result) => {
                return (
                  <TableRow key={result.id}>
                    <TableCell>
                      <Typography sx={{ color: "white" }}>
                        {result.key}
                      </Typography>
                    </TableCell>
                    <TableCell>
                      <Typography sx={{ color: "white" }}>
                        {result.value}
                      </Typography>
                    </TableCell>
                  </TableRow>
                );
              })}
            </TableBody>
          </Table>
        </TableContainer>
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
          marginBottom: "2rem",
        }}
      >
        <Box
          display="flex"
          flexDirection="row"
          gap={4}
          justifyContent="center"
          sx={{ "& > *": { flex: 1 } }}
        >
          <CustomSearch
            disabled={results.length === 0}
            label="Filter by entitlement key"
            value={entitlementKeyInput}
            onChange={(e) => setEntitlementKeyInput(e.target.value)}
          />
          <CustomSearch
            disabled={results.length === 0}
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

export default ExecutableEntitlements;
