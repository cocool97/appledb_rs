import { Link } from "react-router-dom";
import "./Card.css";
import React from "react";

const Card = (props) => {
    const { id, icon, main, secondary, to } = props;

    function displaySecondary(value) {
        return value ? `(${value})` : ""
    }

    return (
        <Link key={id} to={to} className="grid-item">
            <div key={id} className="card">
                <div className="icon">{icon}</div>
                <span className="main">{main}</span>
                <span className="secondary">{displaySecondary(secondary)}</span>
            </div>
        </Link>
    );
};

export default Card;
