#!/bin/bash

set -euxo pipefail

# install the operator: https://istio.io/latest/docs/setup/install/operator/
istioctl operator init

# create a mesh with 'default' profile (as specified here: https://docs.flagger.app/install/flagger-install-on-kubernetes#install-flagger-with-helm)
kubectl apply -f - <<EOF
apiVersion: install.istio.io/v1alpha1
kind: IstioOperator
metadata:
  namespace: istio-system
  name: gitpod-services-controlplane
spec:
  profile: default
EOF

# auto-inject proxy contains into all new pods
kubectl label namespace default istio-injection=enabled --overwrite

# install prometheus
kubectl apply -f https://raw.githubusercontent.com/istio/istio/release-1.11/samples/addons/prometheus.yaml
