import React, { useEffect, useRef, useState } from "react"
import { API_URL, GET_ALL_EXECUTABLES_ENDPOINT } from "../Constants";
import CustomAutocomplete from "../components/CustomAutocomplete";
import { Typography } from "@mui/material";
import { Executable } from "../types/executables";
import { Version } from "../types/versions";
import { Framework } from "../types/framework";
import { CustomSearch } from "../components/CustomSearch";

const ExecutablesFrameworks = () => {
    const [executables, setExecutables] = useState<Executable[]>([]);
    const [executable, setExecutable] = useState<Executable | null>(null);
    const [executableVersions, setExecutableVersions] = useState<Version[]>([]);
    const prevExecutable = useRef<Executable | null>(null);
    const [versionID, setVersionID] = useState<number | null>(null);
    const [frameworks, setFramworks] = useState<Framework[]>([]);
    const [frameworkSearch, setFrameworkSearch] = useState("");

    useEffect(() => {
        fetch(GET_ALL_EXECUTABLES_ENDPOINT)
            .then((response) => response.json())
            .then((data) => setExecutables(data))
            .catch((error) => console.log(error));
    }, []);

    useEffect(() => {
        if (executable && executable !== prevExecutable.current) {
            prevExecutable.current = executable;
            setExecutableVersions([]);
            setVersionID(null);
            setFramworks([]);

            fetch(`${API_URL}/executables/${executable.id}/versions`)
                .then((response) => response.json())
                .then((data) => setExecutableVersions(data))
                .catch((error) => console.log(error));
        } else if (!executable) {
            prevExecutable.current = null;
            setExecutableVersions([]);
            setVersionID(null);
            setFramworks([]);
        }
    }, [executable]);

    useEffect(() => {
        if (versionID) {
            setFramworks([]);
            fetch(`${API_URL}/executables/${versionID}/frameworks`)
                .then((response) => response.json())
                .then((data) => setFramworks(data))
                .catch((error) => console.log(error));
        } else {
            setFramworks([]);
        }
    }, [versionID]);

    const displayVersionChoice = (version) => (version.display_name ?? "Unknown") + " - " + version.model_code + " - " + version.version;

    const filteredFrameworks = frameworks.filter(fw =>
        fw.full_path.toLowerCase().includes(frameworkSearch.toLowerCase())
    );

    return (
        <div style={{ display: "flex", flexDirection: "column", gap: "1rem" }}>
            <CustomAutocomplete
                options={executables.map((executable) => executable.full_path)}
                inputLabel="Executable"
                value={executable?.full_path || null}
                onChange={(event, newValue) => {
                    const selectedExecutable = executables.find(exec => exec.full_path === newValue);
                    setExecutable(selectedExecutable || null);
                }}
            />
            <CustomAutocomplete
                disabled={executableVersions.length === 0}
                options={executableVersions.map((version) => displayVersionChoice(version))}
                inputLabel="Available version"
                value={versionID ? displayVersionChoice(executableVersions.find(v => v.id === versionID)) : null}
                onChange={(event, newValue) => {
                    const selectedVersion = executableVersions.find(v => displayVersionChoice(v) === newValue);
                    setVersionID(selectedVersion?.id || null);
                }}
            />
            {frameworks.length > 0 && (
                <CustomSearch
                    label="Filter frameworks"
                    value={frameworkSearch}
                    onChange={(e) => setFrameworkSearch(e.target.value)}
                />
            )}
            {filteredFrameworks.map((framework, index) => (
                <Typography key={index}>{framework.full_path}</Typography>
            ))}
        </div>
    );
};

export default ExecutablesFrameworks;
