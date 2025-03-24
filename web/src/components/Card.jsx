import "./Card.css";

const Card = (props) => {
    const { id, icon, main, secondary } = props;

    function displaySecondary(value) {
        return value ? `(${value})` : ""
    }

    return (
        <div key={id} className="card">
            <div className="icon">{icon}</div>
            <span className="main">{main}</span>
            <span className="secondary">{displaySecondary(secondary)}</span>
        </div>
    );
};

export default Card;
