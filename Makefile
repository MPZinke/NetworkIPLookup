

build:
	docker build -t network_lookup \
	  --build-arg NETWORKLOOKUP_BEARERTOKEN="${NETWORKLOOKUP_BEARERTOKEN}" \
	  --build-arg NETWORKLOOKUP_BEARERTOKEN="${NETWORKLOOKUP_BEARERTOKEN}" \
	  --build-arg NETWORKLOOKUP_ROUTER_DOMAIN="${NETWORKLOOKUP_ROUTER_DOMAIN}" \
	  --build-arg NETWORKLOOKUP_DB_USER="${NETWORKLOOKUP_DB_USER}" \
	  --build-arg NETWORKLOOKUP_DB_PASSWORD="${NETWORKLOOKUP_DB_PASSWORD}" \
	 .


run:
	# https://stackoverflow.com/a/49907758
	docker run --add-host="host.docker.internal:host-gateway" -p 8000:8081 network_lookup


clean:
	docker rmi `docker images --filter dangling=true -q` --force


kill:
	docker stop `docker ps -q`
