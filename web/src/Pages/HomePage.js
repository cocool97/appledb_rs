import React, { useState, useEffect } from 'react';
import Card from '../Components/Card';
import { MdPhoneIphone } from "react-icons/md";
import { Link } from 'react-router-dom';
import './HomePage.css';

const HomePage = () => {
    const [models, setModels] = useState([]);

    useEffect(() => {
        fetch("http://127.0.0.1:4000/api/v1/devices/all")
            .then((response) => response.json())
            .then((data) => setModels(data))
            .catch((error) => console.log(error));
    }, []);

    function modelName(display_name, model) {
        return display_name ?? model;
    }

    return (
        <div className="container">
            <div className="grid">
                {models.map((model) => (
                    <div className="container-card">
                        <Link key={model.id} to={`/model/${model.id}`} className="grid-item">
                            <Card
                                id={model.id}
                                icon={<MdPhoneIphone />}
                                span={modelName(model.display_name, model.model)}
                            />
                        </Link>
                    </div>
                ))}
            </div>
        </div>
    );
}

export default HomePage;