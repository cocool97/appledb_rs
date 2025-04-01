import * as React from 'react';
import { styled } from '@mui/material/styles';
import Box from '@mui/material/Box';
import CssBaseline from '@mui/material/CssBaseline';
import { Route, BrowserRouter as Router, Routes } from "react-router-dom";
import HomeRoute from "./routes/HomeRoute"
import Stats from "./routes/Stats"
import Diffing from "./routes/Diffing"
import ModelPage from "./routes/ModelPage"
import EntitlementsRoute from "./routes/EntitlementsRoute"
import CustomAppBar from './components/CustomAppBar';
import { DIFF_ROUTE, DRAWER_WIDTH, ENTITLEMENTS_VERSION, MAIN_ROUTE, MODELS, STATS_ROUTE } from './Constants';
import CustomDrawer from './components/CustomDrawer';


const Main = styled('main', { shouldForwardProp: (prop) => prop !== 'open' })(
    ({ theme }) => ({
        flexGrow: 1,
        padding: theme.spacing(3),
        transition: theme.transitions.create('margin', {
            easing: theme.transitions.easing.sharp,
            duration: theme.transitions.duration.leavingScreen,
        }),
        marginLeft: `-${DRAWER_WIDTH}px`,
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
    const [open, setOpen] = React.useState(false);

    const handleDrawerOpen = () => {
        setOpen(true);
    };

    const handleDrawerClose = () => {
        setOpen(false);
    };

    return (
        <Box sx={{ display: 'flex', height: "inherit" }}>
            <CssBaseline />
            <Router>
                <CustomAppBar
                    open={open}
                    handleDrawerOpen={handleDrawerOpen}
                />

                <CustomDrawer
                    open={open}
                    handleDrawerClose={handleDrawerClose}
                />

                <Main open={open} sx={{ position: "relative", top: "64px", height: "calc(100vh - 64px)", overflowY: "scroll" }}>
                    <Routes>
                        <Route exact path={MAIN_ROUTE} element={<HomeRoute />} />
                        <Route exact path={STATS_ROUTE} element={<Stats />} />
                        <Route exact path={DIFF_ROUTE} element={<Diffing />} />
                        <Route exact path={MODELS} element={<ModelPage />} />
                        <Route exact path={ENTITLEMENTS_VERSION} element={<EntitlementsRoute />} />
                        <Route path="*" element={<div>NOT FOUND :)</div>} />
                    </Routes>
                </Main>
            </Router>
        </Box>
    );
}