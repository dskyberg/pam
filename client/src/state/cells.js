import { fromPromise, createActor, assign, setup } from 'xstate'
import fetchFromGraph from './fetch';

const OPERATION_NAME = 'Cells';
const QUERY = `query Cells($pageSize: Int, $page: Int) {
    cells(pageSize: $pageSize, page: $page) {
        id
        name
        csp
        country
        region
        cspRegion
  }
}`;

export const machine = setup({
    actions: {
        setPageDimensions: assign({
            pageSize: ({ event }) => {
                console.log('setPageDimensions:', event);
                return event.event.pageSize
            },
            page: ({ event }) => event.event.page
        }),
    },
    actors: {
        fetchData: fromPromise(async ({ input }) => fetchFromGraph(QUERY, OPERATION_NAME, input.pageSize, input.page))
    }
}).createMachine({
    /** @xstate-layout N4IgpgJg5mDOIC5QGMwBs2wHQEsJrAGIAxAUQBUBhACQG0AGAXUVAAcB7WHAFx3YDsWIAB6IAjABYAbFgkBmOQFYpkgJxLVigExaANCACeiABxiscnaq30LYxQHYFigL7P9qDNjwFCAZQoA+gAKAIIA4qQBACIAkgCypAByvjEA8skMzEggHFy8AkKiCJIy8koqEuqKmjr6RsX2MtWNEjqKxvZaqsbGru7omFho7ACGEDj8UIQQAmC4-ABu7ADWcx6Dw2MTUAgTS8gj+fyZmUK5PHyC2UXtxlhayhL29mL02sZaEnWIWnKqWCp2ooxFIpB0XnJ7H0QOtsJtxpNCGAAE7I9jIrCsNCHABm6IAtlhYUNRgidnt2AcjicmGdOBcCtcfnYsOpfvR6HZOaopHpDD8XlgOr8pBIxJoeqLocScSMcGgAK7IogAJQoKoAmqdsucjoVxNJZE4KlUanz6nIxHcpDypPQedUxPYJBJXG4QPx2BA4EJYXS8pd9QgALRSb4hxSs1TR4z0LQ29qQyHSgZefBgf0Mq6gG72LDPaTdZ4fRz2VTh8VyLD0ezveg9bRPXru4nw7aZvVMhCtGRvF1iHSVQdicOfGS8yrCqSKVoSYxyFOeLCwBXIVCweA6+mdnMmQXOuRPB52uTGKT2Ud2rCKGvPBxiSGPReDWXypUZrcBxm77vmxCW+h8w+VR7DeeNPjdZwgA */
    id: OPERATION_NAME,
    initial: 'ready',
    context: {
        data: null,
        page: 0,
        pageSize: 25,
        error: null
    },
    states: {
        loading: {
            invoke: {
                src: 'fetchData',
                input: ({ context: { pageSize, page } }) => ({ pageSize, page }),
                onDone: {
                    target: 'ready',
                    actions: assign({
                        data: ({ event }) => {
                            console.log('loading onDone:', event)
                            return event.output
                        }
                    }),
                },
                onError: {
                    target: 'failure',
                    actions: assign({
                        error: ({ event }) => event.output,
                    })
                },
            },
        },
        ready: {
            on: {
                SET_PAGE_DIMENSIONS: {
                    target: 'loading',
                    actions: 'setPageDimensions',
                },
                FETCH: {
                    target: 'loading'
                },
            }
        },
        failure: {
            on: {
                RETRY: 'loading',
            },
        },
    },
});

export const actor = createActor(machine);
actor.start();

