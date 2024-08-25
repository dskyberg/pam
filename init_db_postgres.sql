
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
    description character varying(128) NOT NULL,
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

CREATE INDEX "availability_item_idx" on "availability" (item_id);
CREATE INDEX "availability_jurisdiction_idx" on "availability" (jurisdiction_id);
CREATE INDEX "availability_lifecycle_idx" on "availability" (lifecycle_id);
CREATE INDEX "availability_compliance_idx" on "availability" (compliance_id);

CREATE TABLE "comment" (
   id character(36) NOT NULL,
   item_id character(36) NOT NULL,
   text character varying(256) NOT NULL,
   created_by character varying(128),
   created timestamp NOT NULL,
   CONSTRAINT "comment_pkey" PRIMARY KEY (id)
);

CREATE INDEX "comment_item_idx" on "comment" (item_id);
CREATE INDEX "comment_created_idx" on "comment" (created);
