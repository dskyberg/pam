
import { useSelector } from '@xstate/react';
import { appStore } from '../state/appStore';

import { useQuery } from '@apollo/client';
import { GET_MATRIX } from '../graph';

import { styled } from '@mui/material/styles';
import { CircularProgress, Drawer as MuiDrawer, Toolbar } from "@mui/material";

import { AvailabilityPanel } from './AvailabilityPanel'
import { ProductPanel } from './ProductPanel';
import { FeaturePanel } from './FeaturePanel'

const drawerWidth = 340;

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
    width: 0,
    [theme.breakpoints.up('sm')]: {
        width: 0,
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

export default function PanelDrawer() {
    const activeItem = useSelector(appStore, (state) => state.context.activeItem);
    const { loading, error, data } = useQuery(GET_MATRIX);

    const open = true

    if (activeItem == undefined || activeItem == null)
        return null

    if (loading) {
        return <CircularProgress />
    }
    if (error) {
        return (
            <pre>{JSON.stringify(error, null, 4)}</pre>
        )
    }

    return (
        <Drawer
            open={open}
            anchor={"right"}
            variant="permanent"
            sx={{ display: "flex", overflow: "hidden", paddingLeft: "1em" }}
        >
            <Toolbar />

            {activeItem.__typename == 'Product' && <ProductPanel item={activeItem} />}
            {activeItem.__typename == 'Feature' && <FeaturePanel item={activeItem} />}
            {activeItem.__typename == 'Availability' && <AvailabilityPanel item={activeItem} matrix={data.matrix} />}

        </Drawer >

    )
}


