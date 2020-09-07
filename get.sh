#!/usr/bin/env bash
# Pulls the complete list of county elections auditors from the Secretary of State site for the great state of Iowa.
# Go Hawkeyes!
# source: https://sos.iowa.gov
set -e

res=$(curl \
    --cookie-jar cookies.txt \
    -sL \
    -o data/complete-list.html.gz \
    -H @headers.txt \
    -w "%{http_code}" \
    "https://sos.iowa.gov/elections/auditors/auditor.asp?CountyID=00")

if [[ $res -gt 299 ]]; then
    printf "Error: got non-success status from GET; HTTP %d\n" $res
    exit 1
fi

# check if they returned gzipped -- they should have but you never know with these web sites...
file data/complete-list.html.gz | grep gzip
if [[ $? -eq 0 ]]; then
    gunzip -c data/complete-list.html.gz > data/complete-list.html
fi
