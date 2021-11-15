#!/bin/bash

set -euxo pipefail

# remove the controlplane
kubectl delete istiooperators.install.istio.io -n istio-system gitpod-services-controlplane

# uninstall the operator 
istioctl operator remove

# remove potential leftovers
istioctl manifest generate | kubectl delete -f -
kubectl delete ns istio-system --grace-period=0 --force