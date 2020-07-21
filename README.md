# fan-quiz-service ![tests](https://github.com/nemesiscodex/actix-blog-app/workflows/tests/badge.svg)
App service for fan quiz bands, questions & answers

## Requirements
- Rust
- Docker
- docker-compose    
 
## Usage
```   
# Copy example .env file    
cp .env.example .env  
   
# Run postgres  
docker-compose up -d postgres  
 
# Install diesel
cargo install diesel_cli --no-default-features --features postgres
  
# Run db migrations
DATABASE_URL=postgres://actix:actix@localhost:5432/actix diesel migration run

# to redo migration (apply down script, then up script) do: diesel migration redo

# Install LLVM/Clang compiler
# https://github.com/bcmyers/argonautica/tree/master/argonautica-rs#installation

# Run unit tests
cargo test

# Run the server (Add --release for an optimized build)
cargo run 

# access the db from command line:
psql postgres://actix:actix@localhost:5432/actix
```
#### Test query:
```
{
  users {
    id
    username
    email
    bio 
    image 
    createdAt
    updatedAt
  }
}
```
Or with curl
```
curl -X POST -H "Content-Type: application/json" -d '{ "query": "{users {id username email bio image createdAt updatedAt}}" }' https://actix-blog-app.herokuapp.com/graphql -s | jq .
```
#### Will get you:
```
{
  "data": {
    "users": [
      {
        "id": "11c21a2b-e131-4b76-b32a-1872790defdb",
        "username": "user1",
        "email": "user1@example.com",
        "bio": null,
        "image": null,
        "createdAt": 1584256602,
        "updatedAt": 1584256602
      }
    ]
  }
}
```

<<<<<<< HEAD
# build docker image
docker build --tag {name_of_build}:{sem.ver} .

# run docker image
docker run --publish {port}:{port} --detach --name {abbreviation} {name_of_build}:{sem.ver}

# stop
docker rm --force {abbreviation}

# start kubernetes:
minikube start --driver=hyperkit

# deploy app
kubectl create deployment kubernetes-bootcamp --image=gcr.io/google-samples/kubernetes-bootcamp:v1

# run local registry server
docker run -d -p 5000:5000 --restart=always --name registry registry:2


#### run this current 'rust' docker image
$ docker build -t my-rust-app .
$ docker run -it --rm --name my-running-app my-rust-app

