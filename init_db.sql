DROP SCHEMA IF EXISTS availability_matrix;
CREATE SCHEMA  availability_matrix;
USE availability_matrix;

CREATE TABLE Category (
  id VARCHAR(36) NOT NULL,
  name VARCHAR(200) NOT NULL,
  PRIMARY KEY (id),
  UNIQUE INDEX id_UNIQUE (id ASC) VISIBLE,
  UNIQUE INDEX name_UNIQUE (name ASC) VISIBLE
);

CREATE TABLE Product (
    id VARCHAR(36) NOT NULL,
    name VARCHAR(200) NOT NULL,
    category_id VARCHAR(36) NOT NULL,
    PRIMARY KEY (id),
    UNIQUE INDEX id_UNIQUE (id ASC) VISIBLE,
    UNIQUE INDEX name_UNIQUE (name ASC) VISIBLE,
    INDEX product_fk0 (category_id)
);

CREATE TABLE Feature (
    id VARCHAR(36) NOT NULL,
    name VARCHAR(128) NOT NULL,
    product_id VARCHAR(36) NOT NULL,
    PRIMARY KEY (id),
    UNIQUE INDEX id_UNIQUE (id ASC) VISIBLE,
    UNIQUE INDEX name_UNIQUE (name ASC) VISIBLE,
    INDEX feature_fk0 (product_id)
);

CREATE TABLE Jurisdiction (
    id VARCHAR(36) NOT NULL,
    name VARCHAR(16) NOT NULL,
    title VARCHAR(200) NOT NULL,
    PRIMARY KEY (id),
    UNIQUE INDEX id_UNIQUE (id ASC) VISIBLE,
    UNIQUE INDEX name_UNIQUE (name ASC) VISIBLE,
    UNIQUE INDEX title_UNIQUE (title ASC) VISIBLE
);

CREATE TABLE Cell (
    id VARCHAR(36) NOT NULL,
    name VARCHAR(200) NOT NULL,
    csp VARCHAR(20) NOT NULL,
    country VARCHAR(10) NOT NULL,
    region VARCHAR(20) NOT NULL,
    csp_region VARCHAR(20) NOT NULL,
    jurisdiction_id VARCHAR(36) NOT NULL,
    PRIMARY KEY (id),
    UNIQUE INDEX id_UNIQUE (id ASC) VISIBLE,
    UNIQUE INDEX name_UNIQUE (name ASC) VISIBLE,
    INDEX cell_fk0 (jurisdiction_id)
);

CREATE TABLE StatusType (
    id VARCHAR(36) NOT NULL,
    name VARCHAR(200) NOT NULL,
    PRIMARY KEY (id),
    UNIQUE INDEX id_UNIQUE (id ASC) VISIBLE,
    UNIQUE INDEX name_UNIQUE (name ASC) VISIBLE
);

CREATE TABLE LifecycleStage (
    id VARCHAR(36) NOT NULL,
    name VARCHAR(200) NOT NULL,
    PRIMARY KEY (id),
    UNIQUE INDEX id_UNIQUE (id ASC) VISIBLE,
    UNIQUE INDEX name_UNIQUE (name ASC) VISIBLE
);

CREATE TABLE Availability (
     id VARCHAR(36) NOT NULL,
     # What is being made available
     item_id VARCHAR(36) NOT NULL,
     # Where it is being made available
     jurisdiction_id VARCHAR(36) NOT NULL,
     stage_id VARCHAR(36) NOT NULL,
     # The status type for this entry
     status_id VARCHAR(36) NOT NULL,
     comment VARCHAR(265),
     last_updated DATETIME NOT NULL,
     PRIMARY KEY (id),
     UNIQUE INDEX id_UNIQUE (id ASC) VISIBLE,
     INDEX availability_fk0 (stage_id),
     INDEX availability_fk1 (status_id),
     INDEX availability_fk2 (jurisdiction_id),
     INDEX availability_fk3 (item_id)
 );

SELECT UUID() INTO @ig_id;
INSERT INTO Category  VALUES (@ig_id, 'Identity Governance');

