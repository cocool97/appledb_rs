import { AppBar, IconButton, Toolbar } from "@mui/material";

import logo from '../../public/favicon.ico';
import { useNavigate } from "react-router-dom";

import "./CustomAppBar.css"

const CustomAppBarButton = (props) => {
  const navigate = useNavigate();

  return (
    <IconButton
      className="custom-app-bar-button"
      disabled={props.disabled ?? false}
      style={{ marginRight: "5rem", color: "white", fontWeight: "bold", fontSize: "1rem" }}
      onClick={props.to ? () => navigate(props.to) : null}
    >
      <p>{props.name}</p>
    </IconButton>
  )
}

const CustomAppBarLogoButton = () => {
  const navigate = useNavigate();

  return (
    <IconButton
      edge="start"
      aria-label="menu"
      sx={{ mr: 2 }}
      style={{ marginRight: "2rem", color: "white" }}
      onClick={() => navigate("/")}
    >
      <img style={{ height: "64px", maxHeight: "64px", filter: "grayscale(0.5)" }} src={logo} alt="logo" />
    </IconButton>
  )
}

const CustomAppBar = () => {
  const version = __APP_VERSION__

  return (
    <AppBar position="static" style={{ backgroundColor: "#4a4a4a", height: "64px", maxHeight: "64px" }}>
      <Toolbar>
        <CustomAppBarLogoButton />

        <div style={{ maxHeight: "64px", height: "64px", display: "flex", width: "100%" }}>
          <CustomAppBarButton name="VERSIONS" to="/" />
          <CustomAppBarButton name="DIFFING" to="/diff" />
          <CustomAppBarButton name="STATS" to="/stats" />
          <CustomAppBarButton name={version} disabled={true} />
        </div>
      </Toolbar>
    </AppBar>
  )
}

export default CustomAppBar;