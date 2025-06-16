include .env


init-db:
	mkdir mydata # Create a directory to store the database, owned by the current user
	docker run --rm --pull always -p 8000:8000 --user $(id -u) -v $(pwd)/mydata:/mydata surrealdb/surrealdb:latest start rocksdb:/mydata/mydatabase.db

help:
	docker run --rm --pull always surrealdb/surrealdb:latest help
