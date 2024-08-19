import * as React from 'react';
import { useNavigate } from "react-router-dom";

import { styled } from '@mui/material/styles';
import Box from '@mui/material/Box';
import MuiDrawer from '@mui/material/Drawer';
import Toolbar from '@mui/material/Toolbar';
import List from '@mui/material/List';
import ListItem from '@mui/material/ListItem';
import ListItemButton from '@mui/material/ListItemButton';
import ListItemIcon from '@mui/material/ListItemIcon';
import ListItemText from '@mui/material/ListItemText';
import HomeIcon from '@mui/icons-material/Home';
import InfoIcon from '@mui/icons-material/InfoTwoTone';
import SpokeIcon from '@mui/icons-material/SpokeTwoTone';
import AppsIcon from '@mui/icons-material/AppsTwoTone';
import AccountTreeIcon from '@mui/icons-material/AccountTreeTwoTone';
import PublicIcon from '@mui/icons-material/PublicTwoTone';

import { useSelector } from '@xstate/react';
import { appStore } from '../state/appStore';

export const drawerWidth = 240;

const openedMixin = (theme) => ({
    width: drawerWidth,
    transition: theme.transitions.create('width', {
        easing: theme.transitions.easing.sharp,
        duration: theme.transitions.duration.enteringScreen,
    }),
    overflowX: 'hidden',
});

const closedMixin = (theme) => ({
    transition: theme.transitions.create('width', {
        easing: theme.transitions.easing.sharp,
        duration: theme.transitions.duration.leavingScreen,
    }),
    overflowX: 'hidden',
    width: `calc(${theme.spacing(7)} + 1px)`,
    [theme.breakpoints.up('sm')]: {
        width: `calc(${theme.spacing(8)} + 1px)`,
    },
});

const Drawer = styled(MuiDrawer, { shouldForwardProp: (prop) => prop !== 'open' })(
    ({ theme, open }) => ({
        width: drawerWidth,
        flexShrink: 0,
        whiteSpace: 'nowrap',
        boxSizing: 'border-box',
        ...(open && {
            ...openedMixin(theme),
            '& .MuiDrawer-paper': openedMixin(theme),
        }),
        ...(!open && {
            ...closedMixin(theme),
            '& .MuiDrawer-paper': closedMixin(theme),
        }),
    }),
);


export default function ClippedDrawer() {
    const navigate = useNavigate();
    const drawerOpen = useSelector(appStore, (state) => state.context.drawerOpen);

    const DrawerListItem = ({ icon, label, href }) => (
        <ListItem disablePadding>
            <ListItemButton onClick={() => { navigate(href) }} >
                <ListItemIcon>
                    {icon}
                </ListItemIcon>
                <ListItemText primary={label} />
            </ListItemButton>
        </ListItem>
    )
    return (
        <Drawer
            variant="permanent"
            open={drawerOpen}
        >
            <Toolbar />
            <Box sx={{ overflow: 'auto' }}>
                <List>
                    <DrawerListItem href="/" label="Home" icon={<HomeIcon />} />
                    <DrawerListItem href="/matrix" label="Availability Matrix" icon={<AccountTreeIcon />} />
                    <DrawerListItem href="/jurisdictions" label="Jurisdictions" icon={<PublicIcon />} />
                    <DrawerListItem href="/cells" label="Cells" icon={<AppsIcon />} />
                    <DrawerListItem href="/about" label="About" icon={<InfoIcon />} />
                </List>
            </Box>
        </Drawer>
    );
}
