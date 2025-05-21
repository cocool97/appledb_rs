import React from "react";
import { Box } from "@mui/material";

const Card = (props) => {
  const { icon, main, secondary } = props;

  return (
    <Box
      sx={{
        width: "200px",
        height: "250px",
        display: "flex",
        flexDirection: "column",
        alignItems: "center",
        justifyContent: "center",
        backgroundColor: "darkgray",
        borderRadius: "10px",
        boxShadow: "0 4px 6px rgba(0, 0, 0, 0.1)",
        p: 2,
        textAlign: "center",
        fontSize: "1rem",
        fontWeight: "bold",
      }}
    >
      <Box
        sx={{
          fontSize: "48px",
          color: "#333",
          mb: "8px",
        }}
      >
        {icon}
      </Box>
      <Box
        component="span"
        sx={{
          fontSize: "20px",
          mt: "8px",
          color: "#222",
          fontWeight: "bold",
        }}
      >
        {main}
      </Box>
      <Box
        component="span"
        sx={{
          fontSize: "14px",
          mt: "8px",
          color: "#696969",
          fontStyle: "italic",
        }}
      >
        {secondary}
      </Box>
    </Box>
  );
};

export default Card;
