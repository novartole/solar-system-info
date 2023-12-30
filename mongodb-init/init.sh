#!/bin/bash

cd /docker-entrypoint-initdb.d/collections \
  && ls -1 *.json | sed 's/.json$//' | while read col; do
    mongoimport \
      --db "$MONGO_INITDB_DATABASE" \
      --collection "$col" \
      --drop \
      --jsonArray \
      --file "$col.json"
  done
