# GitHub Actions setup

Three workflows live here:

| Workflow | Trigger | What it does |
|---|---|---|
| `ci.yml` | every PR + push to `main` | cargo fmt/clippy/test + terraform fmt/validate |
| `deploy.yml` | push to `main` (code paths) + manual dispatch | build amd64 image → push to ECR → SSM `docker pull && systemctl restart` → smoke test |
| `terraform-plan.yml` | PRs touching `deploy/terraform/**` | `terraform plan` posted as a PR comment |

## One-time setup

### 1. Apply the new Terraform resources

The OIDC provider, two IAM roles, and the EC2 `AmazonSSMManagedInstanceCore`
attachment must exist in AWS before any workflow can run.

```sh
cd deploy/terraform
terraform plan
terraform apply
```

Note the new outputs:
```sh
terraform output github_deploy_role_arn
terraform output github_plan_role_arn
terraform output ec2_instance_id
terraform output ecr_repository_url
```

### 2. Register repository **Variables** (not secrets — these aren't sensitive)

In GitHub: **Settings → Secrets and variables → Actions → Variables tab → New repository variable**

| Name | Value | Source |
|---|---|---|
| `AWS_REGION` | `ap-northeast-2` | constant |
| `AWS_DEPLOY_ROLE_ARN` | `arn:aws:iam::…:role/web-server-gha-deploy` | `terraform output github_deploy_role_arn` |
| `AWS_PLAN_ROLE_ARN` | `arn:aws:iam::…:role/web-server-gha-plan` | `terraform output github_plan_role_arn` |
| `ECR_REPOSITORY_URL` | `…dkr.ecr.ap-northeast-2.amazonaws.com/web-server` | `terraform output ecr_repository_url` |
| `EC2_INSTANCE_ID` | `i-…` | `terraform output ec2_instance_id` |

No secrets are needed — OIDC handles AWS auth, and these variables aren't sensitive enough to need encryption.

### 3. Push and watch

```sh
git push origin main
# → Actions tab → "deploy" workflow should run end-to-end
```

If the smoke test step at the end is green, https://hyeontae.cloud is serving the new build.

## Trust policy details

- `gha-deploy` role only trusts subjects matching
  `repo:HyeonTee/web_server:ref:refs/heads/main`. Pull request workflows can't
  assume it — they can only assume `gha-plan` (read-only).
- `gha-plan` trusts `repo:HyeonTee/web_server:pull_request`. Even if a fork
  opens a PR, it cannot reach prod — read-only is the worst case.

If you rename the repo or fork it under a different owner, update
`github_owner` / `github_repo` variables and `terraform apply`.

## Troubleshooting

- **"Not authorized to perform sts:AssumeRoleWithWebIdentity"** — the workflow's
  `sub` claim doesn't match what the trust policy expects. Check the workflow
  run logs (right after `Configure AWS credentials` step) for the actual `sub`
  it sent.
- **SSM Send-Command says "InvalidInstanceId"** — instance was recreated.
  `terraform apply` then update the `EC2_INSTANCE_ID` variable in GitHub.
- **Image push 403** — the `gha-deploy` policy only allows pushing to the
  exact ECR repo Terraform created. If you renamed the repo, update via
  `terraform apply`.
