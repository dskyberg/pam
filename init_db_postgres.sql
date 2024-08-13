
CREATE TABLE "category"
(
    id character(36) NOT NULL,
    name character varying(256)  NOT NULL,
    CONSTRAINT "category_pkey" PRIMARY KEY (id),
    CONSTRAINT "category_name_UNIQUE" UNIQUE (name)
);

CREATE TABLE "product"
(
    id character(36) NOT NULL,
    name character varying(200) NOT NULL,
    category_id character(36) NOT NULL,
    CONSTRAINT "product_pkey" PRIMARY KEY (id),
    CONSTRAINT "product_name_UNIQUE" UNIQUE (name),
    CONSTRAINT "product_fk0" FOREIGN KEY (category_id)
        REFERENCES "category" (id) MATCH SIMPLE
);

CREATE TABLE "feature" (
    id character(36) NOT NULL,
    name character varying(128) NOT NULL,
    product_id character(36) NOT NULL,
    CONSTRAINT "feature_pkey" PRIMARY KEY (id),
    CONSTRAINT "feature_name_UNIQUE" UNIQUE (name),
    CONSTRAINT "feature_fk0" FOREIGN KEY (product_id)
        REFERENCES "product" (id) MATCH SIMPLE
);

CREATE TABLE "jurisdiction" (
    id character(36) NOT NULL,
    name character varying(128) NOT NULL,
    title character varying(200) NOT NULL,
    CONSTRAINT "jurisdiction_pkey" PRIMARY KEY (id),
    CONSTRAINT "jurisdiction_name_UNIQUE" UNIQUE (name),
    CONSTRAINT "jurisdiction_title_UNIQUE" UNIQUE (title)
);

CREATE TABLE "cell" (
    id character(36) NOT NULL,
    name character varying(128) NOT NULL,
    csp character varying(20) NOT NULL,
    country character varying(10) NOT NULL,
    region character varying(20) NOT NULL,
    csp_region character varying(20) NOT NULL,
    jurisdiction_id character(36) NOT NULL,
    CONSTRAINT "cell_pkey" PRIMARY KEY (id),
    CONSTRAINT "cell_name_UNIQUE" UNIQUE (name),
    CONSTRAINT "cell_fk0" FOREIGN KEY (jurisdiction_id)
        REFERENCES "jurisdiction" (id) MATCH SIMPLE
);

CREATE TABLE "compliance" (
    id character(36) NOT NULL,
    name character varying(128) NOT NULL,
    CONSTRAINT "compliance_pkey" PRIMARY KEY (id),
    CONSTRAINT "compliance_name_UNIQUE" UNIQUE (name)
);

CREATE TABLE "lifecycle" (
    id character(36) NOT NULL,
    name character varying(10) NOT NULL,
    CONSTRAINT "lifecycle_pkey" PRIMARY KEY (id),
    CONSTRAINT "lifecycle_name_UNIQUE" UNIQUE (name)
);

CREATE TABLE "availability" (
    id character(36) NOT NULL,
     item_id character(36) NOT NULL,
     jurisdiction_id character(36) NOT NULL,
     lifecycle_id character(36) NOT NULL,
     compliance_id character(36) NOT NULL,
     last_updated timestamp NOT NULL,
     CONSTRAINT "availability_pkey" PRIMARY KEY (id)
 );

CREATE TABLE "history" (
    id character(36) NOT NULL,
    date timestamp NOT NULL,
);

CREATE INDEX "availability_item_idx" on "availability" (item_id);
CREATE INDEX "availability_jurisdiction_idx" on "availability" (jurisdiction_id);
CREATE INDEX "availability_lifecycle_idx" on "availability" (lifecycle_id);
CREATE INDEX "availability_compliance_idx" on "availability" (compliance_id);

CREATE TABLE "comment" (
   id character(36) NOT NULL,
   item_id character(36) NOT NULL,
   text character varying(256) NOT NULL,
   created timestamp NOT NULL,
   created_by character varying(128),
   CONSTRAINT "comment_pkey" PRIMARY KEY (id)
);

CREATE INDEX "comment_item_idx" on "comment" (item_id);
CREATE INDEX "comment_created_idx" on "comment" (created);


