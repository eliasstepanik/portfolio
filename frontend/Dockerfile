# Use pre-built frontend files
FROM nginx:alpine

# Copy pre-built files to nginx
COPY dist /usr/share/nginx/html

# Copy nginx configuration
COPY nginx.conf /etc/nginx/nginx.conf

EXPOSE 80

CMD ["nginx", "-g", "daemon off;"]