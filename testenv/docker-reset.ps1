Write-Host "Stopping container..."
docker-compose down --volumes
Write-Host "Restart container..."
docker-compose up -d