SELECT UUID() INTO @p_ud;
INSERT INTO Product  VALUES (@p_ud, 'Universal Directory',@ig_id);
INSERT INTO Feature VALUES (UUID(), 'Kerberos: Integrated Windows Auth', @p_ud);
INSERT INTO Feature VALUES (UUID(), 'Kerberos: Silent Activation', @p_ud);
INSERT INTO Feature VALUES (UUID(), 'Kerberos: Agentless Desktop SSO', @p_ud);
INSERT INTO Feature VALUES (UUID(), 'AD Del Auth', @p_ud);
INSERT INTO Feature VALUES (UUID(), 'LDAP Interface', @p_ud);

INSERT INTO Product  VALUES (UUID(), 'Lifecycle Management',@ig_id);
INSERT INTO Product  VALUES (UUID(), 'Secure Partner Access',@ig_id);
INSERT INTO Product  VALUES (UUID(), 'Okta Identity Governance',@ig_id);

SELECT UUID() INTO @am_id;
INSERT INTO Category  VALUES (@am_id, 'Access Management');

SELECT UUID() INTO @p_sso;
INSERT INTO Product  VALUES (@p_sso, 'Single Sign-On',@am_id);
INSERT INTO Feature VALUES (UUID(), 'FastPass', @p_sso);
INSERT INTO Feature VALUES (UUID(), 'O365', @p_sso);
INSERT INTO Feature VALUES (UUID(), 'O365 GCC High (Profile Sync and icense and Role Management)', @p_sso);
INSERT INTO Feature VALUES (UUID(), 'O365 GCC High (User and Universal Sync)', @p_sso);
INSERT INTO Feature VALUES (UUID(), 'O365 DOD', @p_sso);

SELECT UUID() INTO @p_amfa;
INSERT INTO Product  VALUES (@p_amfa, 'Adaptive MFA',@am_id);
INSERT INTO Feature VALUES (UUID(), 'Device Trust', @p_amfa);
INSERT INTO Feature VALUES (UUID(), 'Desktop MFA', @p_amfa);
INSERT INTO Feature VALUES (UUID(), 'Policy Recommender with Okta AI', @p_amfa);
INSERT INTO Feature VALUES (UUID(), 'Pre-enrolled Yubikeys', @p_amfa);
INSERT INTO Feature VALUES (UUID(), 'SMS / Voice Authenticator', @p_amfa);
INSERT INTO Feature VALUES (UUID(), 'Email Authenticator', @p_amfa);
INSERT INTO Feature VALUES (UUID(), 'Okta Verify', @p_amfa);
INSERT INTO Feature VALUES (UUID(), 'Okta Mobile (on Classic)', @p_amfa);

INSERT INTO Product  VALUES (UUID(), 'API Access Management',@am_id);
INSERT INTO Product  VALUES (UUID(), 'Okta Access Gateway',@am_id);
INSERT INTO Product  VALUES (UUID(), 'Okta Device Accews',@am_id);
INSERT INTO Product  VALUES (UUID(), 'Identity Threat Protection',@am_id);

SELECT UUID() INTO @ci_id;
INSERT INTO Category  VALUES (@ci_id, 'Customer Identity');
INSERT INTO Product  VALUES (UUID(), 'Customer Identity Solution',@ci_id);

SELECT UUID() INTO @platform_id;
INSERT INTO Category  VALUES (@platform_id, 'Platform');
INSERT INTO Product  VALUES (UUID(), 'Okta Workflows',@platform_id);

SELECT UUID() into @g2k_id;
INSERT INTO Product  VALUES (@g2k_id, 'Okta for Global 2000',@platform_id);
INSERT INTO Feature VALUES (UUID(), 'Org2Org', @g2k_id);
INSERT INTO Feature VALUES (UUID(), 'Ariel', @g2k_id);

INSERT INTO Product  VALUES (UUID(), 'Multi Org Deployment',@platform_id);
INSERT INTO Product  VALUES (UUID(), 'Enhanced Disaster Recovery',@platform_id);
INSERT INTO Product  VALUES (UUID(), 'Dynamic Scale',@platform_id);
INSERT INTO Product  VALUES (UUID(), 'Log Investigator with Okta AI',@platform_id);

