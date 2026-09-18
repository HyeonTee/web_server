#!/usr/bin/env bash
# Publish static/ to hyeontae.gwinam.com (S3 + CloudFront in the prod account).
#
# Infra lives in github.com/HyeonTee/aws, stack prod/homepage. This script only
# syncs files and invalidates the CDN cache.
#
# Usage:
#   local:  DISTRIBUTION_ID=... deploy/static/deploy.sh
#           (uses AWS profile "personal" and hops into prod via OrganizationAccountAccessRole)
#   CI:     credentials already belong to the prod account (GitHub OIDC role); no hop.
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/../.." && pwd)"
SRC="$ROOT/static"

PROD_ACCOUNT_ID="${PROD_ACCOUNT_ID:-071026217169}"
BUCKET="${BUCKET:-hyeontae-gwinam-com-071026217169}"
DISTRIBUTION_ID="${DISTRIBUTION_ID:?set DISTRIBUTION_ID (tofu -chdir=aws/prod/homepage output -raw distribution_id)}"

# Use whatever credentials are already present (CI) unless a profile is wanted.
if [ -z "${AWS_ACCESS_KEY_ID:-}" ]; then
  export AWS_PROFILE="${AWS_PROFILE:-personal}"
fi

current_account=$(aws sts get-caller-identity --query Account --output text)
if [ "$current_account" != "$PROD_ACCOUNT_ID" ]; then
  echo "==> assume OrganizationAccountAccessRole in $PROD_ACCOUNT_ID (from $current_account)"
  creds=$(aws sts assume-role \
    --role-arn "arn:aws:iam::${PROD_ACCOUNT_ID}:role/OrganizationAccountAccessRole" \
    --role-session-name homepage-deploy \
    --query 'Credentials.[AccessKeyId,SecretAccessKey,SessionToken]' --output text)
  export AWS_ACCESS_KEY_ID; AWS_ACCESS_KEY_ID=$(cut -f1 <<<"$creds")
  export AWS_SECRET_ACCESS_KEY; AWS_SECRET_ACCESS_KEY=$(cut -f2 <<<"$creds")
  export AWS_SESSION_TOKEN; AWS_SESSION_TOKEN=$(cut -f3 <<<"$creds")
  unset AWS_PROFILE
fi

echo "==> sync $SRC -> s3://$BUCKET"
# Bucket layout mirrors URL paths: /index.html, /about.html, /static/css/...
aws s3 sync "$SRC" "s3://$BUCKET/static" --delete --exclude '.DS_Store' --exclude '*.html' \
  --cache-control 'public, max-age=31536000, immutable'
aws s3 sync "$SRC" "s3://$BUCKET/" --delete --exclude '*' --include '*.html' \
  --cache-control 'public, max-age=300' --content-type 'text/html; charset=utf-8'

echo "==> invalidate CloudFront $DISTRIBUTION_ID"
aws cloudfront create-invalidation --distribution-id "$DISTRIBUTION_ID" --paths '/*' \
  --query 'Invalidation.[Id,Status]' --output text

echo "==> done: https://hyeontae.gwinam.com/"
