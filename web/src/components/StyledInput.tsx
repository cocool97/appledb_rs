import React from "react";
import "./StyledInput.css"

const StyledInput = (props) => {
    const { placeholder, value, onChange } = props;

    return (
        <div className="container">
            <input type="text" placeholder={placeholder} value={value} className="styled-input" onChange={(e) => onChange(e.target.value)} />
        </div>
    );
};

export default StyledInput;
