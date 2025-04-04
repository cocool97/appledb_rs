import { useEffect, useState, useRef } from "react"
import { API_URL, GET_ALL_EXECUTABLES_ENDPOINT } from "../Constants";
import { Autocomplete, List, ListItem, ListItemText, TextField } from "@mui/material";
import CustomSelect from "../components/Select";
import React from "react";

const DiffResults = (props) => {
    const { diff } = props;

    return (
        <div style={{ display: "flex" }}>
            {
                diff && (
                    <>
                        <DiffResult header="Unchanged" color="orange" result={diff.unchanged} />
                        <DiffResult header="Added" color="green" result={diff.added} />
                        <DiffResult header="Removed" color="red" result={diff.removed} />
                    </>
                )
            }
        </div>
    )
}

const DiffResult = (props) => {
    const { header, result, color } = props;
    return (
        <div style={{ display: "flex", flex: 1, color: "white", flexDirection: "column" }}>
            <div style={{ textAlign: "center", fontWeight: "bold", fontSize: "1.5rem", marginBottom: "2rem", color: color }}>
                {header}
            </div>
            <List>
                {result.map((result) => {
                    return (
                        <ListItem key={result.id}>
                            <ListItemText sx={{ display: "flex", flex: 1, textAlign: "center" }}>{result.key}</ListItemText>
                            <ListItemText sx={{ display: "flex", flex: 1, textAlign: "center", justifyContent: "center" }}>{result.value}</ListItemText>
                        </ListItem>
                    )
                })}

            </List>
        </div>
    )
}

const Diffing = () => {
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
            fetch(`${API_URL}/api/v1/executables/${executable.id}/versions`)
                .then((response) => response.json())
                .then((data) => setExecutableVersions(data))
                .catch((error) => console.log(error));
        }
    }, [executable]);

    useEffect(() => {
        if (from && to) {
            fetch(`${API_URL}/api/v1/entitlements/diff/${from}/${to}`)
                .then((response) => response.json())
                .then((data) => setDiff(data))
                .catch((error) => console.log(error));
        }
    }, [from, to]);

    const displayVersionChoice = (version) => {
        return (version.display_name ?? "Unknown") + " - " + version.model_code + " - " + version.version
    }

    const versionIDGetter = (version) => {
        return version.id
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
                options={executables.map((executable) => { return executable.full_path })}
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

export default Diffing;