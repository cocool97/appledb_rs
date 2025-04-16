import React, { useEffect, useRef, useState } from "react"
import { API_URL, GET_ALL_EXECUTABLES_ENDPOINT } from "../Constants";
import { Autocomplete, Table, TableBody, TableContainer, TextField } from "@mui/material";
import CustomSelect from "../components/CustomSelect";
import { ExpandableTableRow } from "../components/CustomDataTable";

const DiffResults = (props) => {
    const { diff } = props;

    return (
        <div style={{ display: "flex" }}>
            {
                diff && (
                    <TableContainer>
                        <Table size="small" sx={{ tableLayout: "fixed" }}>
                            <TableBody>
                                <ExpandableTableRow
                                    label="Added"
                                    mainCellLabel="Entitlement name"
                                    mainCellLabelGetter={(item) => item.key}
                                    secondaryCellLabel="Entitlement value"
                                    secondaryCellLabelGetter={(item) => item.value}
                                    items={diff.added}
                                />
                                <ExpandableTableRow
                                    label="Removed"
                                    mainCellLabel="Entitlement name"
                                    mainCellLabelGetter={(item) => item.key}
                                    secondaryCellLabel="Entitlement value"
                                    secondaryCellLabelGetter={(item) => item.value}
                                    items={diff.removed}
                                />
                                <ExpandableTableRow
                                    label="Unchanged"
                                    mainCellLabel="Entitlement name"
                                    mainCellLabelGetter={(item) => item.key}
                                    secondaryCellLabel="Entitlement value"
                                    secondaryCellLabelGetter={(item) => item.value}
                                    items={diff.unchanged}
                                />
                            </TableBody>
                        </Table>
                    </TableContainer>
                )
            }
        </div>
    )
}

const EntitlementsDiffing = () => {
    const [executables, setExecutables] = useState([]);

    const [executable, setExecutable] = useState(null);
    const [executableVersions, setExecutableVersions] = useState([]);
    const prevExecutable = useRef(null);

    const [from, setFrom] = useState(null);
    const [to, setTo] = useState(null);

    const [diff, setDiff] = useState(null);

    useEffect(() => {
        fetch(GET_ALL_EXECUTABLES_ENDPOINT)
            .then((response) => response.json())
            .then((data) => setExecutables(data))
            .catch((error) => console.log(error));
    }, []);

    useEffect(() => {
        if (executable && executable !== prevExecutable.current) {
            fetch(`${API_URL}/executables/${executable.id}/versions`)
                .then((response) => response.json())
                .then((data) => setExecutableVersions(data))
                .catch((error) => console.log(error));
        }
    }, [executable]);

    useEffect(() => {
        if (from && to) {
            fetch(`${API_URL}/entitlements/diff/${from}/${to}`)
                .then((response) => response.json())
                .then((data) => setDiff(data))
                .catch((error) => console.log(error));
        }
    }, [from, to]);

    const displayVersionChoice = (version) => (version.display_name ?? "Unknown") + " - " + version.model_code + " - " + version.version

    const versionIDGetter = (version) => version.id

    return (
        <div style={{ display: "flex", flexDirection: "column", gap: "1rem" }}>
            <Autocomplete
                fullWidth
                disablePortal
                sx={{   // magic...
                    "& + .MuiAutocomplete-popper .MuiAutocomplete-option": {
                        backgroundColor: "transparent",
                        color: "black"
                    },
                    "& + .MuiAutocomplete-popper .MuiAutocomplete-option[aria-selected='true']":
                    {
                        backgroundColor: "transparent",
                        color: "black"
                    },
                    "& + .MuiAutocomplete-popper .MuiAutocomplete-option[aria-selected ='true'] .Mui-focused":
                    {
                        backgroundColor: "transparent",
                        color: "black"
                    },
                }}
                options={executables.map((executable) => executable.full_path)}
                renderInput={(params) => <TextField sx={{ label: { color: 'white' }, input: { color: "white !important" }, "Mui-expanded": { color: "red" } }} key={params.full_path} {...params} label="Executable" />}
                onChange={(event, newValue) => {
                    const selectedExecutable = executables.find(exec => exec.full_path === newValue);
                    setExecutable(selectedExecutable || null);
                }}
            />

            <CustomSelect
                label="From"
                onChange={setFrom}
                choices={executableVersions}
                labelDisplayFunc={displayVersionChoice}
                idGetter={versionIDGetter}
            />

            <CustomSelect
                label="To"
                onChange={setTo}
                choices={executableVersions}
                labelDisplayFunc={displayVersionChoice}
                idGetter={versionIDGetter}
            />

            <DiffResults diff={diff} />
        </div>
    )
}

export default EntitlementsDiffing;