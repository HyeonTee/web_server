variable "region" {
  description = "AWS region"
  type        = string
  default     = "ap-northeast-2"
}

variable "aws_profile" {
  description = "Named AWS profile from ~/.aws/credentials. null = use default credential chain."
  type        = string
  default     = null
}

variable "expected_account_id" {
  description = "Fail-safe: terraform aborts when the active credentials map to a different account."
  type        = string
  default     = null
}

variable "project_name" {
  description = "Used as a prefix for resource names"
  type        = string
  default     = "web-server"
}

variable "domain_name" {
  description = "Apex domain to point at the server (e.g. example.com)"
  type        = string
}

variable "ec2_instance_type" {
  description = "EC2 instance type"
  type        = string
  default     = "t3.micro"
}

variable "ssh_pubkey" {
  description = "Public key contents (e.g. cat ~/.ssh/id_ed25519.pub)"
  type        = string
}

variable "allowed_ssh_cidr" {
  description = "CIDR allowed to SSH. Strongly recommend restricting to your own IP /32."
  type        = string
  default     = "0.0.0.0/0"
}

variable "create_hosted_zone" {
  description = "Create a new Route53 hosted zone for domain_name. Set false to reuse an existing one."
  type        = bool
  default     = true
}

variable "github_owner" {
  description = "GitHub user/org that owns the repo (used in OIDC trust policy)"
  type        = string
  default     = "HyeonTee"
}

variable "github_repo" {
  description = "GitHub repository name (used in OIDC trust policy)"
  type        = string
  default     = "web_server"
}

variable "tags" {
  description = "Default tags applied to all resources"
  type        = map(string)
  default = {
    Project   = "web-server"
    ManagedBy = "terraform"
  }
}
