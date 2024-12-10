# Build stage
FROM node:alpine as build

WORKDIR /app
COPY package*.json ./
RUN npm install -g pnpm
RUN pnpm install
COPY . .
RUN pnpm build

# Production stage
FROM nginx:alpine-slim

# Copy built assets from build stage
COPY --from=build /app/dist /usr/share/nginx/html

# Start Nginx
CMD ["nginx", "-g", "daemon off;"]