DO $$
DECLARE
  ig_id character(36) := gen_random_uuid();
  am_id character(36) := gen_random_uuid();
  p_ud character(36) := gen_random_uuid();
  p_sso character(36) := gen_random_uuid();
  p_amfa character(36) := gen_random_uuid();
  ci_id character(36) := gen_random_uuid();
  platform_id character(36) := gen_random_uuid();
  g2k_id character(36) := gen_random_uuid();
  standard_id character(36) := gen_random_uuid();
  pa_id character(36) := gen_random_uuid();

  j_nac character(36) := gen_random_uuid();
  j_frm character(36) := gen_random_uuid();
  j_frh character(36) := gen_random_uuid();
  j_il4 character(36) := gen_random_uuid();
  j_emea character(36) := gen_random_uuid();
  j_apj character(36) := gen_random_uuid();
  j_gcp_nac character(36) := gen_random_uuid();

  ok1 character(36) := gen_random_uuid();
  ok2 character(36) := gen_random_uuid();
  ok3 character(36) := gen_random_uuid();
  ok4 character(36) := gen_random_uuid();
  ok6 character(36) := gen_random_uuid();
  ok11 character(36) := gen_random_uuid();
  ok7 character(36) := gen_random_uuid();
  ok12 character(36) := gen_random_uuid();
  ok14 character(36) := gen_random_uuid();
  ok9 character(36) := gen_random_uuid();
  eu1 character(36) := gen_random_uuid();
  ok5 character(36) := gen_random_uuid();
  ok10 character(36) := gen_random_uuid();
  og1 character(36) := gen_random_uuid();
  om1 character(36) := gen_random_uuid();
  ok8 character(36) := gen_random_uuid();
  ok16 character(36) := gen_random_uuid();
  ok17 character(36) := gen_random_uuid();

  beta character(36) := gen_random_uuid();
  lea character(36) := gen_random_uuid();
  ea character(36) := gen_random_uuid();
  ga character(36) := gen_random_uuid();
  eol character(36) := gen_random_uuid();

  not_available character(36) := gen_random_uuid();
  available character(36) := gen_random_uuid();
  audit_ready character(36) := gen_random_uuid();
  authorized character(36) := gen_random_uuid();

  f_custom_domains character(36) := gen_random_uuid();

BEGIN
  INSERT INTO "category"  VALUES (ig_id, 'Identity Governance');
  INSERT INTO "category"  VALUES (am_id, 'Access Management');
  INSERT INTO "category"  VALUES (ci_id, 'Customer Identity');
  INSERT INTO "category"  VALUES (platform_id, 'Platform');
  INSERT INTO "category"  VALUES (pa_id, 'Priveleged Access');
  INSERT INTO "category"  VALUES (gen_random_uuid(), 'Okta Personal');
  INSERT INTO "category"  VALUES (gen_random_uuid(), 'Identity Security Posture Management');


INSERT INTO "product"  VALUES (p_ud, 'Universal Directory', ig_id);
INSERT INTO "feature" VALUES (gen_random_uuid(), 'Kerberos: Integrated Windows Auth', p_ud);
INSERT INTO "feature" VALUES (gen_random_uuid(), 'Kerberos: Silent Activation', p_ud);
INSERT INTO "feature" VALUES (gen_random_uuid(), 'Kerberos: Agentless Desktop SSO', p_ud);
INSERT INTO "feature" VALUES (gen_random_uuid(), 'AD Del Auth', p_ud);
INSERT INTO "feature" VALUES (gen_random_uuid(), 'LDAP Interface', p_ud);

INSERT INTO "product"  VALUES (gen_random_uuid(), 'Lifecycle Management',ig_id);
INSERT INTO "product"  VALUES (gen_random_uuid(), 'Secure Partner Access',ig_id);
INSERT INTO "product"  VALUES (gen_random_uuid(), 'Okta Identity Governance',ig_id);


INSERT INTO "product"  VALUES (p_sso, 'Single Sign-On',am_id);
INSERT INTO "feature" VALUES (gen_random_uuid(), 'FastPass', p_sso);
INSERT INTO "feature" VALUES (gen_random_uuid(), 'O365', p_sso);
INSERT INTO "feature" VALUES (gen_random_uuid(), 'O365 GCC High (Profile Sync and icense and Role Management)', p_sso);
INSERT INTO "feature" VALUES (gen_random_uuid(), 'O365 GCC High (User and Universal Sync)', p_sso);
INSERT INTO "feature" VALUES (gen_random_uuid(), 'O365 DOD', p_sso);

