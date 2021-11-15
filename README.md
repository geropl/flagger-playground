# flagger-playground


## connect
```
BROWSER="" gcloud auth login
gcloud container clusters get-credentials cluster --zone europe-west1-b --project gpl-flagger
```

## deploy new version
connect and:
```bash
kubectl edit deployment # increase "VERSION" env var, for instance
```

wait for canary being "promoted":
```bash
kubectl wait canary/responder --for=condition=promoted
```

## reproduce

```bash
./istio/install.sh
./flagger/install.sh
kubectl apply -f workload/deployment.sh
kubectl apply -f flagger/canary-responder.sh
```

## build
```bash
./workload/build.sh
docker push geropl/responder:latest
```
