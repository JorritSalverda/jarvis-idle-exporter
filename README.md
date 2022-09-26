## Installation

To install this application using Helm run the following commands: 

```bash
helm repo add jorritsalverda https://helm.jorritsalverda.com
kubectl create namespace jarvis-idle-exporter

helm upgrade \
  jarvis-idle-exporter \
  jorritsalverda/jarvis-idle-exporter \
  --install \
  --namespace jarvis-idle-exporter \
  --set secret.gcpServiceAccountKeyfile='{abc: blabla}' \
  --wait
```