INSERT INTO "product"  VALUES (p_amfa, 'Adaptive MFA',am_id);
INSERT INTO "feature" VALUES (gen_random_uuid(), 'Device Trust', p_amfa);
INSERT INTO "feature" VALUES (gen_random_uuid(), 'Desktop MFA', p_amfa);
INSERT INTO "feature" VALUES (gen_random_uuid(), 'Policy Recommender with Okta AI', p_amfa);
INSERT INTO "feature" VALUES (gen_random_uuid(), 'Pre-enrolled Yubikeys', p_amfa);
INSERT INTO "feature" VALUES (gen_random_uuid(), 'SMS / Voice Authenticator', p_amfa);
INSERT INTO "feature" VALUES (gen_random_uuid(), 'Email Authenticator', p_amfa);
INSERT INTO "feature" VALUES (gen_random_uuid(), 'Okta Verify', p_amfa);
INSERT INTO "feature" VALUES (gen_random_uuid(), 'Okta Mobile (on Classic)', p_amfa);

INSERT INTO "product"  VALUES (gen_random_uuid(), 'API Access Management', am_id);
INSERT INTO "product"  VALUES (gen_random_uuid(), 'Okta Access Gateway', am_id);
INSERT INTO "product"  VALUES (gen_random_uuid(), 'Okta Device Accews', am_id);
INSERT INTO "product"  VALUES (gen_random_uuid(), 'Identity Threat Protection', am_id);

INSERT INTO "product"  VALUES (gen_random_uuid(), 'Customer Identity Solution', ci_id);

INSERT INTO "product"  VALUES (gen_random_uuid(), 'Okta Workflows',platform_id);

INSERT INTO "product"  VALUES (g2k_id, 'Okta for Global 2000',platform_id);
INSERT INTO "feature" VALUES (gen_random_uuid(), 'Org2Org', g2k_id);
INSERT INTO "feature" VALUES (gen_random_uuid(), 'Ariel', g2k_id);

INSERT INTO "product"  VALUES (gen_random_uuid(), 'Multi Org Deployment',platform_id);
INSERT INTO "product"  VALUES (gen_random_uuid(), 'Enhanced Disaster Recovery',platform_id);
INSERT INTO "product"  VALUES (gen_random_uuid(), 'Dynamic Scale',platform_id);
INSERT INTO "product"  VALUES (gen_random_uuid(), 'Log Investigator with Okta AI',platform_id);

INSERT INTO "product"  VALUES (standard_id, 'Standard Okta Offerings (No SKU)',platform_id);
INSERT INTO "feature"  VALUES (f_custom_domains, 'Custom Domains',standard_id);
INSERT INTO "feature"  VALUES (gen_random_uuid(), 'Customer Email Domains',standard_id);
INSERT INTO "feature"  VALUES (gen_random_uuid(), 'Standard Disaster Recovery',standard_id);
INSERT INTO "feature"  VALUES (gen_random_uuid(), 'Protected Actions',standard_id);
INSERT INTO "feature"  VALUES (gen_random_uuid(), 'Log Streaming for EventBridge',standard_id);
INSERT INTO "feature"  VALUES (gen_random_uuid(), 'Log Streaming for Splunk Cloud',standard_id);
INSERT INTO "feature"  VALUES (gen_random_uuid(), 'Interactive Reports (Reports 2.0)',standard_id);
INSERT INTO "feature"  VALUES (gen_random_uuid(), 'Cloud Provisioning Connector',standard_id);

INSERT INTO "product"  VALUES (gen_random_uuid(), 'Okta Privileged Acces',pa_id);
INSERT INTO "product"  VALUES (gen_random_uuid(), 'Advanced Server Access',pa_id);

INSERT INTO "jurisdiction" VALUES (j_nac, 'aws_nac', 'North America Commercial');
INSERT INTO "jurisdiction" VALUES (j_frm, 'aws_frm', 'Okta for Government Moderate & HIPAA');
INSERT INTO "jurisdiction" VALUES (j_frh, 'aws_frh', 'Okta for Government High');
INSERT INTO "jurisdiction" VALUES (j_il4, 'aws_il4', 'Okta for U.S. Military');
INSERT INTO "jurisdiction" VALUES (j_emea, 'aws_emea', 'EMEA');
INSERT INTO "jurisdiction" VALUES (j_apj, 'aws_apj', 'APJ');
INSERT INTO "jurisdiction" VALUES (j_gcp_nac, 'gcp_nac', 'GCP North America Commercial');

