use anyhow::Result;
use std::collections::BTreeMap;
use tracing_actix_web::TracingLogger;

use crate::{
    database::Pool,
    schema::{
        Availability, Category, Cell, Comment, Compliance, Feature, Jurisdiction, Lifecycle,
        Product,
    },
};

struct MyMap(BTreeMap<String, String>);
impl MyMap {
    pub fn new() -> Self {
        Self(BTreeMap::new())
    }
    pub fn insert(&mut self, key: &str, value: &str) {
        self.0.insert(key.to_owned(), value.to_owned());
    }
    pub fn get(&self, key: &str) -> Result<&String> {
        self.0.get(key).ok_or(anyhow!("Key not found: {}", key))
    }
}

static LIFECYCLES: [&str; 5] = ["BETA", "LEA", "EA", "GA", "EOL"];
static COMPLIANCES: [&str; 4] = ["Not Available", "Available", "Audit Ready", "Authorized"];
static JURISDICTIONS: [(&str, &str); 7] = [
    ("aws_nac", "North America Commercial"),
    ("aws_frm", "Okta for Government Moderate & HIPAA"),
    ("aws_frh", "Okta for Government High"),
    ("aws_il4", "Okta for U.S. Military"),
    ("aws_emea", "EMEA"),
    ("aws_apj", "APJ"),
    ("gcp_nac", "GCP North America Commercial"),
];
static CELLS: [(&str, &str, &str, &str, &str, &str); 18] = [
    ("OK1", "AWS", "AMER", "USA", "us-east-1", "aws_nac"),
    ("OK2", "AWS", "AMER", "USA", "us-east-1", "aws_nac"),
    ("OK3", "AWS", "AMER", "USA", "us-east-1", "aws_nac"),
    ("OK4", "AWS", "AMER", "USA", "us-east-1", "aws_nac"),
    ("OK6", "AWS", "AMER", "USA", "us-east-2", "aws_nac"),
    ("OK11", "AWS", "AMER", "USA", "us-east-2", "aws_nac"),
    ("OK7", "AWS", "AMER", "USA", "us-west-2", "aws_nac"),
    ("OK12", "AWS", "AMER", "USA", "us-west-2", "aws_nac"),
    ("OK14", "AWS", "AMER", "USA", "us-west-2", "aws_nac"),
    ("OK9", "AWS", "EMEA", "IRE", "eu-west-1", "aws_emea"),
    ("EU1", "AWS", "EMEA", "GER", "eu-central-1", "aws_emea"),
    ("OK5", "AWS", "AMER", "USA", "us-west-2", "aws_frm"),
    ("OK10", "AWS", "AMER", "USA", "us-east-2", "aws_frm"),
    ("OG1", "AWS", "AMER", "USA", "us-gov-west-1", "aws_frh"),
    ("OM1", "AWS", "AMER", "USA", "us-gov-east-1", "aws_il4"),
    ("OK8", "AWS", "APJ", "AUS", "ap-southeast-2", "aws_apj"),
    ("OK16", "AWS", "APJ", "JAP", "ap-northeast-1", "aws_apj"),
    ("OK17", "GCP", "AMER", "USA", "us-east-1", "gcp_nac"),
];

async fn init_lifecycle(pool: &Pool) -> Result<MyMap> {
    let mut map = MyMap::new();
    for name in LIFECYCLES {
        let result = Lifecycle::create(name, pool).await?;
        map.insert(name, &result.id);
    }

    Ok(map)
}

async fn init_compliance(pool: &Pool) -> Result<MyMap> {
    let mut map = MyMap::new();
    for name in COMPLIANCES {
        let result = Compliance::create(name, pool).await?;
        map.insert(name, &result.id);
    }

    Ok(map)
}

async fn init_jurisdictions(pool: &Pool) -> Result<MyMap> {
    let mut map = MyMap::new();
    for (name, title) in JURISDICTIONS {
        let result = Jurisdiction::create(name, title, pool).await?;
        map.insert(name, &result.id);
    }

    Ok(map)
}

async fn init_cells(j: &MyMap, pool: &Pool) -> Result<usize> {
    for (name, csp, country, region, csp_region, jurisdiction) in CELLS {
        Cell::create(
            name,
            csp,
            country,
            region,
            csp_region,
            j.get(jurisdiction)?,
            pool,
        )
        .await?;
    }
    Ok(CELLS.len())
}

