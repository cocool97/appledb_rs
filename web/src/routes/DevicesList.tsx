import {
  Box,
  IconButton,
  List,
  ListItem,
  ListItemButton,
  ListItemIcon,
  ListItemText,
} from "@mui/material";
import React, { useEffect, useState } from "react";
import {
  API_URL,
  ENTITLEMENTS_SEARCH_ROUTE,
  EXECUTABLES_ROUTE,
  FRAMEWORKS_ROUTE,
  GET_ALL_DEVICES_ENDPOINT,
} from "../Constants";
import {
  DEVICE_ICON,
  ENTITLEMENT_ICON,
  EXECUTABLE_ICON,
  FRAMEWORK_ICON,
} from "../icons";
import { Link } from "react-router-dom";
import { Device } from "../types/device";

const DevicesList = () => {
  const [devices, setDevices] = useState<Device[]>([]);

  useEffect(() => {
    fetch(`${GET_ALL_DEVICES_ENDPOINT}`)
      .then((response) => {
        if (!response.ok) {
          throw new Error("Failed to fetch devices list");
        }
        return response.json();
      })
      .then((data) => {
        setDevices(data);
      })
      .catch((error) => {
        console.error(error);
        setDevices([]);
      });
  }, []);

  return (
    <Box>
      <List sx={{ width: "100%" }} component="nav">
        {devices.map((device) => {
          return (
            <ListItem
              key={device.model_code}
              sx={{
                backgroundColor: "#555555",
                borderRadius: "5px",
                marginBottom: "10px",
              }}
            >
              <ListItemIcon>{DEVICE_ICON}</ListItemIcon>
              <ListItemText
                primary={`${device.display_name} (${device.model_code})`}
              />
              {[
                { route: ENTITLEMENTS_SEARCH_ROUTE, icon: ENTITLEMENT_ICON },
                { route: EXECUTABLES_ROUTE, icon: EXECUTABLE_ICON },
                { route: FRAMEWORKS_ROUTE, icon: FRAMEWORK_ICON },
              ].map((elem) => {
                return (
                  <IconButton
                    component={Link}
                    to={`${elem.route}?device_id=${device.id}`}
                    sx={{ maxWidth: "fit-content" }}
                  >
                    {elem.icon}
                  </IconButton>
                );
              })}
            </ListItem>
          );
        })}
      </List>
    </Box>
  );
};

export default DevicesList;
