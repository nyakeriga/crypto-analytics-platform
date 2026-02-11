# DevOps

Infrastructure automation: Docker, Kubernetes, Terraform, CI/CD.

## Example Dockerfile
```dockerfile
FROM node:18
WORKDIR /app
COPY . .
RUN npm install
CMD ["npm", "start"]
```
