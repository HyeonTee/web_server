# Ansible deploy

Provisions the EC2 instance Terraform created — installs Docker, nginx, certbot, and runs the `web_server` container as a systemd service.

## Prerequisites

- Terraform `apply` finished, `terraform output public_ip` returns the EIP.
- DNS for `domain_name` already resolves to that EIP (`dig +short A <domain>`).
- An image already pushed to ECR with the tag set in `group_vars/all.yml` (default `latest`).
- Ansible 2.15+ on your laptop (`pipx install ansible-core` or `brew install ansible`).
- `community.general` collection: `ansible-galaxy collection install community.general`.

## First-time setup

```sh
cd deploy/ansible
cp inventory.example.ini inventory.ini
cp group_vars/all.yml.example group_vars/all.yml
# Edit both files — fill in EIP, domain, ECR account id, email
```

Push your image to ECR (from your laptop, with Docker logged in):

```sh
ECR=$(cd ../terraform && terraform output -raw ecr_repository_url)
aws ecr get-login-password --region ap-northeast-2 --profile personal \
  | docker login --username AWS --password-stdin "${ECR%/*}"
# Use ${ECR} (not $ECR) before a colon — zsh treats ":l" as a lowercase modifier
docker tag  web_server:latest "${ECR}:latest"
docker push "${ECR}:latest"
```

## Run

```sh
# Dry run first — no changes, just diff
ansible-playbook playbook.yml --check --diff

# Apply
ansible-playbook playbook.yml
```

The first run does the full setup including issuing a real Let's Encrypt cert.
**Keep `letsencrypt_staging: true` in `group_vars/all.yml` until everything looks right** — staging certs don't count against the prod rate limit (5 issuances per domain per week). Once the staging cert appears, flip to `false` and re-run; the role will detect the staging cert, you'll need to delete it first:

```sh
ssh ubuntu@<EIP>
sudo certbot delete --cert-name <your-domain>
exit
ansible-playbook playbook.yml
```

## After deploy

```sh
curl -I https://<your-domain>
# Expect: HTTP/2 200, plus security headers (HSTS, X-Frame-Options, ...)
```

To deploy a new image:

```sh
docker push "$ECR:latest"
ansible-playbook playbook.yml --tags app    # role tags not wired yet — runs full playbook for now
# or just rerun: pull will report `changed` and the systemd unit restarts the container.
```

## Roles

| Role     | Does |
|----------|------|
| common   | timezone, apt upgrade, unattended-upgrades, ufw (22/80/443) |
| docker   | Docker Engine + amazon-ecr-credential-helper (auto ECR auth) |
| nginx    | reverse proxy to `127.0.0.1:8080`, ACME http-01 challenge path, HTTPS block enabled once cert exists |
| certbot  | issues Let's Encrypt cert (webroot mode), enables `certbot.timer` for auto-renewal |
| app      | `docker pull` the ECR image, install/enable systemd unit, restart on image change |
