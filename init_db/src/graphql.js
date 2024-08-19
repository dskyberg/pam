export const GRAPHQL_URL = "http://localhost:8080/graph/graphql";

export async function query(query, operationName, variables) {
  let data = { query, operationName, variables };
  return fetch(GRAPHQL_URL, {
    method: "POST",
    headers: {
      "Content-Type": "application/json",
    },
    body: JSON.stringify(data),
  })
    .then((response) => response.json())
    .then((data) => {
      if (data.errors !== undefined) {
        throw data.errors;
      }
      return data.data;
    });
}
