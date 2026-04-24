use file_ops::append_to_file;
use rand::RngExt;
pub fn add_git_acctions(path: &std::path::PathBuf, proj_name: &str) -> Result<(), std::io::Error> {
    println!("dockerhub token is in /home/mgs/.docker/config.json");
    let gh = r#"
// set up git repo

git inte 

// then add and push

git branch -M master main

gh repo create

gh secret set DOCKERHUB_USERNAME --body "maxthemerman"
gh secret set VPS_IP              --body "your.server.ip"
gh secret set VPS_USER            --body "root"
gh secret set VPS_SSH_KEY         --body "$(cat ~/.ssh/id_ed25519)"

// to get token, cd into .docker
cat config.json
// copy the auth string (just insid the quotes)
echo "..." | base64 -d
// use interactive prompt to avoid pasting stuff in bash history
// DONT USE THE `docker-usernme:` part. just the `dckr_pat_....` part
gh secret set DOCKERHUB_TOKEN
"#;

    println!("{}", gh);
    println!("name of proj is: {}", proj_name);
    let deploy_file = format!(
        r###"

 name: CI/CD Pipeline

 on:
   push:
     branches: [ "main" ] # Trigger on every push to the main branch

 jobs:
   # 1. Build and Push Job
   build-and-push:
     runs-on: ubuntu-latest
     strategy:
       matrix:
         include:
           - service: {proj_name}_frontend
             context: ./frontend      # Path to your frontend folder
           - service: {proj_name}_backend
             context: .               # Path to your backend folder
           - service: {proj_name}_python
             context: ./fastapi-template
     steps:
       - name: Checkout Code
         uses: actions/checkout@v4

       - name: Login to Docker Hub
         uses: docker/login-action@v3
         with:
           username: ${{ secrets.DOCKERHUB_USERNAME }}
           password: ${{ secrets.DOCKERHUB_TOKEN }}

       - name: Build and Push ${{ matrix.service }}
         uses: docker/build-push-action@v5
         with:
           context: ${{ matrix.context }}
           push: true
           # Tags the image as: username/frontend:latest and username/backend:latest
           tags: ${{ secrets.DOCKERHUB_USERNAME }}/${{ matrix.service }}:latest

   # 2. Deployment Job
   deploy:
     needs: build-and-push # Only run if the builds succeed
     runs-on: ubuntu-latest
     steps:
       - name: Checkout Code
         uses: actions/checkout@v4

       - name: Copy Config to Server
         uses: appleboy/scp-action@v0.1.7
         with:
           host: ${{ secrets.VPS_IP}}
           username: ${{ secrets.VPS_USER }}
           key: ${{ secrets.VPS_SSH_KEY }}
           source: "prod.yaml" # File to copy
           target: "~/{proj_name}/" # Folder on your server

       - name: SSH and Restart Containers
         uses: appleboy/ssh-action@v1.0.3
         with:
           host: ${{ secrets.VPS_IP}}
           username: ${{ secrets.VPS_USER}}
           key: ${{ secrets.VPS_SSH_KEY }}
           script: |
             # Navigate to the project folder
             cd ~/{proj_name}

             # Pull the latest images pushed in the previous job
             docker compose -f prod.yaml pull

             # Restart the services in detached mode
             docker compose -f prod.yaml up -d

             # (Optional) Clean up unused images to save disk space
             docker image prune -f
    "###
    );
    match std::fs::create_dir_all(path) {
        Ok(_) => println!("created dir"),
        Err(e) => println!("an error when making dir: {}", e),
    }
    //TODO: does not create folders
    // command spone + mkdir would be cheep quick option
    std::fs::create_dir_all(".github/workflows");
    append_to_file(&path.join(".github/workflows/deploy.yml"), &deploy_file);
    let mut rng = rand::rng();
    let port = rng.random_range(10000..=99999);
    let prod_file = format!(
        r###"
services:
  db:
    image: postgres:15-alpine
    container_name: {proj_name}_db
    environment:
      POSTGRES_USER: ${{POSTGRES_USER:-dbuser}}
      POSTGRES_PASSWORD: ${{POSTGRES_PASSWORD:?set POSTGRES_PASSWORD}}
      POSTGRES_DB: ${{POSTGRES_DB:-data}}
    volumes:
      - bens_chat2_postgres_data:/var/lib/postgresql/data
    healthcheck:
      test: ["CMD-SHELL", "pg_isready -U ${{POSTGRES_USER:-dbuser}} -d ${{POSTGRES_DB:-data}}"]
      interval: 5s
      timeout: 5s
      retries: 10

  minio:
    image: minio/minio:latest
    container_name: {proj_name}_minio
    command: server /data --console-address ":9001"
    environment:
      MINIO_ROOT_USER: ${{MINIO_ROOT_USER:-minioadmin}}
      MINIO_ROOT_PASSWORD: ${{MINIO_ROOT_PASSWORD:-minioadmin}}
    ports:
      - "9000:9000"
      - "9001:9001"
    volumes:
      - bens_chat2_minio_data:/data
    healthcheck:
      test: ["CMD", "curl", "-f", "http://localhost:9000/minio/health/live"]
      interval: 30s
      timeout: 20s
      retries: 3

  createbuckets:
    image: minio/mc
    container_name: {proj_name}_createbuckets
    depends_on:
      - minio
    volumes:
      - ./minio-cors.json:/minio-cors.json
    entrypoint: >
      /bin/sh -c "
      sleep 10;
        /usr/bin/mc alias set myminio http://minio:9000 ${{MINIO_ROOT_USER:-minioadmin}} ${{MINIO_ROOT_PASSWORD:-minioadmin}} || exit 1;
        /usr/bin/mc mb myminio/bucket || true;
        /usr/bin/mc anonymous set public myminio/bucket || exit 1;
        /usr/bin/mc anonymous set-json /minio-cors.json myminio/bucket || true;
        echo 'Bucket creation completed successfully';
      exit 0;
      "

  app:
    image: maxthemerman/{proj_name}-app:latest
    container_name: {proj_name}_app
    ports:
      - "{port}:8081"
    environment:
      DATABASE_URL: postgres://${{POSTGRES_USER:-dbuser}}:${{POSTGRES_PASSWORD:?set POSTGRES_PASSWORD}}@db:5432/${{POSTGRES_DB:-data}}
      DATABASE_CONNECT_TIMEOUT: "30"
      JWT_SECRET: ${{JWT_SECRET:?set JWT_SECRET}}
      JWT_EXP_HOURS: ${{JWT_EXP_HOURS:-24}}
      CORS_ALLOWED_ORIGINS: ${{CORS_ALLOWED_ORIGINS:?set CORS_ALLOWED_ORIGINS}}
      SQLX_OFFLINE: "true"
      MINIO_ACCESS_KEY: ${{MINIO_ROOT_USER:-minioadmin}}
      MINIO_SECRET_KEY: ${{MINIO_ROOT_PASSWORD:?set MINIO_ROOT_PASSWORD}}
      MINIO_ENDPOINT: "minio:9000"
      MINIO_PUBLIC_ENDPOINT: ${{MINIO_PUBLIC_ENDPOINT:-localhost:9000}}
      MINIO_SECURE: ${{MINIO_SECURE:-false}}
    depends_on:
      db:
        condition: service_healthy
    restart: on-failure
    healthcheck:
      test: ["CMD", "wget", "--spider", "http://localhost:8081/health"]
      interval: 10s
      timeout: 5s
      retries: 5
      start_period: 30s

volumes:
  {proj_name}_postgres_data:
  {proj_name}_minio_data:

    "###
    );
    append_to_file(&path.join("prod.yaml"), &prod_file);
    let dockerignor = r###"

    # Ignore build artifacts
target/
**/node_modules/

# Ignore source control
.git
.github

# Ignore local environment files
.env
*.log

# Ignore other service folders not needed for this build context
fastapi-template/

# Ignore temporary files
**/.DS_Store
**/Thumbs.db

# Ignore Docker related files
Dockerfile
docker-compose.yaml
prod.yaml
""###;
    append_to_file(&path.join(".dockerignore"), dockerignor)
}
