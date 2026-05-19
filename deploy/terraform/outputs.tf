output "public_ip" {
  description = "Elastic IP of the EC2 instance"
  value       = aws_eip.web.public_ip
}

output "ssh_command" {
  description = "Convenience SSH command"
  value       = "ssh ubuntu@${aws_eip.web.public_ip}"
}

output "ecr_repository_url" {
  description = "ECR repository URL (use for docker tag/push)"
  value       = aws_ecr_repository.app.repository_url
}

output "route53_nameservers" {
  description = "Set these at your domain registrar if create_hosted_zone = true"
  value       = local.zone_ns
}

output "github_deploy_role_arn" {
  description = "Register as AWS_DEPLOY_ROLE_ARN variable in GitHub Actions"
  value       = aws_iam_role.github_deploy.arn
}

output "github_plan_role_arn" {
  description = "Register as AWS_PLAN_ROLE_ARN variable in GitHub Actions"
  value       = aws_iam_role.github_plan.arn
}

output "ec2_instance_id" {
  description = "Used by GitHub Actions deploy workflow to target SSM Send-Command"
  value       = aws_instance.web.id
}
