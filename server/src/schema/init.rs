use anyhow::Result;

use crate::database::Pool;

use super::{Category, Compliance, Feature, Lifecycle, Product};

/// Initialize the database
pub async fn init(pool: &Pool) -> Result<u32> {
    let mut records = 0;

    // Don't run if the DB has already been initialized
    if Category::count(pool).await? > 0 {
        return Ok(0);
    }

    let _lifecycles = Lifecycle::fetch_all(None, None, pool).await?;
    let _compliances = Compliance::fetch_all(None, None, pool).await?;

    let category = Category::create("Identity Governance", pool).await?;
    records += 1;

    let product = Product::create("Universal Directory", &category.id, pool).await?;
    records += 1;
    Feature::create("Kerberos: Integrated Windows Auth", &product.id, pool).await?;
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
        "'O365 GCC High (Profile Sync and icense and Role Management)",
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
    Feature::create("Custom Domains", &product.id, pool).await?;
    records += 1;
    Feature::create("Customer Email Domains", &product.id, pool).await?;
    records += 1;
    Feature::create("Standard Disaster Recovery", &product.id, pool).await?;
    records += 1;
    Feature::create("Protected Actions", &product.id, pool).await?;
    records += 1;
    Feature::create("Log Streaming for EventBridge", &product.id, pool).await?;
    records += 1;
    Feature::create("Log Streaming for Splunk Cloud", &product.id, pool).await?;
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

    Category::create("Okta Personal", pool).await?;
    records += 1;
    Category::create("Identity Security Posture Management", pool).await?;
    records += 1;

    Ok(records)
    /*

        INSERT INTO "availability" (id, item_id, jurisdiction_id, lifecycle_id, compliance_id, last_updated) VALUES (gen_random_uuid(), p_ud, j_nac, ga, available, CURRENT_TIMESTAMP);
        INSERT INTO "availability" (id, item_id, jurisdiction_id, lifecycle_id, compliance_id, last_updated) VALUES (gen_random_uuid(), p_ud, j_frm, ga, available, CURRENT_TIMESTAMP);
        INSERT INTO "availability" (id, item_id, jurisdiction_id, lifecycle_id, compliance_id, last_updated) VALUES (gen_random_uuid(), p_ud, j_frh, ga, available, CURRENT_TIMESTAMP);
        INSERT INTO "availability" (id, item_id, jurisdiction_id, lifecycle_id, compliance_id, last_updated) VALUES (gen_random_uuid(), p_ud, j_il4, ga, available, CURRENT_TIMESTAMP);
        INSERT INTO "availability" (id, item_id, jurisdiction_id, lifecycle_id, compliance_id, last_updated) VALUES (gen_random_uuid(), p_ud, j_emea, ga, available, CURRENT_TIMESTAMP);
        INSERT INTO "availability" (id, item_id, jurisdiction_id, lifecycle_id, compliance_id, last_updated) VALUES (gen_random_uuid(), p_ud, j_apj, ga, available, CURRENT_TIMESTAMP);
        INSERT INTO "availability" (id, item_id, jurisdiction_id, lifecycle_id, compliance_id, last_updated) VALUES (gen_random_uuid(), p_ud, j_gcp_nac, ga, not_available, CURRENT_TIMESTAMP);
        INSERT INTO "availability" (id, item_id, jurisdiction_id, lifecycle_id, compliance_id, last_updated) VALUES (gen_random_uuid(), f_custom_domains, j_nac, ga, available, CURRENT_TIMESTAMP);
        INSERT INTO "availability" (id, item_id, jurisdiction_id, lifecycle_id, compliance_id, last_updated) VALUES (gen_random_uuid(), f_custom_domains, j_frm, ga, available, CURRENT_TIMESTAMP);
        INSERT INTO "availability" (id, item_id, jurisdiction_id, lifecycle_id, compliance_id, last_updated) VALUES (gen_random_uuid(), f_custom_domains, j_frh, ga, available, CURRENT_TIMESTAMP);
        INSERT INTO "availability" (id, item_id, jurisdiction_id, lifecycle_id, compliance_id, last_updated) VALUES (gen_random_uuid(), f_custom_domains, j_il4, ga, not_available, CURRENT_TIMESTAMP);
        INSERT INTO "availability" (id, item_id, jurisdiction_id, lifecycle_id, compliance_id, last_updated) VALUES (gen_random_uuid(), f_custom_domains, j_emea, ga, available, CURRENT_TIMESTAMP);
        INSERT INTO "availability" (id, item_id, jurisdiction_id, lifecycle_id, compliance_id, last_updated) VALUES (gen_random_uuid(), f_custom_domains, j_apj, ga, available, CURRENT_TIMESTAMP);
    */
}
