import { API_URL, GET_ALL_EXECUTABLES_ENDPOINT } from "../Constants";
import React, { useEffect, useRef, useState } from "react"
import CustomAutocomplete from "../components/CustomAutocomplete";
import DiffResults from "../components/DiffResults";
import { Diff } from "../types/diff";


const FrameworksDiffing = () => {
    const [executables, setExecutables] = useState([]);

    const [executable, setExecutable] = useState(null);
    const [executableVersions, setExecutableVersions] = useState([]);
    const prevExecutable = useRef(null);

    const [from, setFrom] = useState(null);
    const [to, setTo] = useState(null);

    const [diff, setDiff] = useState<Diff | null>(null);

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
            fetch(`${API_URL}/frameworks/diff/${from}/${to}`)
                .then((response) => response.json())
                .then((data) => setDiff(data))
                .catch((error) => console.log(error));
        }
    }, [from, to]);

    const displayVersionChoice = (version) => (version.display_name ?? "Unknown") + " - " + version.model_code + " - " + version.version

    return (
        <div style={{ display: "flex", flexDirection: "column", gap: "1rem" }}>
            <CustomAutocomplete
                options={executables.map((executable) => executable.full_path)}
                inputLabel="Executable"
                onChange={(event, newValue) => {
                    const selectedExecutable = executables.find(exec => exec.full_path === newValue);
                    setExecutable(selectedExecutable || null);
                }}
            />

            <CustomAutocomplete
                disabled={executableVersions.length === 0}
                options={executableVersions.map((executableVersion) => displayVersionChoice(executableVersion))}
                inputLabel="From version"
                onChange={(event, newValue) => {
                    const selectedVersion = executableVersions.find(version => displayVersionChoice(version) === newValue);
                    setFrom(selectedVersion.id || null);
                }}
            />

            <CustomAutocomplete
                disabled={executableVersions.length === 0}
                options={executableVersions.map((executableVersion) => displayVersionChoice(executableVersion))}
                inputLabel="To version"
                onChange={(event, newValue) => {
                    const selectedVersion = executableVersions.find(version => displayVersionChoice(version) === newValue);
                    setTo(selectedVersion.id || null);
                }}
            />

            <DiffResults
                diff={diff}
                mainCellLabel="Framework name"
                mainCellLabelGetter={(item) => item.full_path}
            />

        </div>
    )
}

export default FrameworksDiffing;