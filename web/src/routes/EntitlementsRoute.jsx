import React, { useState, useEffect } from 'react';
import { useParams } from 'react-router-dom';
import CustomDataTable from '../components/CustomDataTable';
import StyledInput from '../components/StyledInput';
import { API_URL } from "../Constants";
import { BarLoader } from "react-spinners"

import "./EntitlementRoute.css"


const EntitlementsRoute = () => {
    const { versionId } = useParams();

    const [results, setResults] = useState({});
    const [isLoading, setLoading] = useState(true)

    const [executableInput, setExecutableInput] = useState('');
    const [entitlementKeyInput, setEntitlementKeyInput] = useState('');
    const [entitlementValueInput, setEntitlementValueInput] = useState('');

    useEffect(() => {
        fetch(`${API_URL}/api/v1/operating_systems/${versionId}/executable_entitlements`)
            .then((response) => response.json())
            .then((data) => setResults(data))
            .then(() => setLoading(false))
            .catch((error) => console.log(error));
    }, [versionId]);

    function filterObject(obj) {
        const result = {};

        for (const [key, value] of Object.entries(obj)) {
            if (!value.name.includes(executableInput)) continue;

            const filteredArray = value.entitlements.filter(item =>
                item.key.toLowerCase().includes(entitlementKeyInput.toLowerCase()) &&
                item.value.toLowerCase().includes(entitlementValueInput.toLowerCase())
            );

            if (filteredArray.length > 0) {
                result[key] = {name: value.name, entitlements: filteredArray};
            }
        }

        return result;
    }

    const renderDataTable = () => {
        if (isLoading) {
            return (
                <div style={{ width: "inherit", height: "inherit", display: "flex", justifyContent: "center", alignItems: "center", backgroundColor: "inherit" }}>
                    <BarLoader color="white" width="10rem" />
                </div>
            )
        }

        return <CustomDataTable data={filterObject(results)} />
    }

    return (
        <div>
            <div
                style={{
                    display: "flex",
                    justifyContent: "space-around",
                    marginBottom: "2rem"
                }}
            >
                <StyledInput placeholder="Filter by executable name" value={executableInput} onChange={setExecutableInput} />
                <StyledInput placeholder="Filter by entitlement key" value={entitlementKeyInput} onChange={setEntitlementKeyInput} />
                <StyledInput placeholder="Filter by entitlement value" value={entitlementValueInput} onChange={setEntitlementValueInput} />
            </div>
            {renderDataTable()}
        </div>
    );
}

export default EntitlementsRoute