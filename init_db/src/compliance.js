import { GRAPHQL_URL, query } from "./graphql.js";

const COMPLIANCES = ["Not Available", "Available", "Audit Ready", "Authorized"];

const graphql_query = `mutation AddCompliance($name: String!) {
  createCompliance(name: $name) {
    id
    name
  }
}`;

export async function add() {
  for (let name of COMPLIANCES) {
    await query(graphql_query, "AddCompliance", { name });
  }
}
