import { GRAPHQL_URL, query } from "./graphql.js";
const CATEGORIES = [
  {
    name: "Identity Governance",
    products: [
      {
        name: "Universal Directory",
        comments: [{ text: "This is a product comment!", createdBy: "davaid.skyberg@okta.com" }],
        availability: [
          {
            jurisdiction: "aws_nac",
            lifecycle: "GA",
            compliance: "Available",
            comments: [
              { text: "This is the first comment", createdBy: "david.skyberg@okta.com" },
              { text: "This is the second comment", createdBy: "david.skyberg@okta.com" },
            ],
          },
          { jurisdiction: "aws_frm", lifecycle: "GA", compliance: "Available" },
          { jurisdiction: "aws_frh", lifecycle: "GA", compliance: "Available" },
          { jurisdiction: "aws_il4", lifecycle: "GA", compliance: "Available" },
          { jurisdiction: "aws_emea", lifecycle: "GA", compliance: "Available" },
          { jurisdiction: "aws_apj", lifecycle: "GA", compliance: "Available" },
          { jurisdiction: "gcp_nac", lifecycle: "EA", compliance: "Not Available" },
        ],
        features: [
          {
            name: "Kerberos: Integrated Windows Auth",
            availability: [
              { jurisdiction: "aws_nac", lifecycle: "GA", compliance: "Available" },
              { jurisdiction: "aws_frm", lifecycle: "GA", compliance: "Available" },
              { jurisdiction: "aws_frh", lifecycle: "GA", compliance: "Available" },
              { jurisdiction: "aws_il4", lifecycle: "GA", compliance: "Not Available" },
              { jurisdiction: "aws_emea", lifecycle: "GA", compliance: "Available" },
              { jurisdiction: "aws_apj", lifecycle: "GA", compliance: "Available" },
              { jurisdiction: "gcp_nac", lifecycle: "EA", compliance: "Not Available" },
            ],
          },
          {
            name: "Kerberos: Silent Activation",
            availability: [
              { jurisdiction: "aws_nac", lifecycle: "GA", compliance: "Available" },
              { jurisdiction: "aws_frm", lifecycle: "GA", compliance: "Available" },
              { jurisdiction: "aws_frh", lifecycle: "GA", compliance: "Available" },
              { jurisdiction: "aws_il4", lifecycle: "GA", compliance: "Not Available" },
              { jurisdiction: "aws_emea", lifecycle: "GA", compliance: "Available" },
              { jurisdiction: "aws_apj", lifecycle: "GA", compliance: "Available" },
              { jurisdiction: "gcp_nac", lifecycle: "EA", compliance: "Not Available" },
            ],
          },
          {
            name: "Kerberos: Agentless Desktop SSO",
            availability: [
              { jurisdiction: "aws_nac", lifecycle: "GA", compliance: "Available" },
              { jurisdiction: "aws_frm", lifecycle: "GA", compliance: "Available" },
              { jurisdiction: "aws_frh", lifecycle: "GA", compliance: "Available" },
              { jurisdiction: "aws_il4", lifecycle: "GA", compliance: "Not Available" },
              { jurisdiction: "aws_emea", lifecycle: "GA", compliance: "Available" },
              { jurisdiction: "aws_apj", lifecycle: "GA", compliance: "Available" },
              { jurisdiction: "gcp_nac", lifecycle: "EA", compliance: "Not Available" },
            ],
          },
          {
            name: "AD Del Auth",
            availability: [
              { jurisdiction: "aws_nac", lifecycle: "GA", compliance: "Available" },
              { jurisdiction: "aws_frm", lifecycle: "GA", compliance: "Available" },
              { jurisdiction: "aws_frh", lifecycle: "GA", compliance: "Available" },
              { jurisdiction: "aws_il4", lifecycle: "GA", compliance: "Available" },
              { jurisdiction: "aws_emea", lifecycle: "GA", compliance: "Available" },
              { jurisdiction: "aws_apj", lifecycle: "GA", compliance: "Available" },
              { jurisdiction: "gcp_nac", lifecycle: "EA", compliance: "Not Available" },
            ],
          },
          {
            name: "LDAP Interface",
            availability: [
              { jurisdiction: "aws_nac", lifecycle: "GA", compliance: "Available" },
              { jurisdiction: "aws_frm", lifecycle: "GA", compliance: "Available" },
              { jurisdiction: "aws_frh", lifecycle: "GA", compliance: "Available" },
              { jurisdiction: "aws_il4", lifecycle: "GA", compliance: "Available" },
              { jurisdiction: "aws_emea", lifecycle: "GA", compliance: "Available" },
              { jurisdiction: "aws_apj", lifecycle: "GA", compliance: "Available" },
              { jurisdiction: "gcp_nac", lifecycle: "EA", compliance: "Not Available" },
            ],
          },
        ],
      },
      {
        name: "Lifecycle Management",
        availability: [
          { jurisdiction: "aws_nac", lifecycle: "GA", compliance: "Available" },
          { jurisdiction: "aws_frm", lifecycle: "GA", compliance: "Available" },
          { jurisdiction: "aws_frh", lifecycle: "GA", compliance: "Available" },
          { jurisdiction: "aws_il4", lifecycle: "GA", compliance: "Available" },
          { jurisdiction: "aws_emea", lifecycle: "GA", compliance: "Available" },
          { jurisdiction: "aws_apj", lifecycle: "GA", compliance: "Available" },
          { jurisdiction: "gcp_nac", lifecycle: "EA", compliance: "Not Available" },
        ],
      },
      {
        name: "Secure Partner Access",
        availability: [
          { jurisdiction: "aws_nac", lifecycle: "GA", compliance: "Available" },
          { jurisdiction: "aws_frm", lifecycle: "GA", compliance: "Available" },
          { jurisdiction: "aws_frh", lifecycle: "GA", compliance: "Available" },
          { jurisdiction: "aws_il4", lifecycle: "GA", compliance: "Available" },
          { jurisdiction: "aws_emea", lifecycle: "GA", compliance: "Available" },
          { jurisdiction: "aws_apj", lifecycle: "GA", compliance: "Available" },
          { jurisdiction: "gcp_nac", lifecycle: "EA", compliance: "Not Available" },
        ],
      },
      {
        name: "Okta Identity Governance",
        availability: [
          { jurisdiction: "aws_nac", lifecycle: "GA", compliance: "Available" },
          { jurisdiction: "aws_frm", lifecycle: "GA", compliance: "Available" },
          { jurisdiction: "aws_frh", lifecycle: "GA", compliance: "Available" },
          { jurisdiction: "aws_il4", lifecycle: "GA", compliance: "Available" },
          { jurisdiction: "aws_emea", lifecycle: "GA", compliance: "Available" },
          { jurisdiction: "aws_apj", lifecycle: "GA", compliance: "Available" },
          { jurisdiction: "gcp_nac", lifecycle: "EA", compliance: "Not Available" },
        ],
      },
    ],
  },
  {
    name: "Access Management",
    products: [
      {
        name: "Single Sign-On",
        availability: [
          { jurisdiction: "aws_nac", lifecycle: "GA", compliance: "Available" },
          { jurisdiction: "aws_frm", lifecycle: "GA", compliance: "Available" },
          { jurisdiction: "aws_frh", lifecycle: "GA", compliance: "Available" },
          { jurisdiction: "aws_il4", lifecycle: "GA", compliance: "Available" },
          { jurisdiction: "aws_emea", lifecycle: "GA", compliance: "Available" },
          { jurisdiction: "aws_apj", lifecycle: "GA", compliance: "Available" },
          { jurisdiction: "gcp_nac", lifecycle: "EA", compliance: "Not Available" },
        ],
        features: [
          {
            name: "FastPass",
            availability: [
              { jurisdiction: "aws_nac", lifecycle: "GA", compliance: "Available" },
              { jurisdiction: "aws_frm", lifecycle: "GA", compliance: "Available" },
              { jurisdiction: "aws_frh", lifecycle: "GA", compliance: "Available" },
              { jurisdiction: "aws_il4", lifecycle: "GA", compliance: "Available" },
              { jurisdiction: "aws_emea", lifecycle: "GA", compliance: "Available" },
              { jurisdiction: "aws_apj", lifecycle: "GA", compliance: "Available" },
              { jurisdiction: "gcp_nac", lifecycle: "EA", compliance: "Not Available" },
            ],
          },
          {
            name: "O365",
            availability: [
              { jurisdiction: "aws_nac", lifecycle: "GA", compliance: "Available" },
              { jurisdiction: "aws_frm", lifecycle: "GA", compliance: "Available" },
              { jurisdiction: "aws_frh", lifecycle: "GA", compliance: "Available" },
              { jurisdiction: "aws_il4", lifecycle: "GA", compliance: "Available" },
              { jurisdiction: "aws_emea", lifecycle: "GA", compliance: "Available" },
              { jurisdiction: "aws_apj", lifecycle: "GA", compliance: "Available" },
              { jurisdiction: "gcp_nac", lifecycle: "EA", compliance: "Not Available" },
            ],
          },
          {
            name: "O365 GCC High (Profile Sync and License and Role Management)",
            availability: [
              { jurisdiction: "aws_nac", lifecycle: "GA", compliance: "Not Available" },
              { jurisdiction: "aws_frm", lifecycle: "GA", compliance: "Not Available" },
              { jurisdiction: "aws_frh", lifecycle: "GA", compliance: "Available" },
              { jurisdiction: "aws_il4", lifecycle: "GA", compliance: "Available" },
              { jurisdiction: "aws_emea", lifecycle: "GA", compliance: "Not Available" },
              { jurisdiction: "aws_apj", lifecycle: "GA", compliance: "Not Available" },
              { jurisdiction: "gcp_nac", lifecycle: "EA", compliance: "Not Available" },
            ],
          },
          {
            name: "O365 GCC High (User and Universal Sync)",
            availability: [
              { jurisdiction: "aws_nac", lifecycle: "GA", compliance: "Not Available" },
              { jurisdiction: "aws_frm", lifecycle: "GA", compliance: "Available" },
              { jurisdiction: "aws_frh", lifecycle: "GA", compliance: "Available" },
              { jurisdiction: "aws_il4", lifecycle: "GA", compliance: "Not Available" },
              { jurisdiction: "aws_emea", lifecycle: "GA", compliance: "Not Available" },
              { jurisdiction: "aws_apj", lifecycle: "GA", compliance: "Not Available" },
              { jurisdiction: "gcp_nac", lifecycle: "EA", compliance: "Not Available" },
            ],
          },
          {
            name: "O365 DOD",
            availability: [
              { jurisdiction: "aws_nac", lifecycle: "GA", compliance: "Not Available" },
              { jurisdiction: "aws_frm", lifecycle: "GA", compliance: "Not Available" },
              { jurisdiction: "aws_frh", lifecycle: "GA", compliance: "Not Available" },
              { jurisdiction: "aws_il4", lifecycle: "GA", compliance: "Not Available" },
              { jurisdiction: "aws_emea", lifecycle: "GA", compliance: "Not Available" },
              { jurisdiction: "aws_apj", lifecycle: "GA", compliance: "Not Available" },
              { jurisdiction: "gcp_nac", lifecycle: "EA", compliance: "Not Available" },
            ],
          },
        ],
      },
      {
        name: "Adaptive MFA",
        features: [
          {
            name: "Device Trust",
            availability: [
              { jurisdiction: "aws_nac", lifecycle: "GA", compliance: "Available" },
              { jurisdiction: "aws_frm", lifecycle: "GA", compliance: "Available" },
              { jurisdiction: "aws_frh", lifecycle: "GA", compliance: "Available" },
              { jurisdiction: "aws_il4", lifecycle: "GA", compliance: "Available" },
              { jurisdiction: "aws_emea", lifecycle: "GA", compliance: "Available" },
              { jurisdiction: "aws_apj", lifecycle: "GA", compliance: "Available" },
              { jurisdiction: "gcp_nac", lifecycle: "EA", compliance: "Not Available" },
            ],
          },
          {
            name: "Desktop MFA",
            availability: [
              { jurisdiction: "aws_nac", lifecycle: "GA", compliance: "Available" },
              { jurisdiction: "aws_frm", lifecycle: "GA", compliance: "Available" },
              { jurisdiction: "aws_frh", lifecycle: "GA", compliance: "Available" },
              { jurisdiction: "aws_il4", lifecycle: "GA", compliance: "Available" },
              { jurisdiction: "aws_emea", lifecycle: "GA", compliance: "Available" },
              { jurisdiction: "aws_apj", lifecycle: "GA", compliance: "Available" },
              { jurisdiction: "gcp_nac", lifecycle: "EA", compliance: "Not Available" },
            ],
          },
          {
            name: "Policy Recommender with Okta AI",
            availability: [
              { jurisdiction: "aws_nac", lifecycle: "GA", compliance: "Available" },
              { jurisdiction: "aws_frm", lifecycle: "GA", compliance: "Available" },
              { jurisdiction: "aws_frh", lifecycle: "GA", compliance: "Available" },
              { jurisdiction: "aws_il4", lifecycle: "GA", compliance: "Available" },
              { jurisdiction: "aws_emea", lifecycle: "GA", compliance: "Available" },
              { jurisdiction: "aws_apj", lifecycle: "GA", compliance: "Available" },
              { jurisdiction: "gcp_nac", lifecycle: "EA", compliance: "Not Available" },
            ],
          },
          {
            name: "Pre-enrolled Yubikeys",
            availability: [
              { jurisdiction: "aws_nac", lifecycle: "GA", compliance: "Available" },
              { jurisdiction: "aws_frm", lifecycle: "GA", compliance: "Not Available" },
              { jurisdiction: "aws_frh", lifecycle: "GA", compliance: "Not Available" },
              { jurisdiction: "aws_il4", lifecycle: "GA", compliance: "Not Available" },
              { jurisdiction: "aws_emea", lifecycle: "GA", compliance: "Available" },
              { jurisdiction: "aws_apj", lifecycle: "GA", compliance: "Available" },
              { jurisdiction: "gcp_nac", lifecycle: "EA", compliance: "Not Available" },
            ],
          },
          {
            name: "SMS / Voice Authenticator",
            availability: [
              { jurisdiction: "aws_nac", lifecycle: "GA", compliance: "Available" },
              { jurisdiction: "aws_frm", lifecycle: "GA", compliance: "Available" },
              { jurisdiction: "aws_frh", lifecycle: "GA", compliance: "Available" },
              { jurisdiction: "aws_il4", lifecycle: "GA", compliance: "Available" },
              { jurisdiction: "aws_emea", lifecycle: "GA", compliance: "Available" },
              { jurisdiction: "aws_apj", lifecycle: "GA", compliance: "Available" },
              { jurisdiction: "gcp_nac", lifecycle: "EA", compliance: "Available" },
            ],
          },
          {
            name: "Email Authenticator",
            availability: [
              { jurisdiction: "aws_nac", lifecycle: "GA", compliance: "Available" },
              { jurisdiction: "aws_frm", lifecycle: "GA", compliance: "Available" },
              { jurisdiction: "aws_frh", lifecycle: "GA", compliance: "Available" },
              { jurisdiction: "aws_il4", lifecycle: "GA", compliance: "Available" },
              { jurisdiction: "aws_emea", lifecycle: "GA", compliance: "Available" },
              { jurisdiction: "aws_apj", lifecycle: "GA", compliance: "Available" },
              { jurisdiction: "gcp_nac", lifecycle: "EA", compliance: "Available" },
            ],
          },
          {
            name: "Okta Verify",
            availability: [
              { jurisdiction: "aws_nac", lifecycle: "GA", compliance: "Available" },
              { jurisdiction: "aws_frm", lifecycle: "GA", compliance: "Available" },
              { jurisdiction: "aws_frh", lifecycle: "GA", compliance: "Available" },
              { jurisdiction: "aws_il4", lifecycle: "GA", compliance: "Available" },
              { jurisdiction: "aws_emea", lifecycle: "GA", compliance: "Available" },
              { jurisdiction: "aws_apj", lifecycle: "GA", compliance: "Available" },
              { jurisdiction: "gcp_nac", lifecycle: "EA", compliance: "Not Available" },
            ],
          },
          {
            name: "Okta Mobile (on Classic)",
            availability: [
              { jurisdiction: "aws_nac", lifecycle: "GA", compliance: "Available" },
              { jurisdiction: "aws_frm", lifecycle: "GA", compliance: "Available" },
              { jurisdiction: "aws_frh", lifecycle: "GA", compliance: "Not Available" },
              { jurisdiction: "aws_il4", lifecycle: "GA", compliance: "Not Available" },
              { jurisdiction: "aws_emea", lifecycle: "GA", compliance: "Not Available" },
              { jurisdiction: "aws_apj", lifecycle: "GA", compliance: "Available" },
              { jurisdiction: "gcp_nac", lifecycle: "EA", compliance: "Not Available" },
            ],
          },
        ],
      },
      {
        name: "API Access Management",
        availability: [
          { jurisdiction: "aws_nac", lifecycle: "GA", compliance: "Available" },
          { jurisdiction: "aws_frm", lifecycle: "GA", compliance: "Available" },
          { jurisdiction: "aws_frh", lifecycle: "GA", compliance: "Available" },
          { jurisdiction: "aws_il4", lifecycle: "GA", compliance: "Available" },
          { jurisdiction: "aws_emea", lifecycle: "GA", compliance: "Available" },
          { jurisdiction: "aws_apj", lifecycle: "GA", compliance: "Available" },
          { jurisdiction: "gcp_nac", lifecycle: "EA", compliance: "Not Available" },
        ],
      },
      {
        name: "Okta Access Gateway",
        availability: [
          { jurisdiction: "aws_nac", lifecycle: "GA", compliance: "Available" },
          { jurisdiction: "aws_frm", lifecycle: "GA", compliance: "Available" },
          { jurisdiction: "aws_frh", lifecycle: "GA", compliance: "Available" },
          { jurisdiction: "aws_il4", lifecycle: "GA", compliance: "Available" },
          { jurisdiction: "aws_emea", lifecycle: "GA", compliance: "Available" },
          { jurisdiction: "aws_apj", lifecycle: "GA", compliance: "Available" },
          { jurisdiction: "gcp_nac", lifecycle: "EA", compliance: "Not Available" },
        ],
      },
    ],
  },
  {
    name: "Customer Identity",
    products: [
      {
        name: "Customer Identity Solution",
        availability: [
          { jurisdiction: "aws_nac", lifecycle: "GA", compliance: "Available" },
          { jurisdiction: "aws_frm", lifecycle: "GA", compliance: "Available" },
          { jurisdiction: "aws_frh", lifecycle: "GA", compliance: "Available" },
          { jurisdiction: "aws_il4", lifecycle: "GA", compliance: "Available" },
          { jurisdiction: "aws_emea", lifecycle: "GA", compliance: "Available" },
          { jurisdiction: "aws_apj", lifecycle: "GA", compliance: "Available" },
          { jurisdiction: "gcp_nac", lifecycle: "EA", compliance: "Not Available" },
        ],
      },
    ],
  },
  {
    name: "Platform",
    products: [
      {
        name: "Okta Workflows",
        availability: [
          { jurisdiction: "aws_nac", lifecycle: "GA", compliance: "Available" },
          { jurisdiction: "aws_frm", lifecycle: "GA", compliance: "Available" },
          { jurisdiction: "aws_frh", lifecycle: "GA", compliance: "Available" },
          { jurisdiction: "aws_il4", lifecycle: "GA", compliance: "Not Available" },
          { jurisdiction: "aws_emea", lifecycle: "GA", compliance: "Not Available" },
          { jurisdiction: "aws_apj", lifecycle: "GA", compliance: "Not Available" },
          { jurisdiction: "gcp_nac", lifecycle: "EA", compliance: "Not Available" },
        ],
      },
      {
        name: "Okta for Global 2000",
        features: [
          {
            name: "Org2Org",
            availability: [
              { jurisdiction: "aws_nac", lifecycle: "GA", compliance: "Available" },
              { jurisdiction: "aws_frm", lifecycle: "GA", compliance: "Available" },
              { jurisdiction: "aws_frh", lifecycle: "GA", compliance: "Available" },
              { jurisdiction: "aws_il4", lifecycle: "GA", compliance: "Available" },
              { jurisdiction: "aws_emea", lifecycle: "GA", compliance: "Available" },
              { jurisdiction: "aws_apj", lifecycle: "GA", compliance: "Available" },
              { jurisdiction: "gcp_nac", lifecycle: "EA", compliance: "Not Available" },
            ],
          },
          {
            name: "Ariel",
            availability: [
              { jurisdiction: "aws_nac", lifecycle: "GA", compliance: "Available" },
              { jurisdiction: "aws_frm", lifecycle: "GA", compliance: "Available" },
              { jurisdiction: "aws_frh", lifecycle: "GA", compliance: "Available" },
              { jurisdiction: "aws_il4", lifecycle: "GA", compliance: "Available" },
              { jurisdiction: "aws_emea", lifecycle: "GA", compliance: "Available" },
              { jurisdiction: "aws_apj", lifecycle: "GA", compliance: "Available" },
              { jurisdiction: "gcp_nac", lifecycle: "EA", compliance: "Not Available" },
            ],
          },
        ],
      },
      {
        name: "Multi Org Deployment",
        availability: [
          { jurisdiction: "aws_nac", lifecycle: "GA", compliance: "Available" },
          { jurisdiction: "aws_frm", lifecycle: "GA", compliance: "Available" },
          { jurisdiction: "aws_frh", lifecycle: "GA", compliance: "Available" },
          { jurisdiction: "aws_il4", lifecycle: "GA", compliance: "Available" },
          { jurisdiction: "aws_emea", lifecycle: "GA", compliance: "Available" },
          { jurisdiction: "aws_apj", lifecycle: "GA", compliance: "Available" },
          { jurisdiction: "gcp_nac", lifecycle: "EA", compliance: "Not Available" },
        ],
      },
      {
        name: "Enhanced Disaster Recovery",
        availability: [
          { jurisdiction: "aws_nac", lifecycle: "GA", compliance: "Available" },
          { jurisdiction: "aws_frm", lifecycle: "GA", compliance: "Not Available" },
          { jurisdiction: "aws_frh", lifecycle: "GA", compliance: "Not Available" },
          { jurisdiction: "aws_il4", lifecycle: "GA", compliance: "Not Available" },
          { jurisdiction: "aws_emea", lifecycle: "GA", compliance: "Not Available" },
          { jurisdiction: "aws_apj", lifecycle: "GA", compliance: "Not Available" },
          { jurisdiction: "gcp_nac", lifecycle: "EA", compliance: "Not Available" },
        ],
      },
      {
        name: "Dynamic Scale",
        availability: [
          { jurisdiction: "aws_nac", lifecycle: "GA", compliance: "Available" },
          { jurisdiction: "aws_frm", lifecycle: "GA", compliance: "Available" },
          { jurisdiction: "aws_frh", lifecycle: "GA", compliance: "Available" },
          { jurisdiction: "aws_il4", lifecycle: "GA", compliance: "Available" },
          { jurisdiction: "aws_emea", lifecycle: "GA", compliance: "Available" },
          { jurisdiction: "aws_apj", lifecycle: "GA", compliance: "Available" },
          { jurisdiction: "gcp_nac", lifecycle: "EA", compliance: "Not Available" },
        ],
      },
      {
        name: "Log Investigator with Okta AI",
        availability: [
          { jurisdiction: "aws_nac", lifecycle: "GA", compliance: "Available" },
          { jurisdiction: "aws_frm", lifecycle: "GA", compliance: "Not Available" },
          { jurisdiction: "aws_frh", lifecycle: "GA", compliance: "Not Available" },
          { jurisdiction: "aws_il4", lifecycle: "GA", compliance: "Not Available" },
          { jurisdiction: "aws_emea", lifecycle: "GA", compliance: "Available" },
          { jurisdiction: "aws_apj", lifecycle: "GA", compliance: "Available" },
          { jurisdiction: "gcp_nac", lifecycle: "EA", compliance: "Not Available" },
        ],
      },
      {
        name: "Standard Okta Offerings (No SKU)",
        features: [
          {
            name: "Custom Domains",
            availability: [
              { jurisdiction: "aws_nac", lifecycle: "GA", compliance: "Available" },
              { jurisdiction: "aws_frm", lifecycle: "GA", compliance: "Available" },
              { jurisdiction: "aws_frh", lifecycle: "GA", compliance: "Available" },
              { jurisdiction: "aws_il4", lifecycle: "GA", compliance: "Available" },
              { jurisdiction: "aws_emea", lifecycle: "GA", compliance: "Available" },
              { jurisdiction: "aws_apj", lifecycle: "GA", compliance: "Available" },
              { jurisdiction: "gcp_nac", lifecycle: "EA", compliance: "Not Available" },
            ],
          },
          {
            name: "Custom Email Domains",
            availability: [
              { jurisdiction: "aws_nac", lifecycle: "GA", compliance: "Available" },
              { jurisdiction: "aws_frm", lifecycle: "GA", compliance: "Available" },
              { jurisdiction: "aws_frh", lifecycle: "GA", compliance: "Available" },
              { jurisdiction: "aws_il4", lifecycle: "GA", compliance: "Available" },
              { jurisdiction: "aws_emea", lifecycle: "GA", compliance: "Available" },
              { jurisdiction: "aws_apj", lifecycle: "GA", compliance: "Available" },
              { jurisdiction: "gcp_nac", lifecycle: "EA", compliance: "Not Available" },
            ],
          },
          {
            name: "Standard Disaster Recovery",
            availability: [
              { jurisdiction: "aws_nac", lifecycle: "GA", compliance: "Available" },
              { jurisdiction: "aws_frm", lifecycle: "GA", compliance: "Available" },
              { jurisdiction: "aws_frh", lifecycle: "GA", compliance: "Available" },
              { jurisdiction: "aws_il4", lifecycle: "GA", compliance: "Available" },
              { jurisdiction: "aws_emea", lifecycle: "GA", compliance: "Available" },
              { jurisdiction: "aws_apj", lifecycle: "GA", compliance: "Available" },
              { jurisdiction: "gcp_nac", lifecycle: "EA", compliance: "Not Available" },
            ],
          },
          {
            name: "Protected Actions",
            availability: [
              { jurisdiction: "aws_nac", lifecycle: "GA", compliance: "Available" },
              { jurisdiction: "aws_frm", lifecycle: "GA", compliance: "Available" },
              { jurisdiction: "aws_frh", lifecycle: "GA", compliance: "Available" },
              { jurisdiction: "aws_il4", lifecycle: "GA", compliance: "Available" },
              { jurisdiction: "aws_emea", lifecycle: "GA", compliance: "Available" },
              { jurisdiction: "aws_apj", lifecycle: "GA", compliance: "Available" },
              { jurisdiction: "gcp_nac", lifecycle: "EA", compliance: "Not Available" },
            ],
          },
          {
            name: "Log Streaming for EventBridge",
            availability: [
              { jurisdiction: "aws_nac", lifecycle: "GA", compliance: "Available" },
              { jurisdiction: "aws_frm", lifecycle: "GA", compliance: "Not Available" },
              { jurisdiction: "aws_frh", lifecycle: "GA", compliance: "Not Available" },
              { jurisdiction: "aws_il4", lifecycle: "GA", compliance: "Not Available" },
              { jurisdiction: "aws_emea", lifecycle: "GA", compliance: "Available" },
              { jurisdiction: "aws_apj", lifecycle: "GA", compliance: "Available" },
              { jurisdiction: "gcp_nac", lifecycle: "EA", compliance: "Not Available" },
            ],
          },
          {
            name: "Log Streaming for Splunk Cloud",
            availability: [
              { jurisdiction: "aws_nac", lifecycle: "GA", compliance: "Available" },
              { jurisdiction: "aws_frm", lifecycle: "GA", compliance: "Not Available" },
              { jurisdiction: "aws_frh", lifecycle: "GA", compliance: "Not Available" },
              { jurisdiction: "aws_il4", lifecycle: "GA", compliance: "Not Available" },
              { jurisdiction: "aws_emea", lifecycle: "GA", compliance: "Available" },
              { jurisdiction: "aws_apj", lifecycle: "GA", compliance: "Available" },
              { jurisdiction: "gcp_nac", lifecycle: "EA", compliance: "Not Available" },
            ],
          },
          {
            name: "Interactive Reports (Reports 2.0)",
            availability: [
              { jurisdiction: "aws_nac", lifecycle: "GA", compliance: "Available" },
              { jurisdiction: "aws_frm", lifecycle: "GA", compliance: "Not Available" },
              { jurisdiction: "aws_frh", lifecycle: "GA", compliance: "Not Available" },
              { jurisdiction: "aws_il4", lifecycle: "GA", compliance: "Not Available" },
              { jurisdiction: "aws_emea", lifecycle: "GA", compliance: "Available" },
              { jurisdiction: "aws_apj", lifecycle: "GA", compliance: "Available" },
              { jurisdiction: "gcp_nac", lifecycle: "EA", compliance: "Not Available" },
            ],
          },
          {
            name: "Cloud Provisioning Connector",
            availability: [
              { jurisdiction: "aws_nac", lifecycle: "GA", compliance: "Available" },
              { jurisdiction: "aws_frm", lifecycle: "GA", compliance: "Available" },
              { jurisdiction: "aws_frh", lifecycle: "GA", compliance: "Available" },
              { jurisdiction: "aws_il4", lifecycle: "GA", compliance: "Available" },
              { jurisdiction: "aws_emea", lifecycle: "GA", compliance: "Available" },
              { jurisdiction: "aws_apj", lifecycle: "GA", compliance: "Available" },
              { jurisdiction: "gcp_nac", lifecycle: "EA", compliance: "Not Available" },
            ],
          },
        ],
      },
    ],
  },
  {
    name: "Priveleged Access",
    products: [
      {
        name: "Okta Privileged Acces",
        availability: [
          { jurisdiction: "aws_nac", lifecycle: "GA", compliance: "Available" },
          { jurisdiction: "aws_frm", lifecycle: "GA", compliance: "Not Available" },
          { jurisdiction: "aws_frh", lifecycle: "GA", compliance: "Not Available" },
          { jurisdiction: "aws_il4", lifecycle: "GA", compliance: "Not Available" },
          { jurisdiction: "aws_emea", lifecycle: "GA", compliance: "Available" },
          { jurisdiction: "aws_apj", lifecycle: "GA", compliance: "Available" },
          { jurisdiction: "gcp_nac", lifecycle: "EA", compliance: "Not Available" },
        ],
      },
      {
        name: "Advanced Server Access",
        availability: [
          { jurisdiction: "aws_nac", lifecycle: "GA", compliance: "Available" },
          { jurisdiction: "aws_frm", lifecycle: "GA", compliance: "Not Available" },
          { jurisdiction: "aws_frh", lifecycle: "GA", compliance: "Not Available" },
          { jurisdiction: "aws_il4", lifecycle: "GA", compliance: "Not Available" },
          { jurisdiction: "aws_emea", lifecycle: "GA", compliance: "Available" },
          { jurisdiction: "aws_apj", lifecycle: "GA", compliance: "Available" },
          { jurisdiction: "gcp_nac", lifecycle: "EA", compliance: "Not Available" },
        ],
      },
    ],
  },
  {
    name: "Okta Personal",
    products: [
      {
        name: "Okta Personal",
        availability: [
          { jurisdiction: "aws_nac", lifecycle: "GA", compliance: "Available" },
          { jurisdiction: "aws_frm", lifecycle: "GA", compliance: "Not Available" },
          { jurisdiction: "aws_frh", lifecycle: "GA", compliance: "Not Available" },
          { jurisdiction: "aws_il4", lifecycle: "GA", compliance: "Not Available" },
          { jurisdiction: "aws_emea", lifecycle: "GA", compliance: "Available" },
          { jurisdiction: "aws_apj", lifecycle: "GA", compliance: "Available" },
          { jurisdiction: "gcp_nac", lifecycle: "EA", compliance: "Not Available" },
        ],
      },
    ],
  },
  {
    name: "Identity Security Posture Management",
    products: [
      {
        name: "Identity Security Posture Management",
        availability: [
          { jurisdiction: "aws_nac", lifecycle: "GA", compliance: "Available" },
          { jurisdiction: "aws_frm", lifecycle: "GA", compliance: "Not Available" },
          { jurisdiction: "aws_frh", lifecycle: "GA", compliance: "Not Available" },
          { jurisdiction: "aws_il4", lifecycle: "GA", compliance: "Not Available" },
          { jurisdiction: "aws_emea", lifecycle: "GA", compliance: "Not Available" },
          { jurisdiction: "aws_apj", lifecycle: "GA", compliance: "Not Available" },
          { jurisdiction: "gcp_nac", lifecycle: "EA", compliance: "Not Available" },
        ],
      },
    ],
  },
];

