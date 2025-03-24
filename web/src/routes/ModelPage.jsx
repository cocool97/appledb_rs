import React, { useEffect, useState } from 'react';
import { Link, useParams } from 'react-router-dom';
import { GoVersions } from "react-icons/go";
import "./HomePage.css"
import Card from '../components/Card';

const ModelPage = () => {
    const { modelId } = useParams();
    const [versions, setVersions] = useState([]);

    useEffect(() => {
        fetch(`http://127.0.0.1:4000/api/v1/devices/${modelId}/operating_system_versions`)
            .then((response) => response.json())
            .then((data) => setVersions(data))
            .catch((error) => console.log(error));
    }, [modelId]);

    return (
        <div className="container">
            <div className="grid">
                {versions.map((version) => (
                    <div className="container-card">
                        <Link key={version.id} to={`/model/${modelId}/version/${version.id}`} className="grid-item">
                            <Card
                                id={version.id}
                                icon={<GoVersions />}
                                main={version.version}
                            />
                        </Link>
                    </div>
                ))}
            </div>
        </div>
    );
}

export default ModelPage