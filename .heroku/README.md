# Heroku

## Caveats

### Deployments to Heroku does not support `scratch` images

Workaround:

- Deploy a container image built from `alpine` ([Dockerfile](Dockerfile)) that is as identical as possible to the container image build from `scratch` ([Dockerfile](../Dockerfile)).

### Deployments to Heroku randomizes port on startup

Workaround:

- Deploy a container image built with a `CMD` ([Dockerfile](Dockerfile)) that remaps environment variable `PORT` to `CRG__HTTP_SERVER__PORT`.
