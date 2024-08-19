import * as React from 'react';
import packageJson from '../../package.json';

import AppBar from '@mui/material/AppBar';
import Toolbar from '@mui/material/Toolbar';
import Typography from '@mui/material/Typography';
import IconButton from '@mui/material/IconButton';
import MenuIcon from '@mui/icons-material/Menu';
import AccountCircle from '@mui/icons-material/AccountCircle';
import LoginIcon from '@mui/icons-material/Login';
import MenuItem from '@mui/material/MenuItem';
import Menu from '@mui/material/Menu';
import { useSelector } from '@xstate/store/react'
import { appStore } from '../state/appStore';


export default function MenuAppBar() {
    const auth = useSelector(appStore, (state) => state.context.auth);

    const [anchorEl, setAnchorEl] = React.useState(null);

    const handleMenu = (event) => {
        setAnchorEl(event.currentTarget);
    };

    const handleClose = () => {
        setAnchorEl(null);
    };

    const handleDrawerToggle = () => {
        appStore.send({ type: 'toggleDrawer' })
    }

    const handleAuthToggle = () => {
        setAnchorEl(null);
        appStore.send({ type: 'authenticate' })
    }

    return (
        <AppBar position="fixed" sx={{ zIndex: (theme) => theme.zIndex.drawer + 1 }}>
            <Toolbar>
                <IconButton
                    size="large"
                    edge="start"
                    color="inherit"
                    aria-label="menu"
                    sx={{ mr: 2 }}
                    onClick={handleDrawerToggle}
                >
                    <MenuIcon />
                </IconButton>
                <Typography variant="h6" component="div" sx={{ flexGrow: 1 }}>
                    {packageJson.displayName}
                </Typography>
                {!auth && (
                    <div>
                        <IconButton
                            size="large"
                            aria-label="loginr"
                            onClick={handleAuthToggle}
                            color="inherit"
                        >
                            <LoginIcon />
                        </IconButton>

                    </div>
                )}
                {auth && (
                    <div>
                        <IconButton
                            size="large"
                            aria-label="account of current user"
                            aria-controls="menu-appbar"
                            aria-haspopup="true"
                            onClick={handleMenu}
                            color="inherit"
                        >
                            <AccountCircle />
                        </IconButton>
                        <Menu
                            id="menu-appbar"
                            anchorEl={anchorEl}
                            anchorOrigin={{
                                vertical: 'top',
                                horizontal: 'right',
                            }}
                            keepMounted
                            transformOrigin={{
                                vertical: 'top',
                                horizontal: 'right',
                            }}
                            open={Boolean(anchorEl)}
                            onClose={handleClose}
                        >
                            <MenuItem onClick={handleClose}>Profile</MenuItem>
                            <MenuItem onClick={handleClose}>My account</MenuItem>
                            <MenuItem onClick={handleAuthToggle}>Log out</MenuItem>
                        </Menu>
                    </div>
                )}
            </Toolbar>
        </AppBar >
    );
}
