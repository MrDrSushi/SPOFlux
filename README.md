# SPOFlux

A demo project showcasing integration between **Rust**, **SharePoint Online**, and **Microsoft Graph API**.

## 📚 Table of Contents

- [Purpose](#purpose)
- [Overview](#overview)
- [Key Features](#key-features)
- [Technologies](#technologies)
- [Getting Started](#getting-started)
- [License](#license)
- [Contributing](#contributing)

## Purpose 🎯

SPOFlux is a demonstration project designed to explore and showcase how to build efficient, modern applications that interact with Microsoft's cloud ecosystem. Specifically, it demonstrates best practices for:

- Building robust applications in **Rust** with strong type safety and memory efficiency
- Authenticating and communicating with **SharePoint Online** via the Microsoft Graph API
- Handling enterprise-grade document management and collaboration workflows
- Implementing secure, scalable patterns for cloud-native applications

## Overview 🔍

This project serves as a proof-of-concept for developers interested in:

1. **Learning Rust**: Understanding how a systems programming language like Rust can handle async/await patterns and complex API integrations.
2. **Graph API Integration**: Exploring how to leverage Microsoft Graph API to access SharePoint Online resources programmatically.
3. **Enterprise Workflows**: Demonstrating practical examples of document operations, metadata management, and collaboration features in SharePoint.

## Key Features ⭐

- **Rust-based**: Compiled for performance, safety, and reliability.
- **Graph API Integration**: Direct integration with Microsoft Graph API for SharePoint Online operations.
- **Async Support**: Non-blocking operations for scalable API calls (using the Tokio crate).
- **Type-Safe**: Leveraging Rust's type system for compile-time guarantees.

## 🛠️ Technologies

- **Language**: Rust
- **API**: Microsoft Graph API
- **Cloud Service**: SharePoint Online (Office 365)
- **Architecture**: Async/await pattern with modern error handling

## 🚀 Getting Started

The SharePoint Online List template can be found in the "Data" folder. There is a PowerShell script to quickly create the list in your tenant, or you can simply upload the template and create the list manually.

### Steps to Set Up:

1. Copy the following files from the **Data** folder to the same location as `SPOFlux.exe`:

   - `settings.json`
   - `world-data-Airports.csv`
   - `world-data-Countries.csv`
   - `world-data-Locations.csv`
   - `world-data-Ports.csv`

2. Edit the `settings.json` file to match the configuration of your M365 Tenant:

   ```json
   {
       "SPORootSite": "tenant-name.sharepoint.com", // URL for your SharePoint Online, also known as the SharePoint Tenant
       "SPOSite": "site-name", // The name of your SPO Site Collection
       "SPOList": "list-name", // The name of the SPO List to be used
       "tenant_id": "xxxxx-xxxxx-xxxxx-xxxxx", // Tenant ID - can be found at the Azure Portal -> Entra
       "tenant_domain": "tenant-domain.com", // Tenant URL - can be found at the Azure Portal -> Entra
       "client_id": "xxxxx-xxxxx-xxxxx-xxxxxx", // ID of the registered Entra App
       "client_secret": "xxxxx", // Client Secret of the registered Entra App
       "client_thumbprint": "xxxxx", // Thumbprint of the Certificate of the registered Entra App
       "entra_applicationname": "Entra Application Name", // Name of registered Entra App
       "certificate_password": "Entra App Password", // Password of certificate for the registered Entra App
       "soft_run": true, // When set to "true" SPOFlux.exe will create items in SPO, otherwise will just output to console (simulated mode)
       "total_records": 1000 // The total number of records to be created; the program is configured to batch 20 records per request
   }
   ```

## 📝 License

[![MIT License](https://img.shields.io/badge/License-MIT-green.svg)](https://choosealicense.com/licenses/mit/)
[![GPLv3 License](https://img.shields.io/badge/License-GPL%20v3-yellow.svg)](https://opensource.org/licenses/)
[![AGPL License](https://img.shields.io/badge/license-AGPL-blue.svg)](http://www.gnu.org/licenses/agpl-3.0)


## 🤝 Contributing

Contributions, suggestions, and feedback are welcome! Feel free to open issues or pull requests.

---

*This is a demo project for educational and exploration purposes.*
