import { useEffect, useState } from "react"
import { API_URL, GET_EXTENDED_OPERATING_SYSTEM_VERSIONS } from "../Constants";
import { Autocomplete, Table, TableBody, TableContainer, TextField } from "@mui/material";
import React from "react";
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
                                    mainCellLabel="Executable name"
                                    mainCellLabelGetter={(item) => item.name}
                                    secondaryCellLabel="Executable full path"
                                    secondaryCellLabelGetter={(item) => item.full_path}
                                    items={diff.added}
                                />
                                <ExpandableTableRow
                                    label="Removed"
                                    mainCellLabel="Executable name"
                                    mainCellLabelGetter={(item) => item.name}
                                    secondaryCellLabel="Executable full path"
                                    secondaryCellLabelGetter={(item) => item.full_path}
                                    items={diff.removed}
                                />
                                <ExpandableTableRow
                                    label="Unchanged"
                                    mainCellLabel="Executable name"
                                    mainCellLabelGetter={(item) => item.name}
                                    secondaryCellLabel="Executable full path"
                                    secondaryCellLabelGetter={(item) => item.full_path}
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

const ExecutablesDiffing = () => {
    const [devices, setDevices] = useState([]);

    const [deviceFrom, setDeviceFrom] = useState(null);
    const [deviceTo, setDeviceTo] = useState(null);
    const [diff, setDiff] = useState(null);

    useEffect(() => {
        fetch(GET_EXTENDED_OPERATING_SYSTEM_VERSIONS)
            .then((response) => response.json())
            .then((data) => setDevices(data))
            .catch((error) => console.log(error));
    }, []);

    useEffect(() => {
        if (deviceFrom && deviceTo) {
            fetch(`${API_URL}/api/v1/executables/diff/${deviceFrom.id}/${deviceTo.id}`)
                .then((response) => response.json())
                .then((data) => setDiff(data))
                .catch((error) => console.log(error));
        }
    }, [deviceFrom, deviceTo]);

    const displayVersionChoice = (version) => {
        return (version.display_name ?? "Unknown") + " - " + version.model_code + " - " + version.version
    }

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
                options={devices.map((device) => { return { id: device.id, label: displayVersionChoice(device) } })}
                renderInput={(params) => <TextField sx={{ label: { color: 'white' }, input: { color: "white !important" }, "Mui-expanded": { color: "red" } }} key={params.id} {...params} label="From version" />}
                onChange={(event, newValue) => {
                    const selectedDevice = devices.find(device => device.id === newValue.id);
                    setDeviceFrom(selectedDevice || null);
                }}
            />

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
                options={devices.map((device) => { return { id: device.id, label: displayVersionChoice(device) } })}
                renderInput={(params) => <TextField sx={{ label: { color: 'white' }, input: { color: "white !important" }, "Mui-expanded": { color: "red" } }} key={params.id} {...params} label="To version" />}
                onChange={(event, newValue) => {
                    const selectedDevice = devices.find(device => device.id === newValue.id);
                    setDeviceTo(selectedDevice || null);
                }}
            />

            <DiffResults diff={diff} />
        </div>
    )
}

export default ExecutablesDiffing;