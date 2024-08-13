const GRAPH_URL = 'http://localhost:8080/api/graphql';



export default function fetchFromGraph(query, operationName, pageSize, page) {

    const graph_query = { query, operationName, variables: { pageSize, page: page + 1 } };
    console.log('fetchFromGraph:', graph_query);

    return fetch(`${GRAPH_URL}`, {
        method: "post",
        headers: {
            'Accept': 'application/json',
            'Content-Type': 'application/json'
        },
        body: JSON.stringify(graph_query)
    })
        .then(blob => blob.json())
        .then(data => {
            console.log('fetch:', data)
            return data.data
        })
        .catch((error) => {
            console.log('fetch error:', error);
            return error
        })
}