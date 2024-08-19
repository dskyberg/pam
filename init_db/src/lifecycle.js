import { GRAPHQL_URL, query } from "./graphql.js";

const LIFECYCLES = ["BETA", "LEA", "EA", "GA", "EOL"];

const graphql_query = `mutation AddLifecycle($name: String!) {
  createLifecycle(name: $name) {
    id
    name
  }
}`;

export async function add() {
  for (let name of LIFECYCLES) {
    await query(graphql_query, "AddLifecycle", { name });
  }
}
