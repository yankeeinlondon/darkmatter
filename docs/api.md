# Darkmatter API

Darkmatter provides both a REST API and a Websocket message passing model that exposes the same functional footprint. Use the API that best suits your needs.

## REST API

### Projects

- `GET projects` - lists all projects which Darkmatter is aware of
- `GET projects/active` - lists projects which are considered "active" currently
- `GET projects/inactive` - lists projects which are _not_ considered "active" currently

### Cache

- `GET cache` - high level stats of cache state
- `GET cache/${project_id}` - high level stats of cache state for a particular project
- `PUT cache/${project id}/${doc_id}`

### Hooks

- `GET hooks` - lists all registered callback hooks; hooks are sorted first by project and [event name](). Note: setting _which_ hooks a project is interested in is done via configuration passed in during initialization not as a dynamic API call.

### Project Activation

- `POST activate/${project id}/true` - elevates a given project/repo to an "active" state
- `POST activate/${project id}/false` - deactivates a given project/repo

### Release Management

- `POST release` - perform a _transient_ release which produces all key metrics from a release but with no expectation of the work product becoming a new release.
- `POST release/x.y.z` - performs a _formal_ release which produces key metrics for review of a CI/CD pipeline (or person)
- `POST release/x.y.z/completed` - finalizes a formal release in the review stage
- `POST release/x.y.z/cancelled` - finalizes a formal release as having been cancelled; a reason can be supplied in the body of the message