INSERT INTO "cell" VALUES (ok1, 'OK1','AWS', 'AMER', 'USA', 'us-east-1', j_nac);
INSERT INTO "cell" VALUES (ok2, 'OK2','AWS', 'AMER', 'USA', 'us-east-1', j_nac);
INSERT INTO "cell" VALUES (ok3, 'OK3','AWS', 'AMER', 'USA', 'us-east-1', j_nac);
INSERT INTO "cell" VALUES (ok4, 'OK4','AWS', 'AMER', 'USA', 'us-east-1', j_nac);
INSERT INTO "cell" VALUES (ok6, 'OK6','AWS', 'AMER', 'USA', 'us-east-2', j_nac);
INSERT INTO "cell" VALUES (ok11, 'OK11','AWS', 'AMER', 'USA', 'us-east-2', j_nac);
INSERT INTO "cell" VALUES (ok7, 'OK7','AWS', 'AMER', 'USA', 'us-west-2', j_nac);
INSERT INTO "cell" VALUES (ok12, 'OK12','AWS', 'AMER', 'USA', 'us-west-2', j_nac);
INSERT INTO "cell" VALUES (ok14, 'OK14','AWS', 'AMER', 'USA', 'us-west-2', j_nac);
INSERT INTO "cell" VALUES (ok9, 'OK9','AWS', 'EMEA', 'IRE', 'eu-west-1', j_emea);
INSERT INTO "cell" VALUES (eu1, 'EU1','AWS', 'EMEA', 'GER', 'eu-central-1', j_emea);
INSERT INTO "cell" VALUES (ok5, 'OK5','AWS', 'AMER', 'USA', 'us-west-2', j_frm);
INSERT INTO "cell" VALUES (ok10, 'OK10','AWS', 'AMER', 'USA', 'us-east-2', j_frm);
INSERT INTO "cell" VALUES (og1, 'OG1','AWS', 'AMER', 'USA', 'us-gov-west-1', j_frh);
INSERT INTO "cell" VALUES (om1, 'OM1','AWS', 'AMER', 'USA', 'us-gov-east-1', j_il4);
INSERT INTO "cell" VALUES (ok8, 'OK8','AWS', 'APJ', 'AUS', 'ap-southeast-2', j_apj);
INSERT INTO "cell" VALUES (ok16, 'OK16','AWS', 'APJ', 'JAP', 'ap-nrtheast-1', j_apj);
INSERT INTO "cell" VALUES (ok17, 'OK17','GCP', 'AMER', 'USA', 'us-east4', j_gcp_nac);

INSERT INTO "lifecycle" VALUES (beta, 'BETA');
INSERT INTO "lifecycle" VALUES (lea, 'LEA');
INSERT INTO "lifecycle" VALUES (ea, 'EA');
INSERT INTO "lifecycle" VALUES (ga, 'GA');
INSERT INTO "lifecycle" VALUES (eol, 'EOL');

INSERT INTO "compliance" VALUES (not_available, 'Not Available');
INSERT INTO "compliance" VALUES (available, 'Available');
INSERT INTO "compliance" VALUES (audit_ready, 'Audit Ready');
INSERT INTO "compliance" VALUES (authorized, 'Authorized');

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

END
$$;

DO $$
BEGIN
INSERT INTO "comment" (id, item_id, text, created) VALUES (gen_random_uuid(), (SELECT id FROM availability LIMIT 1), 'The first comment', CURRENT_TIMESTAMP);
INSERT INTO "comment" (id, item_id, text, created) VALUES (gen_random_uuid(), (SELECT id FROM availability LIMIT 1), 'The second comment', CURRENT_TIMESTAMP);

INSERT INTO "comment" (id, item_id, text, created) VALUES (gen_random_uuid(), (SELECT id FROM availability LIMIT 1 OFFSET 2), 'The first comment', CURRENT_TIMESTAMP);

END
$$;