/// Initialize the database
pub async fn initialize_db(pool: &Pool) -> Result<u32> {
    let mut records = 0u32;

    // Don't run if the DB has already been initialized
    if Category::count(pool).await? > 0 {
        return Ok(0);
    }

    let l = init_lifecycle(pool).await?;
    records += l.0.len() as u32;

    let c = init_compliance(pool).await?;
    records += c.0.len() as u32;
    let j = init_jurisdictions(pool).await?;
    records += l.0.len() as u32;

    records += init_cells(&j, pool).await? as u32;

    let category = Category::create("Identity Governance", pool).await?;
    records += 1;

    let product = Product::create("Universal Directory", &category.id, pool).await?;
    records += 1;
    /*
    aws_nas GA Authorized
    aws_frm GA Authorized
    aws_frh GA Authorized
    aws_il4 GA Authorized
    aws_emea GA Available
    aws_apj GA Available
    gcp_nac GA Available
    */
    let availability = Availability::create(
        &product.id,
        j.get("aws_nac")?,
        l.get("GA")?,
        c.get("Available")?,
        pool,
    )
    .await?;
    records += 1;
    Comment::create(&availability.id, "The first comment", None, pool).await?;
    records += 1;
    Comment::create(&availability.id, "The second comment", None, pool).await?;
    records += 1;

    Availability::create(
        &product.id,
        j.get("aws_frm")?,
        l.get("GA")?,
        c.get("Available")?,
        pool,
    )
    .await?;
    records += 1;
    Availability::create(
        &product.id,
        j.get("aws_frh")?,
        l.get("GA")?,
        c.get("Available")?,
        pool,
    )
    .await?;
    records += 1;
    Availability::create(
        &product.id,
        j.get("aws_il4")?,
        l.get("GA")?,
        c.get("Available")?,
        pool,
    )
    .await?;
    records += 1;
    Availability::create(
        &product.id,
        j.get("aws_emea")?,
        l.get("GA")?,
        c.get("Available")?,
        pool,
    )
    .await?;
    records += 1;
    Availability::create(
        &product.id,
        j.get("aws_apj")?,
        l.get("EA")?,
        c.get("Available")?,
        pool,
    )
    .await?;
    records += 1;
    Availability::create(
        &product.id,
        j.get("gcp_nac")?,
        l.get("EA")?,
        c.get("Not Available")?,
        pool,
    )
    .await?;
    records += 1;

    /*
    aws_nas GA Authorized
    aws_frm GA Authorized
    aws_frh GA Not Available
    aws_il4 GA Not Available
    aws_emea GA Available
    aws_apj GA Available
    gcp_nac GA Available
    */

    let feature = Feature::create("Kerberos: Integrated Windows Auth", &product.id, pool).await?;
    records += 1;
    Comment::create(&feature.id, "The first comment", None, pool).await?;
    records += 1;

    Feature::create("Kerberos: Silent Activation", &product.id, pool).await?;
    records += 1;
    Feature::create("Kerberos: Agentless Desktop SSO", &product.id, pool).await?;
    records += 1;
    Feature::create("AD Del Auth", &product.id, pool).await?;
    records += 1;
    Feature::create("LDAP Interface", &product.id, pool).await?;
    records += 1;

    Product::create("Lifecycle Management", &category.id, pool).await?;
    records += 1;

    Product::create("Secure Partner Access", &category.id, pool).await?;
    records += 1;

    Product::create("Okta Identity Governance", &category.id, pool).await?;
    records += 1;

    let category = Category::create("Access Management", pool).await?;
    records += 1;

    let product = Product::create("Single Sign-On", &category.id, pool).await?;
    records += 1;
    Feature::create("FastPass", &product.id, pool).await?;
    records += 1;
    Feature::create("O365", &product.id, pool).await?;
    records += 1;
    Feature::create(
        "O365 GCC High (Profile Sync and icense and Role Management)",
        &product.id,
        pool,
    )
    .await?;
    records += 1;
    Feature::create("O365 GCC High (User and Universal Sync)", &product.id, pool).await?;
    records += 1;
    Feature::create("O365 DOD", &product.id, pool).await?;
    records += 1;

    let product = Product::create("Adaptive MFA", &category.id, pool).await?;
    records += 1;
    Feature::create("Device Trust", &product.id, pool).await?;
    records += 1;
    Feature::create("Desktop MFA", &product.id, pool).await?;
    records += 1;
    Feature::create("Policy Recommender with Okta AI", &product.id, pool).await?;
    records += 1;
    Feature::create("Pre-enrolled Yubikeys", &product.id, pool).await?;
    records += 1;
    Feature::create("SMS / Voice Authenticator", &product.id, pool).await?;
    records += 1;
    Feature::create("Email Authenticator", &product.id, pool).await?;
    records += 1;
    Feature::create("Okta Verify", &product.id, pool).await?;
    records += 1;
    Feature::create("Okta Mobile (on Classic)", &product.id, pool).await?;
    records += 1;

    Product::create("API Access Management", &category.id, pool).await?;
    records += 1;
    Product::create("Okta Access Gateway", &category.id, pool).await?;
    records += 1;
    Product::create("Okta Device Access", &category.id, pool).await?;
    records += 1;
    Product::create("Identity Threat Protection", &category.id, pool).await?;
    records += 1;

    let category = Category::create("Customer Identity", pool).await?;
    records += 1;
    Product::create("Customer Identity Solution", &category.id, pool).await?;
    records += 1;

    let category = Category::create("Platform", pool).await?;
    records += 1;

    Product::create("Okta Workflows", &category.id, pool).await?;
    records += 1;

    let product = Product::create("Okta for Global 2000", &category.id, pool).await?;
    records += 1;
    Feature::create("Org2Org", &product.id, pool).await?;
    records += 1;
    Feature::create("Ariel", &product.id, pool).await?;
    records += 1;

    Product::create("Multi Org Deployment", &category.id, pool).await?;
    records += 1;

    Product::create("Enhanced Disaster Recovery", &category.id, pool).await?;
    records += 1;

    Product::create("Dynamic Scale", &category.id, pool).await?;
    records += 1;

    Product::create("Log Investigator with Okta AI", &category.id, pool).await?;
    records += 1;

    let product = Product::create("Standard Okta Offerings (No SKU)", &category.id, pool).await?;
    records += 1;
    let feature = Feature::create("Custom Domains", &product.id, pool).await?;
    records += 1;
    Availability::create(
        &feature.id,
        j.get("aws_nac")?,
        l.get("GA")?,
        c.get("Available")?,
        pool,
    )
    .await?;
    records += 1;

    Availability::create(
        &feature.id,
        j.get("aws_frm")?,
        l.get("GA")?,
        c.get("Available")?,
        pool,
    )
    .await?;
    records += 1;
    Availability::create(
        &feature.id,
        j.get("aws_frh")?,
        l.get("GA")?,
        c.get("Available")?,
        pool,
    )
    .await?;
    records += 1;
    Availability::create(
        &feature.id,
        j.get("aws_il4")?,
        l.get("GA")?,
        c.get("Not Available")?,
        pool,
    )
    .await?;
    records += 1;
    Availability::create(
        &feature.id,
        j.get("aws_emea")?,
        l.get("GA")?,
        c.get("Available")?,
        pool,
    )
    .await?;
    records += 1;
    Availability::create(
        &feature.id,
        j.get("aws_apj")?,
        l.get("GA")?,
        c.get("Available")?,
        pool,
    )
    .await?;
    records += 1;

    Feature::create("Custom Email Domains", &product.id, pool).await?;
    records += 1;
    Feature::create("Standard Disaster Recovery", &product.id, pool).await?;
    records += 1;
    Feature::create("Protected Actions", &product.id, pool).await?;
    records += 1;
    Feature::create("Log Streaming for EventBridge", &product.id, pool).await?;
    records += 1;
    Feature::create("Log Streaming for Splunk Cloud", &product.id, pool).await?;
    records += 1;
    Feature::create("Interactive Reports (Reports 2.0)", &product.id, pool).await?;
    records += 1;
    Feature::create("Cloud Provisioning Connector", &product.id, pool).await?;
    records += 1;

    let category = Category::create("Priveleged Access", pool).await?;
    records += 1;
    Product::create("Okta Privileged Acces", &category.id, pool).await?;
    records += 1;
    Product::create("Advanced Server Access", &category.id, pool).await?;
    records += 1;

    let category = Category::create("Okta Personal", pool).await?;
    records += 1;
    Product::create("Okta Personal", &category.id, pool).await?;
    records += 1;

    let category = Category::create("Identity Security Posture Management", pool).await?;
    records += 1;
    Product::create("Identity Security Posture Management", &category.id, pool).await?;
    records += 1;

    Ok(records)
}
