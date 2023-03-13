# oponn

## GOAL:
Build a template for an app that can be loaded into an AWS EC2 instance via code-build.

The app should do the following:
1. Pull data from an outside api and load it into a struct
2. Upload data from struct to an outside datastore (Atlas-mongodb initially)
3. Finally, serve the data from the struct/mongodb via a GraphQL API

4. The above app should come with a basic AWS infrastructure for an EC2 instance
in an ECS connected to a load-balancer. The app should auto-deploy from github
via codepipeline on every merge to main branch.

5. Include a perf analysis (load-testing) for the app.