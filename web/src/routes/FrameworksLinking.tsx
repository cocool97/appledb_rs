import React, { useEffect, useState } from "react";
import { API_URL, GET_ALL_FRAMEWORKS_ENDPOINT } from "../Constants";
import {
  Table,
  TableBody,
  TableCell,
  TableContainer,
  TableHead,
  TableRow,
  Typography,
} from "@mui/material";
import { Executable } from "../types/executables";
import { CustomSearch } from "../components/CustomSearch";
import { BarLoader } from "react-spinners";

const FrameworksLinking = (props) => {
  const { framework_id, operating_system_version_id } = props;

  const [isLoading, setLoading] = useState(false);

  const [executables, setExecutables] = useState<Executable[]>([]);
  const [executableSearch, setExecutableSearch] = useState("");

  useEffect(() => {
    setLoading(true);

    if (!framework_id || !operating_system_version_id) {
      setExecutables([]);
      return;
    }

    fetch(
      `${API_URL}/frameworks/${framework_id}/executables/${operating_system_version_id}`,
    )
      .then((response) => {
        if (!response.ok) {
          throw new Error("Failed to fetch executables");
        }
        return response.json();
      })
      .then((data) => {
        setExecutables(data);
      })
      .catch((error) => {
        console.error(error);
        setExecutables([]);
      })
      .finally(() => setLoading(false));
  }, [framework_id, operating_system_version_id]);

  const filteredExecutables = executables.filter((exec) =>
    exec.full_path.toLowerCase().includes(executableSearch.toLowerCase()),
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
      filteredExecutables.length !== 0 && (
        <TableContainer>
          <Table size="small" sx={{ tableLayout: "fixed" }}>
            <TableHead>
              <TableRow>
                <TableCell>
                  <Typography variant="h6" sx={{ color: "white" }}>
                    Executable full path
                  </Typography>
                </TableCell>
              </TableRow>
            </TableHead>
            <TableBody>
              {filteredExecutables.map((result) => {
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
    <div style={{ display: "flex", flexDirection: "column", gap: "1rem" }}>
      {executables.length > 0 && (
        <CustomSearch
          label="Filter executables"
          value={executableSearch}
          onChange={(e) => setExecutableSearch(e.target.value)}
        />
      )}
      {renderDataTable()}
    </div>
  );
};

export default FrameworksLinking;
