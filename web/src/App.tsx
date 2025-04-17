import * as React from 'react';
import { styled } from '@mui/material/styles';
import Box from '@mui/material/Box';
import CssBaseline from '@mui/material/CssBaseline';
import { Route, BrowserRouter as Router, Routes } from "react-router-dom";
import Stats from "./routes/Stats"
import EntitlementsDiffing from "./routes/EntitlementsDiffing"
import EntitlementsSearch from "./routes/EntitlementsSearch"
import CustomAppBar from './components/CustomAppBar';
import { DRAWER_WIDTH, ENTITLEMENTS_DIFF_ROUTE, ENTITLEMENTS_SEARCH_ROUTE, EXECUTABLES_DIFF_ROUTE, EXECUTABLES_FRAMEWORKS_ROUTE, FRAMEWORKS_DIFF_ROUTE, MAIN_ROUTE, TASKS_ROUTE } from './Constants';
import CustomDrawer from './components/CustomDrawer';
import ExecutablesDiffing from './routes/ExecutablesDffing';
import Tasks from './routes/Tasks';
import FrameworksDiffing from './routes/FrameworksDiffing';
import { TitledComponent } from './components/TitledComponent';
import ExecutablesFrameworks from './routes/ExecutablesFrameworks';

const Main = styled('main', { shouldForwardProp: (prop) => prop !== 'open' })(
    ({ theme }) => ({
        flexGrow: 1,
        marginLeft: `-${DRAWER_WIDTH}px`,
        padding: theme.spacing(3),
        transition: theme.transitions.create('margin', {
            duration: theme.transitions.duration.leavingScreen,
            easing: theme.transitions.easing.sharp,
        }),
        variants: [
            {
                props: ({ open }) => open,
                style: {
                    transition: theme.transitions.create('margin', {
                        easing: theme.transitions.easing.easeOut,
                        duration: theme.transitions.duration.enteringScreen,
                    }),
                    marginLeft: 0,
                },
            },
        ],
    }),
);

export default function App() {
    const [drawerOpen, setDrawerOpen] = React.useState(false);

    return (
        <Box sx={{ display: 'flex', height: "inherit" }}>
            <CssBaseline />
            <Router>
                <CustomAppBar
                    drawerOpen={drawerOpen}
                    handleDrawerOpen={() => setDrawerOpen(true)}
                />

                <CustomDrawer
                    drawerOpen={drawerOpen}
                    setDrawerOpen={setDrawerOpen}
                />

                <Main open={drawerOpen} sx={{ height: "calc(100vh - 64px)", overflowY: "scroll", position: "relative", top: "64px", }}>
                    <Routes>
                        <Route
                            path={MAIN_ROUTE}
                            element={<TitledComponent title="Server stats" component={<Stats />} />}
                        />
                        <Route
                            path={ENTITLEMENTS_SEARCH_ROUTE}
                            element={<TitledComponent title="Entitlements per versions" component={<EntitlementsSearch />} />}
                        />
                        <Route
                            path={ENTITLEMENTS_DIFF_ROUTE}
                            element={<TitledComponent title="Entitlements diffing" component={<EntitlementsDiffing />} />}
                        />

                        <Route
                            path={EXECUTABLES_DIFF_ROUTE}
                            element={<TitledComponent title="Executables diffing" component={<ExecutablesDiffing />} />}
                        />
                        <Route
                            path={EXECUTABLES_FRAMEWORKS_ROUTE}
                            element={<TitledComponent title="Executables frameworks" component={<ExecutablesFrameworks />} />}
                        />

                        <Route
                            path={FRAMEWORKS_DIFF_ROUTE}
                            element={<TitledComponent title="Frameworks diffing" component={<FrameworksDiffing />} />}
                        />

                        <Route
                            path={TASKS_ROUTE}
                            element={<TitledComponent title="Running tasks" component={<Tasks />} />}
                        />

                        <Route
                            path="*"
                            element={<div>NOT FOUND :)</div>}
                        />
                    </Routes>
                </Main>
            </Router>
        </Box>
    );
}