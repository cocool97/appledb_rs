import { Box, Typography } from "@mui/material";
import React from "react";
import logo from "../logo.png";
import "./NotFound.css";

export const NotFound = () => {
  return (
    <Box height="100%" sx={{ textAlign: "center", alignContent: "center" }}>
      <img
        className="rotating"
        style={{
          filter: "grayscale(0.5)",
        }}
        src={logo}
        alt="logo"
      />
      <Typography variant="h5">Not Found...</Typography>
    </Box>
  );
};
