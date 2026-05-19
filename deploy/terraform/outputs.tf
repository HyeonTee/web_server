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
