# How gum works

https://napi.rs/docs/introduction/simple-package

```mermaid
graph TD
    subgraph User Actions
        U1[Run: gum new everything-gold]
        U2[Make changes to project files]
        U3[View compiled assets]
    end

    subgraph Initial Setup
        A1[Generate project folder wi  th gumentry.ts]
        A2[Set up Deno project]
        A3[Initialize gum-ts TypeScript bridge]
        A4[Set up napi.rs for FFI]
        A5[Load FFIed libraries from gum into gumentry.ts]
    end

    subgraph Build & Monitor
        B1[Start gum watch to monitor file changes]
        B2{File changes detected?}
        B3[Execute gumentry.ts]
        B4[Use FFIed libraries for IPC with gum CLI]
        B5[Send root definitions data via IPC]
        B6[Compile assets]
        B7[Precompile assets]
        B8[Transfer to Bedrock directory for addons]
    end

    %% Define connections between subgraphs
    U1 --> A1
    A1 --> A2
    A2 --> A3
    A3 --> A4
    A4 --> A5
    A5 --> B1
    U2 --> B2
    B2 -->|Yes| B3
    B3 --> B4
    B4 --> B5
    B5 --> B6
    B6 --> B7
    B7 --> B8
    B8 --> U3

    %% Style definitions
    style User Actions fill:#f9f,stroke:#333,stroke-width:2px
    style Initial Setup fill:#ccf,stroke:#333,stroke-width:2px
    style Build & Monitor fill:#9cf,stroke:#333,stroke-width:2px

```