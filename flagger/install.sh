#!/bin/bash

set -euxo pipefail

helm repo add flagger https://flagger.app
kubectl apply -f https://raw.githubusercontent.com/fluxcd/flagger/main/artifacts/flagger/crd.yaml

helm upgrade -i flagger flagger/flagger \
    --namespace=istio-system \
    --set crd.create=false \
    --set meshProvider=istio \
    --set metricsServer=http://prometheus.istio-system:9090

[ "$(kubectl get ns test)" = "1" ] && kubectl create namespace test
helm upgrade -i flagger-loadtester flagger/loadtester \
    --namespace=test \
    --set cmd.timeout=1h