SELECT UUID() INTO @standard_id;
INSERT INTO Product  VALUES (@standard_id, 'Standard Okta Offerings (No SKU)',@platform_id);
INSERT INTO Feature  VALUES (UUID(), 'Custom Domains',@standard_id);
INSERT INTO Feature  VALUES (UUID(), 'Customer Email Domains',@standard_id);
INSERT INTO Feature  VALUES (UUID(), 'Standard Disaster Recovery',@standard_id);
INSERT INTO Feature  VALUES (UUID(), 'Protected Actions',@standard_id);
INSERT INTO Feature  VALUES (UUID(), 'Log Streaming for EventBridge',@standard_id);
INSERT INTO Feature  VALUES (UUID(), 'Log Streaming for Splunk Cloud',@standard_id);
INSERT INTO Feature  VALUES (UUID(), 'Interactive Reports (Reports 2.0)',@standard_id);
INSERT INTO Feature  VALUES (UUID(), 'Cloud Provisioning Connector',@standard_id);

SELECT UUID() INTO @pa_id;
INSERT INTO Category  VALUES (@pa_id, 'Priveleged Access');
INSERT INTO Product  VALUES (UUID(), 'Okta Privileged Acces',@pa_id);
INSERT INTO Product  VALUES (UUID(), 'Advanced Server Access',@pa_id);

INSERT INTO Category  VALUES (UUID(), 'Okta Personal');
INSERT INTO Category  VALUES (UUID(), 'Identity Security Posture Management');

SELECT UUID() INTO @j_nac;
INSERT INTO Jurisdiction VALUES (@j_nac, 'aws_nac', 'North America Commercial');
SELECT UUID() INTO @j_frm;
INSERT INTO Jurisdiction VALUES (@j_frm, 'aws_frm', 'Okta for Government Moderate & HIPAA');
SELECT UUID() INTO @j_frh;
INSERT INTO Jurisdiction VALUES (@j_frh, 'aws_frh', 'Okta for Government High');
SELECT UUID() INTO @j_il4;
INSERT INTO Jurisdiction VALUES (@j_il4, 'aws_il4', 'Okta for U.S. Military');
SELECT UUID() INTO @j_emea;
INSERT INTO Jurisdiction VALUES (@j_emea, 'aws_emea', 'EMEA');
SELECT UUID() INTO @j_apj;
INSERT INTO Jurisdiction VALUES (@j_apj, 'aws_apj', 'APJ');
SELECT UUID() INTO @j_gcp_nac;
INSERT INTO Jurisdiction VALUES (@j_gcp_nac, 'gcp_nac', 'GCP North America Commercial');

SELECT UUID() INTO @ok1;
INSERT INTO Cell VALUES (@ok1, 'OK1','AWS', 'AMER', 'USA', 'us-east-1', @j_nac);
SELECT UUID() INTO @ok2;
INSERT INTO Cell VALUES (@ok2, 'OK2','AWS', 'AMER', 'USA', 'us-east-1', @j_nac);
SELECT UUID() INTO @ok3;
INSERT INTO Cell VALUES (@ok3, 'OK3','AWS', 'AMER', 'USA', 'us-east-1', @j_nac);
SELECT UUID() INTO @ok4;
INSERT INTO Cell VALUES (@ok4, 'OK4','AWS', 'AMER', 'USA', 'us-east-1', @j_nac);
SELECT UUID() INTO @ok6;
INSERT INTO Cell VALUES (@ok6, 'OK6','AWS', 'AMER', 'USA', 'us-east-2', @j_nac);
SELECT UUID() INTO @ok11;
INSERT INTO Cell VALUES (@ok11, 'OK11','AWS', 'AMER', 'USA', 'us-east-2', @j_nac);
SELECT UUID() INTO @ok7;
INSERT INTO Cell VALUES (@ok7, 'OK7','AWS', 'AMER', 'USA', 'us-west-2', @j_nac);
SELECT UUID() INTO @ok12;
INSERT INTO Cell VALUES (@ok12, 'OK12','AWS', 'AMER', 'USA', 'us-west-2', @j_nac);
SELECT UUID() INTO @ok14;
INSERT INTO Cell VALUES (@ok14, 'OK14','AWS', 'AMER', 'USA', 'us-west-2', @j_nac);
SELECT UUID() INTO @ok9;
INSERT INTO Cell VALUES (@ok9, 'OK9','AWS', 'EMEA', 'IRE', 'eu-west-1', @j_emea);
SELECT UUID() INTO @eu1;
INSERT INTO Cell VALUES (@eu1, 'EU1','AWS', 'EMEA', 'GER', 'eu-central-1', @j_emea);
SELECT UUID() INTO @ok5;
INSERT INTO Cell VALUES (@ok5, 'OK5','AWS', 'AMER', 'USA', 'us-west-2', @j_frm);
SELECT UUID() INTO @ok10;
INSERT INTO Cell VALUES (@ok10, 'OK10','AWS', 'AMER', 'USA', 'us-east-2', @j_frm);
SELECT UUID() INTO @og1;
INSERT INTO Cell VALUES (@og1, 'OG1','AWS', 'AMER', 'USA', 'us-gov-west-1', @j_frh);
SELECT UUID() INTO @om1;
INSERT INTO Cell VALUES (@om1, 'OM1','AWS', 'AMER', 'USA', 'us-gov-east-1', @j_il4);
SELECT UUID() INTO @ok8;
INSERT INTO Cell VALUES (@ok8, 'OK8','AWS', 'APJ', 'AUS', 'ap-southeast-2', @j_apj);
SELECT UUID() INTO @ok16;
INSERT INTO Cell VALUES (@ok16, 'OK16','AWS', 'APJ', 'JAP', 'ap-nrtheast-1', @j_apj);
SELECT UUID() INTO @ok17;
INSERT INTO Cell VALUES (@ok17, 'OK17','GCP', 'AMER', 'USA', 'us-east4', @j_gcp_nac);

