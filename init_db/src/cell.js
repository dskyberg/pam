import { GRAPHQL_URL, query } from "./graphql.js";

const CELLS = [
  {
    name: "OK1",
    csp: "AWS",
    region: "AMER",
    country: "USA",
    cspRegion: "us-east-1",
    jurisdiction: "aws_nac",
  },
  {
    name: "OK2",
    csp: "AWS",
    region: "AMER",
    country: "USA",
    cspRegion: "us-east-1",
    jurisdiction: "aws_nac",
  },
  {
    name: "OK3",
    csp: "AWS",
    region: "AMER",
    country: "USA",
    cspRegion: "us-east-1",
    jurisdiction: "aws_nac",
  },
  {
    name: "OK4",
    csp: "AWS",
    region: "AMER",
    country: "USA",
    cspRegion: "us-east-1",
    jurisdiction: "aws_nac",
  },
  {
    name: "OK6",
    csp: "AWS",
    region: "AMER",
    country: "USA",
    cspRegion: "us-east-2",
    jurisdiction: "aws_nac",
  },
  {
    name: "OK11",
    csp: "AWS",
    region: "AMER",
    country: "USA",
    cspRegion: "us-east-2",
    jurisdiction: "aws_nac",
  },
  {
    name: "OK7",
    csp: "AWS",
    region: "AMER",
    country: "USA",
    cspRegion: "us-west-2",
    jurisdiction: "aws_nac",
  },
  {
    name: "OK12",
    csp: "AWS",
    region: "AMER",
    country: "USA",
    cspRegion: "us-west-2",
    jurisdiction: "aws_nac",
  },
  {
    name: "OK14",
    csp: "AWS",
    region: "AMER",
    country: "USA",
    cspRegion: "us-west-2",
    jurisdiction: "aws_nac",
  },
  {
    name: "OK9",
    csp: "AWS",
    region: "EMEA",
    country: "IRE",
    cspRegion: "eu-west-1",
    jurisdiction: "aws_emea",
  },
  {
    name: "EU1",
    csp: "AWS",
    region: "EMEA",
    country: "GER",
    cspRegion: "eu-central-1",
    jurisdiction: "aws_emea",
  },
  {
    name: "OK5",
    csp: "AWS",
    region: "AMER",
    country: "USA",
    cspRegion: "us-west-2",
    jurisdiction: "aws_frm",
  },
  {
    name: "OK10",
    csp: "AWS",
    region: "AMER",
    country: "USA",
    cspRegion: "us-east-2",
    jurisdiction: "aws_frm",
  },
  {
    name: "OG1",
    csp: "AWS",
    region: "AMER",
    country: "USA",
    cspRegion: "us-gov-west-1",
    jurisdiction: "aws_frh",
  },
  {
    name: "OM1",
    csp: "AWS",
    region: "AMER",
    country: "USA",
    cspRegion: "us-gov-east-1",
    jurisdiction: "aws_il4",
  },
  {
    name: "OK8",
    csp: "AWS",
    region: "APJ",
    country: "AUS",
    cspRegion: "ap-southeast-2",
    jurisdiction: "aws_apj",
  },
  {
    name: "OK16",
    csp: "AWS",
    region: "APJ",
    country: "JAP",
    cspRegion: "ap-northeast-1",
    jurisdiction: "aws_apj",
  },
  {
    name: "OK17",
    csp: "GCP",
    region: "AMER",
    country: "USA",
    cspRegion: "us-east-1",
    jurisdiction: "gcp_nac",
  },
];

const graphql_query = `mutation AddCell($input: CellInput!) {
  createCell(input: $input) {
    id
    name

  }
}`;

export async function add() {
  for (let input of CELLS) {
    await query(graphql_query, "AddCell", { input });
  }
}
