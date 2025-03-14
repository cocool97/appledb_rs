import "./Card.css";

const Card = (props) => {
    const { id, icon, span } = props;

    return (
        <div key={id} className="card">
           <div className="icon">{icon}</div> 
            <span className="name">{span}</span>
        </div>
    );
};

export default Card;
