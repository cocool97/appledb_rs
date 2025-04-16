import "./HomePage.css"
import React, { useEffect, useState } from 'react';
import { API_URL } from '../Constants';
import Card from '../components/Card';
import { GoVersions } from "react-icons/go";
import { useParams } from 'react-router-dom';

interface Version {
    id: number
    version: string
}

const ModelPage = () => {
    const { modelId } = useParams();
    const [versions, setVersions] = useState<[Version]>([]);

    useEffect(() => {
        fetch(`${API_URL}/devices/${modelId}/operating_system_versions`)
            .then((response) => response.json())
            .then((data) => setVersions(data))
            .catch((error) => console.log(error));
    }, [modelId]);

    return (
        <div className="grid">
            {versions.map((version) => (
                <Card
                    key={version.id}
                    id={version.id}
                    to={`/model/${modelId}/version/${version.id}`}
                    icon={<GoVersions />}
                    main={version.version}
                />
            ))}
        </div>
    );
}

export default ModelPage