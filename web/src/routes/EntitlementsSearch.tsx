import React, { useCallback, useEffect, useMemo, useState } from 'react';
import { BarLoader } from "react-spinners";
import { API_URL, GET_ALL_DEVICES_ENDPOINT } from "../Constants";
import { CustomDataTable } from '../components/CustomDataTable';
import "./EntitlementSearch.css";
import { Box } from '@mui/material';
import CustomAutocomplete from '../components/CustomAutocomplete';
import { CustomSearch } from '../components/CustomSearch';

const EntitlementsSearch = () => {
    const [models, setModels] = useState([]);

    const [chosenModel, setChosenModel] = useState(null);

    const [versions, setVersions] = useState([]);
    const [chosenVersion, setChosenVersion] = useState(null);

    const [results, setResults] = useState({});
    const [isLoading, setLoading] = useState(false);

    const [executableInput, setExecutableInput] = useState('');
    const [entitlementKeyInput, setEntitlementKeyInput] = useState('');
    const [entitlementValueInput, setEntitlementValueInput] = useState('');

    const displayModelChoice = (model) => (model.display_name ?? "Unknown") + " - " + model.model_code

    useEffect(() => {
        fetch(GET_ALL_DEVICES_ENDPOINT)
            .then((response) => response.json())
            .then((data) => setModels(data))
            .catch((error) => console.log(error));
    }, []);

    useEffect(() => {
        if (chosenModel) {
            fetch(`${API_URL}/devices/${chosenModel.id}/operating_system_versions`)
                .then((response) => response.json())
                .then((data) => setVersions(data))
                .catch((error) => console.log(error));
        }
    }, [chosenModel]);

    useEffect(() => {
        if (chosenVersion) {
            setLoading(true)
            fetch(`${API_URL}/operating_systems/${chosenVersion.id}/executable_entitlements`)
                .then((response) => response.json())
                .then((data) => setResults(data))
                .then(() => setLoading(false))
                .catch((error) => console.log(error));
        }
    }, [chosenVersion]);

    const filterObject = useCallback((obj) => {
        const result = {};
        const lowerExecutableInput = executableInput.toLowerCase();
        const lowerEntitlementKey = entitlementKeyInput.toLowerCase();
        const lowerEntitlementValue = entitlementValueInput.toLowerCase();

        for (const [key, value] of Object.entries(obj)) {
            if (!value.name.toLowerCase().includes(lowerExecutableInput)) continue;

            const filteredArray = value.entitlements.filter(item =>
                item.key.toLowerCase().includes(lowerEntitlementKey) &&
                item.value.toLowerCase().includes(lowerEntitlementValue)
            );

            if (filteredArray.length > 0) {
                result[key] = { entitlements: filteredArray, name: value.name };
            }
        }
        return result;
    }, [executableInput, entitlementKeyInput, entitlementValueInput]);

    const filteredResults = useMemo(() => filterObject(results), [results, filterObject]);

    const renderDataTable = () => {
        if (isLoading) {
            return (
                <div style={{ width: "inherit", height: "inherit", display: "flex", justifyContent: "center", alignItems: "center", backgroundColor: "inherit" }}>
                    <BarLoader color="white" width="10rem" />
                </div>
            );
        }
        return Object.keys(results).length !== 0 && <CustomDataTable data={filteredResults} />;
    };

    return (
        <Box>
            <Box style={{ display: "flex", flexDirection: "column", justifyContent: "space-around", marginBottom: "2rem" }}>
                <Box display="flex" flexDirection="row" marginBottom="1rem" gap={4}>
                    <CustomAutocomplete
                        options={models.map((model) => displayModelChoice(model))}
                        inputLabel="Device"
                        onChange={(event, newValue) => {
                            const selectedModel = models.find(model => displayModelChoice(model) === newValue);
                            setChosenModel(selectedModel || null);
                        }}
                    />

                    <CustomAutocomplete
                        disabled={versions.length === 0}
                        options={versions.map((version) => version.version)}
                        inputLabel="Device versions"
                        onChange={(event, newValue) => {
                            const selectedVersion = versions.find(version => version.version === newValue);
                            setChosenVersion(selectedVersion || null);
                        }}
                    />
                </Box>

                <Box
                    display="flex"
                    flexDirection="row"
                    gap={4}
                    justifyContent="center"
                    sx={{ "& > *": { flex: 1 } }}
                >
                    <CustomSearch
                        disabled={Object.keys(results).length === 0}
                        label="Filter by executable name"
                        value={executableInput}
                        onChange={(e) => setExecutableInput(e.target.value)}
                    />
                    <CustomSearch
                        disabled={Object.keys(results).length === 0}
                        label="Filter by entitlement key"
                        value={entitlementKeyInput}
                        onChange={(e) => setEntitlementKeyInput(e.target.value)}
                    />
                    <CustomSearch
                        disabled={Object.keys(results).length === 0}
                        label="Filter by entitlement value"
                        value={entitlementValueInput}
                        onChange={(e) => setEntitlementValueInput(e.target.value)}
                    />
                </Box>

            </Box>

            {renderDataTable()}
        </Box>
    );
};

export default EntitlementsSearch;