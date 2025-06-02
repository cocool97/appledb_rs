import * as React from "react";
import { styled } from "@mui/material/styles";
import Box from "@mui/material/Box";
import CssBaseline from "@mui/material/CssBaseline";
import { Route, BrowserRouter as Router, Routes } from "react-router-dom";
import Home from "./routes/Home";
import CustomAppBar from "./components/CustomAppBar";
import {
  DRAWER_WIDTH,
  ENTITLEMENTS_DIFF_ROUTE,
  ENTITLEMENTS_SEARCH_ROUTE,
  EXECUTABLES_DIFF_ROUTE,
  EXECUTABLES_ROUTE,
  FRAMEWORKS_DIFF_ROUTE,
  FRAMEWORKS_ROUTE,
  HOME_ROUTE,
  TASKS_ROUTE,
} from "./Constants";
import CustomDrawer from "./components/CustomDrawer";
import Tasks from "./routes/Tasks";
import { TitledComponent } from "./components/TitledComponent";
import {
  EXECUTABLE_INFOS,
  FRAMEWORK_INFOS,
  HOME_PAGE_INFOS,
  TASKS_INFOS,
} from "./tooltip_text";
import { Executables } from "./routes/Executables";
import EntitlementsSearch from "./routes/EntitlementsSearch";
import EntitlementsDiffing from "./routes/diffing/EntitlementsDiffing";
import FrameworksDiffing from "./routes/diffing/FrameworksDiffing";
import ExecutablesDiffing from "./routes/diffing/ExecutablesDffing";
import { Frameworks } from "./routes/Frameworks";
import {
  ENTITLEMENT_ICON,
  EXECUTABLE_ICON,
  FRAMEWORK_ICON,
  SEARCH_ICON,
  TASK_ICON,
} from "./icons";
import { NotFound } from "./routes/NotFound";

const Main = styled("main", { shouldForwardProp: (prop) => prop !== "open" })(
  ({ theme }) => ({
    flexGrow: 1,
    marginLeft: `-${DRAWER_WIDTH}px`,
    padding: theme.spacing(3),
    transition: theme.transitions.create("margin", {
      duration: theme.transitions.duration.leavingScreen,
      easing: theme.transitions.easing.sharp,
    }),
    variants: [
      {
        props: ({ open }) => open,
        style: {
          transition: theme.transitions.create("margin", {
            easing: theme.transitions.easing.easeOut,
            duration: theme.transitions.duration.enteringScreen,
          }),
          marginLeft: 0,
        },
      },
    ],
  }),
);

const ROUTES = [
  {
    path: HOME_ROUTE,
    element: (
      <TitledComponent
        title="Server stats"
        tooltip={HOME_PAGE_INFOS}
        component={<Home />}
      />
    ),
  },
  {
    path: EXECUTABLES_ROUTE,
    element: (
      <TitledComponent
        title="Executable information"
        tooltip={EXECUTABLE_INFOS}
        logo={EXECUTABLE_ICON}
        component={<Executables />}
      />
    ),
  },
  {
    path: FRAMEWORKS_ROUTE,
    element: (
      <TitledComponent
        title="Framework information"
        tooltip={FRAMEWORK_INFOS}
        logo={FRAMEWORK_ICON}
        component={<Frameworks />}
      />
    ),
  },
  {
    path: ENTITLEMENTS_SEARCH_ROUTE,
    element: (
      <TitledComponent
        title="Entitlements - Search"
        logo={SEARCH_ICON}
        component={<EntitlementsSearch />}
      />
    ),
  },
  {
    path: ENTITLEMENTS_DIFF_ROUTE,
    element: (
      <TitledComponent
        title="Entitlements - Diff"
        logo={ENTITLEMENT_ICON}
        component={<EntitlementsDiffing />}
      />
    ),
  },
  {
    path: FRAMEWORKS_DIFF_ROUTE,
    element: (
      <TitledComponent
        title="Frameworks - Diff"
        logo={FRAMEWORK_ICON}
        component={<FrameworksDiffing />}
      />
    ),
  },
  {
    path: EXECUTABLES_DIFF_ROUTE,
    element: (
      <TitledComponent
        title="Executables - Diff"
        logo={EXECUTABLE_ICON}
        component={<ExecutablesDiffing />}
      />
    ),
  },
  {
    path: TASKS_ROUTE,
    element: (
      <TitledComponent
        title="Running tasks"
        tooltip={TASKS_INFOS}
        logo={TASK_ICON}
        component={<Tasks />}
      />
    ),
  },
  {
    path: "*",
    element: <NotFound />,
  },
];

export default function App() {
  const [drawerOpen, setDrawerOpen] = React.useState(false);

  return (
    <Box sx={{ display: "flex", height: "inherit" }}>
      <CssBaseline />
      <Router>
        <CustomAppBar
          drawerOpen={drawerOpen}
          handleDrawerOpen={() => setDrawerOpen(true)}
        />

        <CustomDrawer drawerOpen={drawerOpen} setDrawerOpen={setDrawerOpen} />

        <Main
          open={drawerOpen}
          sx={{
            height: "calc(100vh - 64px)",
            overflowY: "scroll",
            position: "relative",
            top: "64px",
          }}
        >
          <Routes>
            {ROUTES.map((route, index) => {
              return (
                <Route key={index} path={route.path} element={route.element} />
              );
            })}
          </Routes>
        </Main>
      </Router>
    </Box>
  );
}
