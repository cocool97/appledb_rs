import {
  Box,
  Collapse,
  IconButton,
  List,
  ListItem,
  ListItemIcon,
  ListItemText,
} from "@mui/material";
import React, { useEffect, useState } from "react";
import {
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
  const [openDevices, setOpenDevices] = useState<Record<string, boolean>>({});

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

  const toggleDevice = (model_code: string) => {
    setOpenDevices((prev) => ({
      ...prev,
      [model_code]: !prev[model_code],
    }));
  };

  return (
    <Box>
      <List sx={{ width: "100%" }} component="nav">
        {devices.map((device) => (
          <React.Fragment key={device.model_code}>
            <ListItem
              onClick={() => toggleDevice(device.model_code)}
              sx={{
                backgroundColor: "#555555",
                borderRadius: "5px",
                cursor: "pointer",
                "&:hover": { backgroundColor: "#666666" },
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
              ].map((elem) => (
                <IconButton
                  key={elem.route}
                  component={Link}
                  to={`${elem.route}?device_id=${device.id}`}
                  sx={{ maxWidth: "fit-content" }}
                  onClick={(e) => e.stopPropagation()}
                >
                  {elem.icon}
                </IconButton>
              ))}
            </ListItem>

            <Collapse
              in={!!openDevices[device.model_code]}
              timeout="auto"
              unmountOnExit
            >
              <List component="div" disablePadding sx={{ pl: 8 }}>
                {device.versions.map((version) => (
                  <ListItem key={version.id} sx={{ pl: 4 }}>
                    <ListItemText primary={version.version} />
                    {[
                      {
                        route: ENTITLEMENTS_SEARCH_ROUTE,
                        icon: ENTITLEMENT_ICON,
                      },
                      { route: EXECUTABLES_ROUTE, icon: EXECUTABLE_ICON },
                      { route: FRAMEWORKS_ROUTE, icon: FRAMEWORK_ICON },
                    ].map((elem) => (
                      <IconButton
                        key={elem.route}
                        component={Link}
                        to={`${elem.route}?device_id=${device.id}&device_version_id=${version.id}`}
                        sx={{ maxWidth: "fit-content" }}
                        onClick={(e) => e.stopPropagation()}
                      >
                        {elem.icon}
                      </IconButton>
                    ))}
                  </ListItem>
                ))}
              </List>
            </Collapse>
          </React.Fragment>
        ))}
      </List>
    </Box>
  );
};

export default DevicesList;
