import React, { useState, useEffect } from 'react';
import { useParams } from 'react-router-dom';
import CustomDataTable from '../components/CustomDataTable';


const VersionPage = () => {
    const { versionId } = useParams();

    const [results, setResults] = useState({});

    const [executableInput, setExecutableInput] = useState('');
    const [entitlementKeyInput, setEntitlementKeyInput] = useState('');
    const [entitlementValueInput, setEntitlementValueInput] = useState('');

    useEffect(() => {
        fetch(`http://127.0.0.1:4000/api/v1/operating_systems/${versionId}/executable_entitlements`)
            .then((response) => response.json())
            .then((data) => setResults(data))
            .catch((error) => console.log(error));
    }, [versionId]);

    function filterObject(obj) {
        const result = {};

        for (const [key, value] of Object.entries(obj)) {
            if (!key.includes(executableInput)) continue;

            const filteredArray = value.filter(item =>
                item.key.toLowerCase().includes(entitlementKeyInput.toLowerCase()) &&
                item.value.toLowerCase().includes(entitlementValueInput.toLowerCase())
            );

            if (filteredArray.length > 0) {
                result[key] = filteredArray;
            }
        }

        return result;
    }

    return (
        <div>
            <label>
                Executable name: <input name="executableInput" value={executableInput} onChange={e => setExecutableInput(e.target.value)} />
            </label>

            <label>
                Entitlement key: <input name="entKeyInput" value={entitlementKeyInput} onChange={e => setEntitlementKeyInput(e.target.value)} />
            </label>

            <label>
                Entitlement value: <input name="entValueInput" value={entitlementValueInput} onChange={e => setEntitlementValueInput(e.target.value)} />
            </label>

            <div>
                <CustomDataTable data={filterObject(results)} />
            </div>
        </div>
    );
}

export default VersionPage