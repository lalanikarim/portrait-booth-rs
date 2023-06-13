# Portrait Booth Management for Events

Rewritten in rust with new features

## Overview

Manage onsite portrait booths with offsite processing for large events.

## Workflow

1. Customer registration and ordering with stripe payments.  
Customers can pre-purchase number of photos before they get their pictures taken.
2. Photos uploaded for offsite processing.  
Photos will be stored in S3 storage with pre-signed links for downloading.
3. Once photos are processed, they'll be uploaded back to S3 storage and pre-signed links will be emailed to the customers.

## Manager Flow - Pre Setup

1. Create Booth.
2. Activate User accounts and assign roles (Operator and Processor).

## Customer Flow

1. Access the application, likely through a QR code.
2. Enter profile information and provide at least either an email or a phone number and verify using OTP.
3. Create an order and select number of photos. Order number is created. Order status is Created.
4. Pay for the order using Stripe. Order status is Paid.
5. Present the Order number to the booth operator.

## Operator Flow

1. Verify Order is paid and number of photos selected.
2. Upload raw photos to the order. Order status is Uploaded.

## Processor Flow

1. Access unprocessed orders and download photos. Order status is In Progress.
2. Process photos and upload processed photos to the order. Order status is Processed.

## Manager Flow - Post Processing

1. Verify orders.
2. Trigger emails with links of processed photos. Order status is Completed.

## Features

1. Rust API backend written with Axum/Leptos Server Functions.
2. Rust frontend written with Leptos.
3. S3 integration for storage backend to store original and processed photos.
4. Presigned expiring URLs using S3.  
5. OAuth2 authentication for onsite operators and offsite processors.
6. Stripe integration for payments.
