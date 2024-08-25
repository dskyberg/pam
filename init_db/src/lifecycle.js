import { GRAPHQL_URL, query } from "./graphql.js";

const LIFECYCLES = [
  { name: "BETA", description: "Beta" },
  { name: "LEA", description: "Limited Early Access" },
  { name: "EA", description: "Early Access" },
  { name: "GA", description: "Generally Available" },
  { name: "EOL", description: "End of Life" },
];

const graphql_query = `mutation AddLifecycle($name: String!, $description: String!) {
  createLifecycle(name: $name, description: $description) {
    id
    name
    description
  }
}`;

export async function add() {
  for (let lifecycle of LIFECYCLES) {
    await query(graphql_query, "AddLifecycle", lifecycle);
  }
}