const category_query = `mutation AddCategory($name: String!) {
  createCategory(name: $name) {
    id
    name
  }
}`;

// ProductInput {
//  name: String,
//  category: String
// }
const product_query = `mutation AddProduct($input: ProductInput!) {
  createProduct(input: $input) {
    id
    name
  }
}`;

// FeatureInput {
//  name: String,
//  product: String
// }
const feature_query = `mutation AddFeature($input: FeatureInput!) {
  createFeature(input: $input) {
    id
    name
  }
}`;

//AvailabilityInput {
//  item_id: String,
//  jurisdiction: String,
//  lifecycle: String,
//  compliance: String
//}

const availability_query = `mutation AddAvailability($input: AvailabilityInput!) {
  createAvailability(input: $input) {
    id
  }
}`;

const comment_query = `mutation AddComment($input: CommentInput!) {
  createComment(input: $input) {
    id
    itemId
    text,
    created
    createdBy
  }
}
  `;

export async function add() {
  for (let category of CATEGORIES) {
    await query(category_query, "AddCategory", { name: category.name });
    if (category.hasOwnProperty("products")) {
      for (let product of category.products) {
        let product_data = await query(product_query, "AddProduct", {
          input: {
            name: product.name,
            category: category.name,
          },
        });
        if (product.hasOwnProperty("comments")) {
          for (let comment of product.comments) {
            let itemId = product_data.createProduct.id;
            let input = { itemId, ...comment };
            await query(comment_query, "AddComment", { input });
          }
        }
        if (product.hasOwnProperty("availability")) {
          let itemId = product_data.createProduct.id;
          for (let availability of product.availability) {
            const { comments, ...rest } = availability;
            const input = { itemId, ...rest };
            let availability_data = await query(availability_query, "AddAvailability", { input });
            if (comments != undefined) {
              for (let comment of comments) {
                let itemId = availability_data.createAvailability.id;
                let input = { itemId, ...comment };
                await query(comment_query, "AddComment", { input });
              }
            }
          }
        }
        if (product.hasOwnProperty("features")) {
          for (let feature of product.features) {
            let feature_data = await query(feature_query, "AddFeature", {
              input: {
                name: feature.name,
                product: product.name,
              },
            });
            if (feature.hasOwnProperty("comments")) {
              for (let comment of feature.comments) {
                let itemId = feature_data.createFeature.id;
                let input = { itemId, ...comment };
                await query(comment_query, "AddComment", { input });
              }
            }
            if (feature.hasOwnProperty("availability")) {
              let itemId = feature_data.createFeature.id;
              for (let availability of feature.availability) {
                const { comments, ...rest } = availability;
                const input = { itemId, ...rest };
                let availability_data = await query(availability_query, "AddAvailability", { input });
                if (comments != undefined) {
                  for (let comment of comments) {
                    let itemId = availability_data.createAvailability.id;
                    let input = { itemId, ...comment };
                    await query(comment_query, "AddComment", { input });
                  }
                }
              }
            }
          }
        }
      }
    }
  }
}
