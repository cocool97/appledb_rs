import React from "react";
import { ENTITLEMENTS_DIFF_ROUTE, DRAWER_WIDTH, EXECUTABLES_DIFF_ROUTE, TASKS_ROUTE, FRAMEWORKS_DIFF_ROUTE, ENTITLEMENTS_SEARCH_ROUTE } from "../Constants";
import { useNavigate } from "react-router-dom";
import { Divider, Drawer, IconButton, List, styled, useTheme } from "@mui/material";
import ChevronLeftIcon from '@mui/icons-material/ChevronLeft';
import ChevronRightIcon from '@mui/icons-material/ChevronRight';
import ListItem from '@mui/material/ListItem';
import ListItemButton from '@mui/material/ListItemButton';
import ListItemIcon from '@mui/material/ListItemIcon';
import ListItemText from '@mui/material/ListItemText';
import ExpandLess from '@mui/icons-material/ExpandLess';
import ExpandMore from '@mui/icons-material/ExpandMore';
import { Collapse } from '@mui/material';
import Typography from '@mui/material/Typography';
import KeyIcon from '@mui/icons-material/Key';
import WorkIcon from '@mui/icons-material/Work';
import DifferenceIcon from '@mui/icons-material/Difference';
import LanguageIcon from '@mui/icons-material/Language';
import LibraryBooksIcon from '@mui/icons-material/LibraryBooks';
import SearchIcon from '@mui/icons-material/Search';

const DrawerHeader = styled('div')(({ theme }) => ({
    alignItems: 'center',
    backgroundColor: "#555555",
    display: 'flex',
    padding: theme.spacing(0, 1),
    ...theme.mixins.toolbar,
    justifyContent: 'flex-end',
}));


const DrawerListItem = (props) => {
    const { to, icon, text } = props;
    const navigate = useNavigate();

    return (
        <ListItem disablePadding>
            <ListItemButton onClick={() => navigate(to)}>
                <ListItemIcon>
                    {icon}
                </ListItemIcon>
                <ListItemText primary={text} sx={{ color: "white", }} />
            </ListItemButton>
        </ListItem>
    )
}

const DrawerListItems = (props) => {
    const { items, categoryName, categoryIcon } = props;
    const [listOpen, setListOpen] = React.useState(false);
    const handleListChange = () => {
        setListOpen(!listOpen);
    };

    return (
        <>
            <ListItemButton onClick={handleListChange}>
                <ListItemIcon>
                    {categoryIcon}
                </ListItemIcon>
                <ListItemText primary={categoryName} sx={{ color: "white", }} />
                {listOpen ? <ExpandLess style={{ color: "white" }} /> : <ExpandMore style={{ color: "white" }} />}
            </ListItemButton>
            <Collapse in={listOpen} sx={{ padding: "0 1rem" }} timeout="auto" unmountOnExit>
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
    )
}

const CustomDrawer = (props) => {
    const theme = useTheme();
    const { setDrawerOpen, drawerOpen } = props;

    return (
        <Drawer
            sx={{
                width: DRAWER_WIDTH,
                flexShrink: 0,
                '& .MuiDrawer-paper': {
                    width: DRAWER_WIDTH,
                    boxSizing: 'border-box',
                    backgroundColor: "#555555"
                },
            }}
            variant="persistent"
            anchor="left"
            open={drawerOpen}
        >
            <DrawerHeader>
                <IconButton onClick={() => setDrawerOpen(!drawerOpen)} style={{ backgroundColor: 'transparent' }} >
                    {theme.direction === 'ltr' ? <ChevronLeftIcon style={{ color: "white" }} /> : <ChevronRightIcon style={{ color: "white" }} />}
                </IconButton>
            </DrawerHeader>
            <Divider />
            <List sx={{ height: "inherit" }}>
                <DrawerListItems
                    categoryName="Entitlements"
                    categoryIcon={<KeyIcon style={{ color: "white" }} />}
                    items={[{ text: "Diffing", to: ENTITLEMENTS_DIFF_ROUTE, icon: <DifferenceIcon style={{ color: "white" }} /> }, { text: "Search", to: ENTITLEMENTS_SEARCH_ROUTE, icon: <SearchIcon style={{ color: "white" }} /> }]}
                />

                <DrawerListItems
                    categoryName="Executables"
                    categoryIcon={<LanguageIcon style={{ color: "white" }} />}
                    items={[{ text: "Diffing", to: EXECUTABLES_DIFF_ROUTE, icon: <DifferenceIcon style={{ color: "white" }} /> }]}
                />

                <DrawerListItems
                    categoryName="Frameworks"
                    categoryIcon={<LibraryBooksIcon style={{ color: "white" }} />}
                    items={[{ text: "Diffing", to: FRAMEWORKS_DIFF_ROUTE, icon: <DifferenceIcon style={{ color: "white" }} /> }]}
                />

                <DrawerListItem
                    to={TASKS_ROUTE}
                    icon={<WorkIcon style={{ color: "white" }} />}
                    text="Tasks"
                />
                <Divider />

                <Typography sx={{ position: "absolute", bottom: 0, width: "100%", textAlign: "center", color: "white", fontWeight: "bold", marginBottom: "1rem" }}>{__APP_VERSION__}</Typography>
            </List>
        </Drawer>
    )
}

export default CustomDrawer;