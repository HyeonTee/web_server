# Resources that hold Terraform's own state.
# First created with local state; state is then migrated into the S3 bucket
# below via `terraform init -migrate-state`. From that point on, Terraform
# reads/writes its state from S3 with DynamoDB-based locking.

locals {
  tfstate_bucket_name = "${var.project_name}-tfstate-${data.aws_caller_identity.current.account_id}"
  tfstate_lock_table  = "${var.project_name}-tfstate-lock"
}

resource "aws_s3_bucket" "tfstate" {
  bucket = local.tfstate_bucket_name

  # Destroying this bucket would lose all Terraform state.
  lifecycle {
    prevent_destroy = true
  }
}

resource "aws_s3_bucket_versioning" "tfstate" {
  bucket = aws_s3_bucket.tfstate.id
  versioning_configuration {
    status = "Enabled"
  }
}

resource "aws_s3_bucket_server_side_encryption_configuration" "tfstate" {
  bucket = aws_s3_bucket.tfstate.id
  rule {
    apply_server_side_encryption_by_default {
      sse_algorithm = "AES256"
    }
  }
}

resource "aws_s3_bucket_public_access_block" "tfstate" {
  bucket                  = aws_s3_bucket.tfstate.id
  block_public_acls       = true
  block_public_policy     = true
  ignore_public_acls      = true
  restrict_public_buckets = true
}

resource "aws_dynamodb_table" "tfstate_lock" {
  name           = local.tfstate_lock_table
  billing_mode   = "PROVISIONED"
  read_capacity  = 1
  write_capacity = 1
  hash_key       = "LockID"

  attribute {
    name = "LockID"
    type = "S"
  }

  lifecycle {
    prevent_destroy = true
  }
}

output "tfstate_bucket" {
  description = "S3 bucket holding Terraform state"
  value       = aws_s3_bucket.tfstate.id
}

output "tfstate_lock_table" {
  description = "DynamoDB table used for Terraform state locking"
  value       = aws_dynamodb_table.tfstate_lock.id
}
