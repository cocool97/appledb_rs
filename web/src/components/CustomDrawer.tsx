import React from "react";
import {
  DRAWER_WIDTH,
  ENTITLEMENTS_DIFF_ROUTE,
  ENTITLEMENTS_SEARCH_ROUTE,
  EXECUTABLES_DIFF_ROUTE,
  EXECUTABLES_ROUTE,
  FRAMEWORKS_DIFF_ROUTE,
  FRAMEWORKS_ROUTE,
  TASKS_ROUTE,
} from "../Constants";
import { useNavigate } from "react-router-dom";
import {
  Box,
  Divider,
  Drawer,
  IconButton,
  List,
  styled,
  useTheme,
} from "@mui/material";
import ChevronLeftIcon from "@mui/icons-material/ChevronLeft";
import ChevronRightIcon from "@mui/icons-material/ChevronRight";
import ListItem from "@mui/material/ListItem";
import ListItemButton from "@mui/material/ListItemButton";
import ListItemIcon from "@mui/material/ListItemIcon";
import ListItemText from "@mui/material/ListItemText";
import ExpandLess from "@mui/icons-material/ExpandLess";
import ExpandMore from "@mui/icons-material/ExpandMore";
import { Collapse } from "@mui/material";
import Typography from "@mui/material/Typography";
import {
  DIFFERENCE_ICON,
  ENTITLEMENT_ICON,
  EXECUTABLE_ICON,
  FRAMEWORK_ICON,
  SEARCH_ICON,
  TASK_ICON,
} from "../icons";

const DrawerHeader = styled("div")(({ theme }) => ({
  alignItems: "center",
  backgroundColor: "#555555",
  display: "flex",
  padding: theme.spacing(0, 1),
  ...theme.mixins.toolbar,
  justifyContent: "flex-end",
}));

const DrawerListItem = (props) => {
  const { to, icon, text } = props;
  const navigate = useNavigate();

  return (
    <ListItem disablePadding>
      <ListItemButton onClick={() => navigate(to)}>
        <ListItemIcon>{icon}</ListItemIcon>
        <ListItemText primary={text} sx={{ color: "white" }} />
      </ListItemButton>
    </ListItem>
  );
};

const DrawerListItems = (props) => {
  const { items, categoryName, categoryIcon } = props;
  const [listOpen, setListOpen] = React.useState(false);
  const handleListChange = () => {
    setListOpen(!listOpen);
  };

  return (
    <>
      <ListItemButton onClick={handleListChange}>
        <ListItemIcon>{categoryIcon}</ListItemIcon>
        <ListItemText primary={categoryName} sx={{ color: "white" }} />
        {listOpen ? (
          <ExpandLess style={{ color: "white" }} />
        ) : (
          <ExpandMore style={{ color: "white" }} />
        )}
      </ListItemButton>
      <Collapse
        in={listOpen}
        sx={{ padding: "0 1rem" }}
        timeout="auto"
        unmountOnExit
      >
        <List component="div" disablePadding>
          {items.map((item, index) => (
            <DrawerListItem
              key={index}
              to={item.to}
              icon={item.icon}
              text={item.text}
            />
          ))}
        </List>
      </Collapse>
      <Divider />
    </>
  );
};

const CustomDrawer = (props) => {
  const theme = useTheme();
  const { setDrawerOpen, drawerOpen } = props;

  return (
    <Drawer
      sx={{
        width: DRAWER_WIDTH,
        flexShrink: 0,
        "& .MuiDrawer-paper": {
          width: DRAWER_WIDTH,
          boxSizing: "border-box",
          backgroundColor: "#555555",
          display: "flex",
          flexDirection: "column",
          justifyContent: "space-between",
        },
      }}
      variant="persistent"
      anchor="left"
      open={drawerOpen}
    >
      <Box>
        <DrawerHeader>
          <IconButton
            onClick={() => setDrawerOpen(!drawerOpen)}
            style={{ backgroundColor: "transparent" }}
          >
            {theme.direction === "ltr" ? (
              <ChevronLeftIcon style={{ color: "white" }} />
            ) : (
              <ChevronRightIcon style={{ color: "white" }} />
            )}
          </IconButton>
        </DrawerHeader>
        <Divider />
        <List>
          <DrawerListItem
            to={EXECUTABLES_ROUTE}
            icon={EXECUTABLE_ICON}
            text="Executables"
          />
          <Divider />

          <DrawerListItem
            to={FRAMEWORKS_ROUTE}
            icon={FRAMEWORK_ICON}
            text="Frameworks"
          />
          <Divider />

          <DrawerListItem
            to={ENTITLEMENTS_SEARCH_ROUTE}
            icon={SEARCH_ICON}
            text="Search for entitlements"
          />
          <Divider />

          <DrawerListItems
            items={[
              {
                to: ENTITLEMENTS_DIFF_ROUTE,
                icon: ENTITLEMENT_ICON,
                text: "Entitlements",
              },
              {
                to: FRAMEWORKS_DIFF_ROUTE,
                icon: FRAMEWORK_ICON,
                text: "Frameworks",
              },
              {
                to: EXECUTABLES_DIFF_ROUTE,
                icon: EXECUTABLE_ICON,
                text: "Executables",
              },
            ]}
            categoryName="Diffing"
            categoryIcon={DIFFERENCE_ICON}
          />
        </List>
      </Box>

      <Box>
        <Divider />
        <DrawerListItem to={TASKS_ROUTE} icon={TASK_ICON} text="Tasks" />
        <Divider />
        <Typography
          sx={{
            width: "100%",
            textAlign: "center",
            color: "white",
            fontWeight: "bold",
            padding: "1rem 0",
          }}
        >
          {__APP_VERSION__}
        </Typography>
      </Box>
    </Drawer>
  );
};

export default CustomDrawer;
