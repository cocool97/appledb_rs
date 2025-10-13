import React, { useCallback, useEffect, useMemo, useState } from "react";
import { BarLoader } from "react-spinners";
import {
  API_URL,
  DEVICE_ID_SEARCH_PARAM,
  DEVICE_VERSION_ID_SEARCH_PARAM,
} from "../Constants";
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
import { Link, useSearchParams } from "react-router-dom";

const ExecutableFrameworks = (props) => {
  const { executable_operating_system_id } = props;

  const [searchParams] = useSearchParams();

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

  const getFrameworkLink = (framework_id: number) => {
    const selectedDeviceIdParam = searchParams.get(DEVICE_ID_SEARCH_PARAM);

    const selectedDeviceVersionIdParam = searchParams.get(
      DEVICE_VERSION_ID_SEARCH_PARAM,
    );

    if (!selectedDeviceIdParam || !selectedDeviceVersionIdParam) {
      return null;
    }

    return `/frameworks?device_id=${selectedDeviceIdParam}&device_version_id=${selectedDeviceVersionIdParam}&framework_id=${framework_id}`;
  };

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

    if (results.length !== 0) {
      return (
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
                const link = getFrameworkLink(result.id);

                const text = (
                  <Typography sx={{ color: "white" }}>
                    {result.full_path}
                  </Typography>
                );

                const component = link ? <Link to={link}>{text}</Link> : text;

                return (
                  <TableRow key={result.id}>
                    <TableCell>{component}</TableCell>
                  </TableRow>
                );
              })}
            </TableBody>
          </Table>
        </TableContainer>
      );
    } else {
      return <Box sx={{ textAlign: "center" }}>No data available...</Box>;
    }
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
