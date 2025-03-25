import React, { useState, useEffect } from 'react';
import Card from '../components/Card';
import { MdPhoneIphone } from "react-icons/md";
import './HomePage.css';
import { GET_ALL_DEVICES_ENDPOINT } from '../Constants';

const HomeRoute = () => {
    const [models, setModels] = useState([]);

    useEffect(() => {
        fetch(GET_ALL_DEVICES_ENDPOINT)
            .then((response) => response.json())
            .then((data) => setModels(data))
            .catch((error) => console.log(error));
    }, []);

    function modelName(display_name, model_code) {
        return display_name ?? model_code;
    }

    function modelCode(display_name, model_code) {
        if (display_name) {
            return model_code
        }

        return null
    }

    const renderModels = () => {
        if (models.length === 0) {
            return (
                <div style={{ width: "inherit", height: "inherit", textAlign: "center", fontWeight: "bold", fontSize: "2.5rem" }}>
                    No models found
                </div>
            )
        }

        return models.map((model) => (
            <Card
                key={model.id}
                id={model.id}
                to={`/model/${model.id}`}
                icon={<MdPhoneIphone />}
                main={modelName(model.display_name, model.model_code)}
                secondary={modelCode(model.display_name, model.model_code)}
            />
        ))
    }

    return (
        <div className="grid">
            {renderModels()}
        </div>
    );
}

export default HomeRoute;