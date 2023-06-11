# Portrait Booth Management for Events

Rewritten in rust with new features

## Overview

Manage onsite portrait booths with offsite processing for large events.

## Workflow

1. Customer registration and orderding with stripe payments.  
Customers can pre-purchase number of photos before they get their pictures taken.
2. Photos uploaded for offsite processing.  
Photos will be stored in S3 storage with pre-signed links for downloading.
3. Onces photos are processed, they'll be uploaded back to S3 storage and pre-signed linkes will be emailed to the customers.

## Features

1. Rust api backend written with Axum/Leptos Server Functions.
2. Rust frontend written with Leptos.
3. S3 integration for storage backend to store original and processed photos.
4. Presigned expiring urls using S3.  
5. OAuth2 authentication for onsite operators and offsite processors.
6. Stripe integration for payments.
