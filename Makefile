include .env


init-db:
	mkdir mydata # Create a directory to store the database, owned by the current user
	docker run --rm --pull always -p 8000:8000 --user $(id -u) -v $(pwd)/mydata:/mydata surrealdb/surrealdb:latest start rocksdb:/mydata/mydatabase.db

help:
	docker run --rm --pull always surrealdb/surrealdb:latest help



# Docker
auth:
	aws ecr get-login-password --region us-east-1 | docker login --username AWS --password-stdin $(DOCKER_REGISTRY)

create-repo:
	aws ecr create-repository --repository-name $(IMAGE) --region us-east-1 || true

docker-prepare:
	cd frontend && cargo build --target wasm32-unknown-unknown
	cp frontend/pkg/* backend/static/pkg/

docker: docker-prepare
	docker build -t $(DOCKER_REGISTRY)/$(IMAGE):$(VERSION) -f ./backend/Dockerfile ./backend
	docker push $(DOCKER_REGISTRY)/$(IMAGE):$(VERSION)


# Kubernetes and Helm
k8s-init:
	kubectl create namespace $(NAMESPACE)

k8s-auth:
	kubectl create secret docker-registry ecr-secret --docker-server=$(DOCKER_REGISTRY) --docker-username=AWS --docker-password=$(DOCKER_PASSWORD) --namespace=$(NAMESPACE)

k8s-deploy:
	kubectl create namespace $(NAMESPACE) || true
	helm upgrade --install $(NAMESPACE) ./k8s --namespace $(NAMESPACE) --set surrealdb.secret.user=$(SURREALDB_USER) --set surrealdb.secret.pass=$(SURREALDB_PASS) -f ./k8s/values.yaml


k8s-debug:
	kubectl create namespace $(NAMESPACE) --dry-run=client -o yaml | kubectl apply -f -
	helm template $(NAMESPACE) ./k8s -f ./k8s/values.yaml | kubectl apply --namespace $(NAMESPACE) -f - --dry-run=server
