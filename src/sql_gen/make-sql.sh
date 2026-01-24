

docker run --name postgres -e POSTGRES_PASSWORD=secret -p 5432:5432 -d postgres

echo $1
docker cp $1 postgres:/$1


docker exec -it postgres psql -U postgres -d postgres -f /$1

cargo install sql-gen

sql-gen \
    --db-url postgres://postgres:secret@localhost:5432/postgres \
    --output src/models/



