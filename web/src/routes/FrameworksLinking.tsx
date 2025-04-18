import React, { useEffect, useRef, useState } from "react"
import { API_URL, GET_ALL_FRAMEWORKS_ENDPOINT } from "../Constants";
import CustomAutocomplete from "../components/CustomAutocomplete";
import { Typography } from "@mui/material";
import { Executable } from "../types/executables";
import { Version } from "../types/versions";
import { Framework } from "../types/framework";
import { CustomSearch } from "../components/CustomSearch";

const FrameworksLinking = () => {
    const [frameworks, setFrameworks] = useState<Framework[]>([]);
    const [framework, setFramework] = useState<Framework | null>(null);
    const prevFramework = useRef<Framework | null>(null);

    const [frameworkVersions, setFrameworkVersions] = useState<Version[]>([]);

    const [versionID, setVersionID] = useState<number | null>(null);
    const [executables, setExecutables] = useState<Executable[]>([]);
    const [executableSearch, setExecutableSearch] = useState("");

    useEffect(() => {
        fetch(GET_ALL_FRAMEWORKS_ENDPOINT)
            .then((response) => response.json())
            .then((data) => setFrameworks(data))
            .catch((error) => console.log(error));
    }, []);

    useEffect(() => {
        if (framework && framework !== prevFramework.current) {
            prevFramework.current = framework;
            setFrameworkVersions([]);
            setVersionID(null);
            setExecutables([]);

            fetch(`${API_URL}/frameworks/${framework.id}/versions`)
                .then((response) => response.json())
                .then((data) => setFrameworkVersions(data))
                .catch((error) => console.log(error));
        } else if (!framework) {
            prevFramework.current = null;
            setFrameworkVersions([]);
            setVersionID(null);
            setExecutables([]);
        }
    }, [framework]);

    useEffect(() => {
        if (framework && versionID) {
            setExecutables([]);
            fetch(`${API_URL}/frameworks/${framework.id}/executables/${versionID}`)
                .then((response) => response.json())
                .then((data) => setExecutables(data))
                .catch((error) => console.log(error));
        } else {
            setExecutables([]);
        }
    }, [versionID]);

    const displayVersionChoice = (version) => (version.display_name ?? "Unknown") + " - " + version.model_code + " - " + version.version;

    const filteredExecutables = executables.filter(exec =>
        exec.full_path.toLowerCase().includes(executableSearch.toLowerCase())
    );

    return (
        <div style={{ display: "flex", flexDirection: "column", gap: "1rem" }}>
            <CustomAutocomplete
                options={frameworks.map((framework) => framework.full_path)}
                inputLabel="Framework"
                value={framework?.full_path || null}
                onChange={(event, newValue) => {
                    const selectedFramework = frameworks.find(framework => framework.full_path === newValue);
                    setFramework(selectedFramework || null);
                }}
            />
            <CustomAutocomplete
                disabled={frameworkVersions.length === 0}
                options={frameworkVersions.map((version) => displayVersionChoice(version))}
                inputLabel="Available version"
                value={versionID ? displayVersionChoice(frameworkVersions.find(v => v.id === versionID)) : null}
                onChange={(event, newValue) => {
                    const selectedVersion = frameworkVersions.find(v => displayVersionChoice(v) === newValue);
                    setVersionID(selectedVersion?.id || null);
                }}
            />
            {executables.length > 0 && (
                <CustomSearch
                    label="Filter frameworks"
                    value={executableSearch}
                    onChange={(e) => setExecutableSearch(e.target.value)}
                />
            )}
            {filteredExecutables.map((executable, index) => (
                <Typography key={index}>{executable.full_path}</Typography>
            ))}
        </div>
    );
};

export default FrameworksLinking;
