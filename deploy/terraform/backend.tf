# Backend config cannot reference variables — these values are duplicated from
# backend_bootstrap.tf (locals.tfstate_bucket_name / tfstate_lock_table) and
# must stay in sync if project_name ever changes.
terraform {
  backend "s3" {
    bucket         = "web-server-tfstate-588738611832"
    key            = "deploy/terraform/terraform.tfstate"
    region         = "ap-northeast-2"
    dynamodb_table = "web-server-tfstate-lock"
    encrypt        = true
  }
}
