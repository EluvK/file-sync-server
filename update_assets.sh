#!/bin/bash

# 生成metadata部分
metadata=$(
  for file in ./assets/*; do
    filename=$(basename "$file")
    hash=$(md5sum "$file" | awk '{print $1}')
    echo "{\"name\": \"$filename\", \"hash\": \"$hash\"},"
  done | sed '$ s/,$//'
)

# 生成完整的config.json
cat <<EOF > config.json
{
    "cert": "./cert/certificate.crt",
    "key": "./cert/private.key",
    "metadata": [
$metadata
    ]
}
EOF
