data "aws_caller_identity" "current" {}

check "account_id" {
  assert {
    condition     = var.expected_account_id == null || var.expected_account_id == data.aws_caller_identity.current.account_id
    error_message = "Wrong AWS account: expected ${coalesce(var.expected_account_id, "<unset>")}, got ${data.aws_caller_identity.current.account_id}. Verify aws_profile / AWS_PROFILE."
  }
}
