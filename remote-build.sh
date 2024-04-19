#!/bin/bash

# kubectl config set-context --current --namespace=default
# kubectl apply -n default -f rust-dev.yaml
# kubectl exec --stdin --tty rust-dev -- /bin/bash

# kubectl run rust-dev --image=rust --command --attach -i --tty
# # /bin/bash
# kubectl attach rust-dev -c rust-dev -i --tty


if [[ ! $(kubectl get po | grep -e "rust-dev.*Running") ]];
then
    kubectl run -n default rust-dev --image=rust --command -- sleep infinity
    kubectl wait -n default --for=condition=Ready pod/rust-dev
fi

# kubectl exec -n default -it rust-dev -- /bin/bash
kubectl exec rust-dev -i -- bash < test-script.sh

kubectl delete pod rust-dev







apt-get update
apt-get install vim
apt-get install iproute2 # for ss command



~/rpk -X brokers=redpanda-0.redpanda.rpanda.svc.cluster.local.:9093 -X tls.enabled=true -X tls.insecure_skip_verify=true topic list