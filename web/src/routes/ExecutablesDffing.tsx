import { useEffect, useState } from "react"
import { API_URL, GET_EXTENDED_OPERATING_SYSTEM_VERSIONS } from "../Constants";
import { TextField } from "@mui/material";
import React from "react";
import CustomAutocomplete from "../components/CustomAutocomplete";
import DiffResults from "../components/DiffResults";
import { Diff } from "../types/diff";


const ExecutablesDiffing = () => {
    const [devices, setDevices] = useState([]);

    const [deviceFrom, setDeviceFrom] = useState(null);
    const [deviceTo, setDeviceTo] = useState(null);
    const [diff, setDiff] = useState<Diff | null>(null);

    useEffect(() => {
        fetch(GET_EXTENDED_OPERATING_SYSTEM_VERSIONS)
            .then((response) => response.json())
            .then((data) => setDevices(data))
            .catch((error) => console.log(error));
    }, []);

    useEffect(() => {
        if (deviceFrom && deviceTo) {
            fetch(`${API_URL}/executables/diff/${deviceFrom.id}/${deviceTo.id}`)
                .then((response) => response.json())
                .then((data) => setDiff(data))
                .catch((error) => console.log(error));
        }
    }, [deviceFrom, deviceTo]);

    const displayVersionChoice = (version) => (version.display_name ?? "Unknown") + " - " + version.model_code + " - " + version.version

    return (
        <div style={{ display: "flex", flexDirection: "column", gap: "1rem" }}>
            <CustomAutocomplete
                options={devices.map((device) => ({ id: device.id, label: displayVersionChoice(device) }))}
                inputLabel="From version"
                onChange={(event, newValue) => {
                    const selectedDevice = devices.find(device => device.id === newValue.id);
                    setDeviceFrom(selectedDevice || null);
                }}
            />

            <CustomAutocomplete
                options={devices.map((device) => ({ id: device.id, label: displayVersionChoice(device) }))}
                inputLabel="To version"
                onChange={(event, newValue) => {
                    const selectedDevice = devices.find(device => device.id === newValue.id);
                    setDeviceTo(selectedDevice || null);
                }}
            />

            <DiffResults
                diff={diff}
                mainCellLabel="Executable name"
                secondaryCellLabel="Executable full path"
                mainCellLabelGetter={(item) => item.name}
                secondaryCellLabelGetter={(item) => item.full_path}
            />
        </div>
    )
}

export default ExecutablesDiffing;