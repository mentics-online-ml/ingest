#!/bin/bash

kubectl config set-context --current --namespace=default
# kubectl apply -n default -f rust-dev.yaml
# kubectl exec --stdin --tty rust-dev -- /bin/bash

kubectl run rust-dev --image=rust --command --attach -i --tty
# /bin/bash
kubectl attach rust-dev -c rust-dev -i --tty


kubectl


if [[ ! $(kubectl get po | grep -e "rust-dev.*Running") ]];
then
    kubectl run rust-dev --image=rust --command -- sleep infinity
    kubectl wait --for=condition=Ready pod/rust-dev
fi

# kubectl exec -it rust-dev -- /bin/bash
kubectl exec rust-dev -i -- bash < test-script.sh

kubectl delete pod rust-dev