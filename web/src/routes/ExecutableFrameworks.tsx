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
import { Framework } from "../types/framework";

const ExecutableFrameworks = (props) => {
  const { executable_operating_system_id } = props;

  const [results, setResults] = useState<Framework[]>([]);
  const [isLoading, setLoading] = useState(false);

  const [frameworkInput, setFrameworkInput] = useState("");

  useEffect(() => {
    if (!executable_operating_system_id) {
      setResults([]);
      return;
    }

    setLoading(true);

    fetch(`${API_URL}/executable/${executable_operating_system_id}/frameworks`)
      .then((response) => {
        if (!response.ok) {
          throw new Error("Failed to fetch frameworks");
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
    (obj: Framework[]) => {
      const lowerFrameworkInput = frameworkInput.toLowerCase();
      return obj.filter((elem) => {
        return elem.full_path.toLowerCase().includes(lowerFrameworkInput);
      });
    },
    [frameworkInput],
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
                    Framework full path
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
                        {result.full_path}
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
            disabled={Object.keys(results).length === 0}
            label="Filter by framework name"
            value={frameworkInput}
            onChange={(e) => setFrameworkInput(e.target.value)}
          />
        </Box>
      </Box>

      {renderDataTable()}
    </Box>
  );
};

export default ExecutableFrameworks;
