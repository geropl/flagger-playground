# flagger-playground

## build
```bash
./workload/build.sh
docker push geropl/responder:latest
```

## deploy
```bash
kubectl apply -f workload/deployment.sh
kubectl apply -f workload/service.sh
```