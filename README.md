# pg_auto_dw

<img src="https://tembo.io/_astro/graphs.CNZLRuSs_Z1YDvaO.webp" style="border-radius: 30px; width: 600px; height: auto;">

[Open-source](LICENSE) PostgreSQL Extension for Automated Data Warehouse Creation

[![Static Badge](https://img.shields.io/badge/%40tembo-community?logo=slack&label=slack)](https://join.slack.com/t/tembocommunity/shared_invite/zt-277pu7chi-NHtvHWvLhHwyK0Y5Y6vTPw)
[![OSSRank](https://shields.io/endpoint?url=https://ossrank.com/shield/4020)](https://ossrank.com/p/4020)
[![Warning: Under Active Development](https://img.shields.io/badge/Warning-Under_Active_Development-red)](https://github.com/tembo-io/pg_auto_dw)

## Background

From [@ryw](https://github.com/ryw) 4-18-24:

> This project attempts to implement an idea I can't shake - an auto-data warehouse extension that uses LLM to inspect operational Postgres schemas, and sets up automation to create a well-formed data warehouse (whether it's Data Vault, Kimball format, etc. I don't care - just something better than a dumb dev like me would build as a DW - a pile of ingested tables, and ad-hoc derivative tables). I don't know if this project will work, but kind of fun to start something without certainty of success. But I have wanted this badly for years as a dev + data engineer.

## Project Vision

To create an open source extension that automates the data warehouse build.  We aim to do this within a structured environment that incorporates best practices and harnesses the capabilities of Large Language Models (LLM) technologies.

**Goals:** This extension will enable users to: 
- Automate the DW Build
- Automate DW Maintenance  
- Understand DW Health
- Support Data Governance

All these capabilities will be delivered through a [small set of intuitive functions](extension/docs/sql_functions/readme.md).

## Principles

* Build in public
    * Public repo
    * Call attention/scrutiny to the work - release every week or two with blog/tweet calling attention to your work
* Documentation-driven development
    * While you’re writing code, write docs on how the product works
* Demo-driven development (recursive, go back to step 1 as needed. It's OK to get things wrong and iterate.)
    * Step 1 - write demo script
    * Step 2 - document vision + demo script in public README
    * Step 3 - mock up demo (fake UX)
    * Step 4 - make fake demo real (narrow use case)
    * Step 5 - ship v0.1 (SemVer) that can do a little more than just the pre-canned demo
        * Ship product + demo video + documentation

## Data Vault

We are starting with automation to facilitate a data vault implementation for our data warehouse. This will be a rudimentary raw vault setup, but we hope it will lead to substantial downstream business models.

## Timeline

We're currently working on a timeline to define points of success and ensure the smooth integration of new contributors to our project. This includes creating milestones, contributor guidelines, and hosting activities such as webinars and meetups. Stay tuned!

## Installation

We are currently developing a new extension, starting with an initial set of defined [functions](extension/docs/sql_functions/readme.md) and implementing a subset of these functions in a mockup extension. This mockup version features skeletal implementations of some functions, designed just to demonstrate our envisioned capabilities as seen in the demo below. Our demo is divided into two parts: Act 1 and Act 2.  If you follow along, I hope this will offer a glimpse of what to expect in the weeks ahead.

If you’re interested in exploring this preliminary version, please follow these steps:

1) [Install Rust](https://doc.rust-lang.org/book/ch01-01-installation.html)
2) [Install PGRX](https://github.com/pgcentralfoundation/pgrx)
3) Run this Codebase

## Demo: Act 1 - "1-Click Build"

> **Note:** Only use the code presented below. Any deviations may cause errors. This demo is for illustrative purposes only. It is currently tested on PGRX using the default PostgreSQL 13 instance.

We want to make building a data warehouse easy.  And, if the source tables are well-structured and appropriately named, constructing a data warehouse can be achieved with a single call to the extension.

1. **Install Extension**

```SQL
/* Installing Extension - Installs and creates sample source tables. */
CREATE EXTENSION pg_auto_dw CASCADE;
```

> **Note:** Installing this extension installs a couple source sample tables in the PUBLIC SCHEMA as well as the PG_CRYPTO extension.

2. **Build Data Warehouse**

```SQL
/* Build me a Data Warehouse for tables that are Ready to Deploy */
SELECT auto_dw.go();
```

> **Note:** This will provide a build ID and some helpful function tips.  Do not implement these tips at this time. They are for illustrative purposes of future functionality.

3. **Data Warehouse Built**

```SQL
/* Data Warehouse Built - No More Code Required */
```

```mermaid
flowchart LR
    Start(("Start")) --> ext["Install Extension"]
    ext -- #10711; --> build["Build Data Warehouse\nauto_dw.go()"]
    build -- #10711; --> DW[("DW Created")]
    DW --> Done(("Done"))
```

## Demo: Act 2 - “Auto Data Governance”

Sometimes it’s best to get a little push-back when creating a data warehouse, which supports appropriate data governance.  In this instance a table was not ready to deploy to the data warehouse as a table column may need to be considered sensitive and handled appropriately.  In this sample script, Auto DW’s engine understands the attribute is useful for analysis, but also may need to be considered sensitive.  In this script the user will:

1) **Identify a Skipped Table**

```SQL
/* Identify source tables skipped and not integration into the data warehouse. */
SELECT schema, "table", status, status_response 
FROM auto_dw.source_table()
WHERE status_code = 'SKIP';
```

> **Note:** Running this code will provide an understanding of which table was skipped along with a high level reason.  You should see the following output from the status_response: “Source Table was skipped as column(s) need additional context. Please run the following SQL query for more information: SELECT schema, table, column, status, status_response FROM auto_dw.source_status_detail() WHERE schema = 'public' AND table = 'customers'.”

2) **Identify the Root Cause**

```SQL
/* Identify the source table column that caused the problem, understand the issue, and potential solution. */
SELECT schema, "table", "column", status, confidence_level, status_response
FROM auto_dw.source_column()
WHERE schema = 'PUBLIC' AND "table" = 'CUSTOMER';
```

> **Note:** Running this code will provide an understanding of which table column was skipped along with a reason in the status_response.  You should see the following output: “Requires Attention: Column cannot be appropriately categorized as it may contain sensitive data.  Specifically, if the zip is an extended zip it may be considered PII.”

3) **Decide to Institute Some Data Governance Best Practices**

```SQL
/* Altering column length restricts the acceptance of extended ZIP codes.*/
ALTER TABLE customer ALTER COLUMN zip TYPE VARCHAR(5);
```

> **Note:** Here the choice was up to the user to make a change that facilitated LLM understanding of data sensitivity.  In this case, limiting the type to VARCHAR(5) will allow the LLM to understand that this column will not contain sensitive information in the future. 

```mermaid
flowchart LR
    Start(("Start")) --> tbl["Identify a Skipped Table\nauto_dw.source_table()"]
    tbl --> col["Identify the Root Cause\nauto_dw.source_column()"]
    col --> DW[("Institute Data Governance\nBest Practices")]
    DW --> Done(("Done"))
```

**Auto DW Process Flow:** The script highlighted in Act 2 demonstrates that there are several approaches to successfully implementing a data warehouse when using this extension. Below is a BPMN diagram that illustrates these various paths.

```mermaid
flowchart LR
 subgraph functions_informative["Informative Functions"]
    direction LR
        health["auto_dw.health()"]
        source_tables["auto_dw.source_tables()"]
        source_column["auto_dw.source_column()"]
  end
 subgraph functions_interactive["Interactive Functions"]
    direction LR
        source_clude["auto_dw.source_include(object_pattern)"]
        update_context["auto_dw.update_context(object, context)"]
        go["auto_dw.go(flag, status)"]
  end
 subgraph data_gov["Data Governance"]
    direction BT
        to_gov{"X"} --> gov["Issue\nGovernance"]
  end
    start(("Start")) --> command["Choose Command"]
    command --> split{"X"}
    split --> health & source_tables & source_column & source_clude & update_context & go --> join{"X"}
    join --> review["Review Results"]
    review --> data_gov --> more_auto{"More\nAutomations?"} 
    more_auto --> |no| done(("Done"))
    more_auto --> |yes| start_again(("Restart"))
```
