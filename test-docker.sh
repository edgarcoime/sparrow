#!/bin/bash

# Test script for Docker setup

echo "🐳 Testing Docker setup for Sparrow 2022..."

echo "📦 Building Docker image..."
docker-compose build

if [ $? -eq 0 ]; then
    echo "✅ Build successful!"
    
    echo "🚀 Starting container..."
    docker-compose up -d
    
    echo "⏳ Waiting for container to start..."
    sleep 5
    
    echo "🌐 Testing application..."
    if curl -s http://localhost:8080 | grep -q "Shorelark"; then
        echo "✅ Application is running correctly!"
        echo "🌍 Open http://localhost:8080 in your browser"
    else
        echo "❌ Application may not be working correctly"
        echo "📋 Container logs:"
        docker-compose logs
    fi
    
    echo ""
    echo "🛑 To stop the container, run: docker-compose down"
else
    echo "❌ Build failed!"
    exit 1
fi
