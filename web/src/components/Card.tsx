import "./Card.css";
import React from "react";

const Card = (props) => {
    const { icon, main, secondary } = props;

    return (
        <div className="card">
            <div className="icon">{icon}</div>
            <span className="main">{main}</span>
            <span className="secondary">{secondary}</span>
        </div>
    );
};

export default Card;
