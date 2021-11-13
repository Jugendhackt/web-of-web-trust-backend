[![](https://img.shields.io/badge/Version-0.1.0b0-black?style=flat-square)]() [![](https://img.shields.io/badge/License-AGPL%203-black?style=flat-square&logo=gnu)](https://www.gnu.org/licenses/agpl-3.0.en.html) [![](https://img.shields.io/badge/OpenAPI%20Version-3.0.2-black?style=flat-square&logo=openapiinitiative)](https://github.com/Jugendhackt/web-of-web-trust-backend/blob/main/schema.yml) [![](https://img.shields.io/badge/Rust-2021-black?style=flat-square&logo=rust)](https://doc.rust-lang.org/edition-guide/rust-2021/index.html)


# Web of Web Trust backend

This is an experimental backend for the web of web trust browser extension. This project is intended to allow for a practical study of new technologies for this project *and* not as a production grade system.

## Architecture

```mermaid
flowchart LR
    subgraph internet
    Extension -- HTTP API --> N{Nginx}    
    end
    subgraph docker
    N --> S{Backend} 
    S -- KV-Cache  --> C{{Redis}}
    S -- Tracing --> T{{Jeager}}
    S -- Database  --> D[(PostgreSQL)]
    end
```