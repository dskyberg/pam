import { GRAPHQL_URL, query } from "./graphql.js";

const JURISDICTIONS = [
  { name: "aws_nac", title: "North America Commercial" },
  { name: "aws_frm", title: "Okta for Government Moderate & HIPAA" },
  { name: "aws_frh", title: "Okta for Government High" },
  { name: "aws_il4", title: "Okta for U.S. Military" },
  { name: "aws_emea", title: "EMEA" },
  { name: "aws_apj", title: "APJ" },
  { name: "gcp_nac", title: "GCP North America Commercial" },
];

const graphql_query = `mutation AddJurisdictions($input: JurisdictionInput!) {
  createJurisdiction(input: $input) {
    id
    name
  }
}`;

export async function add() {
  for (let input of JURISDICTIONS) {
    await query(graphql_query, "AddJurisdictions", { input });
  }
}
