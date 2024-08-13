/*
    Global state, shared between appbar, drawer, and other components.
*/
import { createStore } from '@xstate/store';

export const appStore = createStore(
    {
        auth: false,
        drawerOpen: false
    },
    {
        authenticate: {
            auth: (context) => !context.auth,
        },
        toggleDrawer: {
            drawerOpen: (context) => !context.drawerOpen,
        }
    }
);

