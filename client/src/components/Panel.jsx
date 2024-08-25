
import { useSelector } from '@xstate/react';
import { appStore } from '../state/appStore';

import { useQuery, useMutation } from '@apollo/client';
import { GET_MATRIX, GET_FULL_MATRIX } from '../graph';

import { styled } from '@mui/material/styles';
import { CircularProgress, Drawer as MuiDrawer, Toolbar, Typography, Stack } from "@mui/material";
import { FormControl, InputLabel, Select, MenuItem } from "@mui/material";
import Comments from "./Comments";
import { UPDATE_AVAILABILITY } from '../graph';


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

export default function Panel() {
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
            sx={{ overflow: "hidden", paddingLeft: "1em" }}
        >
            <Toolbar />
            <div >
                {activeItem.__typename == 'Product' && <Product item={activeItem} />}
                {activeItem.__typename == 'Feature' && <Feature item={activeItem} />}
                {activeItem.__typename == 'Availability' && <Availability item={activeItem} matrix={data.matrix} />}
            </div>
        </Drawer >

    )
}

function Product({ item }) {

    return (
        <div sx={{ display: "flex" }}>
            <Typography variant="h6">Product</Typography>
            <Typography >{item.name}</Typography>
            <Typography >{item.id}</Typography>
        </div>
    )
}


function Feature({ item }) {
    return (
        <div sx={{ display: "flex" }}>
            <Typography variant="h6">Feature</Typography>
            <Typography >{item.name}</Typography>
            <Typography >{item.id}</Typography>
        </div>
    )
}

function Availability({ item, matrix }) {

    const [updateAvailability, updateResult] = useMutation(UPDATE_AVAILABILITY,
        {
            refetchQueries: [
                GET_FULL_MATRIX, // DocumentNode object parsed with gql
                'GetFullMatrix' // Query name
            ]
        },
    );


    const handleLifecycleChange = (event) => {
        updateAvailability({
            variables: {
                input: {
                    id: item.id,
                    lifecycle: event.target.value
                }
            }
        }).then(() => {
            console.log("Changed lifecycle to", event.target.value)
        }).catch((err) => console.log("Got an Apollo Error:", err.message))
    };

    const handleComplianceChange = (event) => {
        updateAvailability({
            variables: {
                input: {
                    id: item.id,
                    compliance: event.target.value
                }
            }
        }).then(() => {
            console.log("Changed Compliance to", event.target.value)
        }).catch((err) => console.log("Got an Apollo Error:", err.message))
    };

    const Jurisdiction = () => {
        const jurisdiction = matrix.jurisdictions.find(j => j.id == item.jurisdiction.id);
        return (
            <Typography>Jurisdiction: {jurisdiction.name}</Typography>
        )
    }

    return (
        <Stack sx={{ paddingLeft: "1em", paddingRight: "1em" }}>
            <Typography variant="h6">Availability</Typography>
            <Typography >{item.id}</Typography>
            <Jurisdiction />
            <Typography >Updated: {item.lastUpdated}</Typography>

            <FormControl variant="standard" fullWidth sx={{ margin: '1em' }}>
                <InputLabel id="lifecycle-label">Lifecycle</InputLabel>
                <Select
                    labelId="lifecycle-label"
                    id="lifecycle"
                    value={item.lifecycle.name}
                    label="Lifecycle"
                    onChange={handleLifecycleChange}
                >
                    {matrix.lifecycles.map((lifecycle) => (<MenuItem key={lifecycle.id} value={lifecycle.name}>{lifecycle.description}</MenuItem>))}
                </Select>
            </FormControl>

            <FormControl variant="standard" fullWidth sx={{ margin: '1em' }}>
                <InputLabel id="compliance-label">Compliance</InputLabel>
                <Select
                    labelId="compliance-label"
                    id="compliance"
                    value={item.compliance.name}
                    label="Compliance"
                    onChange={handleComplianceChange}
                >
                    {matrix.compliances.map(compliance => (<MenuItem key={compliance.id} value={compliance.name}>{compliance.name}</MenuItem>))}
                </Select>
            </FormControl>
            <Comments itemId={item.id} />
        </Stack>
    );
}
