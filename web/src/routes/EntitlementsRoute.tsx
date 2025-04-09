import React, { useState, useEffect, useMemo, useCallback } from 'react';
import { useParams } from 'react-router-dom';
import { CustomDataTable } from '../components/CustomDataTable';
import StyledInput from '../components/StyledInput';
import { API_URL } from "../Constants";
import { BarLoader } from "react-spinners";

import "./EntitlementRoute.css";

const EntitlementsRoute = () => {
    const { versionId } = useParams();

    const [results, setResults] = useState({});
    const [isLoading, setLoading] = useState(true);

    const [executableInput, setExecutableInput] = useState('');
    const [entitlementKeyInput, setEntitlementKeyInput] = useState('');
    const [entitlementValueInput, setEntitlementValueInput] = useState('');

    useEffect(() => {
        fetch(`${API_URL}/operating_systems/${versionId}/executable_entitlements`)
            .then((response) => response.json())
            .then((data) => setResults(data))
            .then(() => setLoading(false))
            .catch((error) => console.log(error));
    }, [versionId]);

    const filterObject = useCallback((obj) => {
        const result = {};
        for (const [key, value] of Object.entries(obj)) {
            if (!value.name.toLowerCase().includes(executableInput.toLowerCase())) continue;

            const filteredArray = value.entitlements.filter(item =>
                item.key.toLowerCase().includes(entitlementKeyInput.toLowerCase()) &&
                item.value.toLowerCase().includes(entitlementValueInput.toLowerCase())
            );

            if (filteredArray.length > 0) {
                result[key] = { name: value.name, entitlements: filteredArray };
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
        return <CustomDataTable data={filteredResults} />;
    };

    return (
        <div>
            <div style={{ display: "flex", justifyContent: "space-around", marginBottom: "2rem" }}>
                <StyledInput placeholder="Filter by executable name" value={executableInput} onChange={setExecutableInput} />
                <StyledInput placeholder="Filter by entitlement key" value={entitlementKeyInput} onChange={setEntitlementKeyInput} />
                <StyledInput placeholder="Filter by entitlement value" value={entitlementValueInput} onChange={setEntitlementValueInput} />
            </div>
            {renderDataTable()}
        </div>
    );
};

export default EntitlementsRoute;