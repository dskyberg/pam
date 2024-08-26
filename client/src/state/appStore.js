/*
    Global state, shared between appbar, drawer, and other components.
*/
//import { createStore } from '@xstate/store';
import { fromStore } from '@xstate/store';
import { createActor } from 'xstate';

export const storeLogic = fromStore(
    {
        auth: false,
        drawerOpen: false,
        panelOpen: true,
        activeItem: null,
    },
    {
        authenticate: {
            auth: (context) => !context.auth,
        },
        toggleDrawer: {
            drawerOpen: (context) => !context.drawerOpen,
        },
        togglePanel: {
            panelOpen: (context) => !context.panelOpen,
        },
        setActiveItem: {
            activeItem: (context, event) => {
                // Clear out if already selected.
                if (context.activeItem?.id == event.item?.id) { return null }
                return event.item;
            }
        }
    }
);

export const appStore = createActor(storeLogic);
appStore.start();