SELECT UUID() INTO @beta;
INSERT INTO LifecycleStage VALUES (@beta, 'BETA');
SELECT UUID() INTO @lea;
INSERT INTO LifecycleStage VALUES (@lea, 'LEA');
SELECT UUID() INTO @ea;
INSERT INTO LifecycleStage VALUES (@ea, 'EA');
SELECT UUID() INTO @ga;
INSERT INTO LifecycleStage VALUES (@ga, 'GA');

SELECT UUID() INTO @not_available;
INSERT INTO StatusType VALUES (@not_available, 'Not Available');

SELECT UUID() INTO @available;
INSERT INTO StatusType VALUES (@available, 'Available');

SELECT UUID() INTO @audit_ready;
INSERT INTO StatusType VALUES (@audit_ready, 'Audit Ready');
SELECT UUID() INTO @authorized;
INSERT INTO StatusType VALUES (@authorized, 'Authorized');

# North America Commercial:  @j_nac;
# FedRAMP Moderate/HIPAA: @j_frm;
# FedRAMP High: @j_frh;
# IL4: @j_il4;
# EMEA{ @j_emea;
# APJ: INTO @j_apj;
# Google Cloud Platform - North America Commercial: @j_gcp_nac;

SELECT UTC_TIMESTAMP() INTO @now;
INSERT INTO Availability (id, item_id, jurisdiction_id, stage_id, status_id,last_updated) VALUES (UUID(), @p_ud, @j_nac, @ga, @available, @now);
INSERT INTO Availability (id, item_id, jurisdiction_id, stage_id, status_id,last_updated) VALUES (UUID(), @p_ud, @j_frm, @ga, @available, @now);
INSERT INTO Availability (id, item_id, jurisdiction_id, stage_id, status_id,last_updated) VALUES (UUID(), @p_ud, @j_frh, @ga, @available, @now);
INSERT INTO Availability (id, item_id, jurisdiction_id, stage_id, status_id,last_updated) VALUES (UUID(), @p_ud, @j_il4, @ga, @available, @now);
INSERT INTO Availability (id, item_id, jurisdiction_id, stage_id, status_id,last_updated) VALUES (UUID(), @p_ud, @j_emea, @ga, @available, @now);
INSERT INTO Availability (id, item_id, jurisdiction_id, stage_id, status_id,last_updated) VALUES (UUID(), @p_ud, @j_apj, @ga, @available, @now);
INSERT INTO Availability (id, item_id, jurisdiction_id, stage_id, status_id, comment, last_updated) VALUES (UUID(), @p_ud, @j_gcp_nac, @ga, @not_available, 'EA Planned for Oct', @now);
