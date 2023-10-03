<div align="center">
   <h1>SellersHut API: Ads</h1>
   <div>
     <img alt="GitHub Workflow Status (with event)" src="https://img.shields.io/github/actions/workflow/status/sellershut/api-ads/test.yaml?label=tests">
     <a href="https://codecov.io/gh/sellershut/api-ads" > 
       <img src="https://codecov.io/gh/sellershut/api-ads/graph/badge.svg"/> 
     </a>
     <img alt="GitHub" src="https://img.shields.io/github/license/sellershut/api-ads"/>
   </div>
</div>

## Features

- #### GraphQL
	- Queries, mutations, subscriptions
- #### Distributed Tracing w/ OpenTelemetry
	- Sentry integration for `error` level logs

## Building

#### Sentry

You can self-host Sentry, follow their [Developer Documentation](https://develop.sentry.dev/self-hosted/) on how to get started. You need the `SENTRY_DSN` variable in your environment. 

#### Opentelemetry

Follow [Jaeger's](https://www.jaegertracing.io/) documentation on [deployment](https://www.jaegertracing.io/docs/1.49/deployment/) and set `OTLP_COLLECTOR` to where your collector is hosted.

#### Redis

The API uses caching through the Redis SDK:
```sh
docker run --network=host --ulimit memlock=-1 docker.dragonflydb.io/dragonflydb/dragonfly
```
> [!NOTE]  
> [Dragonfly](https://www.dragonflydb.io) works as well since it's a drop-in replacement!

Lastly, check your [environment](.env.example) and make sure everything is populated accordingly
