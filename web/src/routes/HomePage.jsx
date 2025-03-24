import React, { useState, useEffect } from 'react';
import Card from '../components/Card';
import { MdPhoneIphone } from "react-icons/md";
import { Link } from 'react-router-dom';
import './HomePage.css';
import { GET_ALL_DEVICES } from '../Constants';

const HomePage = () => {
    const [models, setModels] = useState([]);

    useEffect(() => {
        fetch(GET_ALL_DEVICES)
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

    return (
        <div className="grid">
            {models.map((model) => (
                <div key={model.id} className="container-card">
                    <Link to={`/model/${model.id}`} className="grid-item">
                        <Card
                            id={model.id}
                            icon={<MdPhoneIphone />}
                            main={modelName(model.display_name, model.model_code)}
                            secondary={modelCode(model.display_name, model.model_code)}
                        />
                    </Link>
                </div>
            ))}
        </div>
    );
}

export default HomePage;