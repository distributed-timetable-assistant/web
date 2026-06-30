# ADR-0001: Establish the Angular Workspace Structure

- **Status:** Accepted
- **Date:** 2026-07-01

## Context

The DiTA frontend architecture has already been established in the project-level ADR selecting Angular as the frontend framework.

This ADR documents the concrete implementation decisions for the `web` repository, including the workspace layout, tooling, project organization, and the commands used to bootstrap the repository. Recording these decisions ensures that future contributors can reproduce the workspace consistently and understand why the repository is organized the way it is.

## Decision

The `web` repository SHALL be implemented as an integrated Nx monorepo using pnpm.

At the time this workspace was created, the following tool versions were used:

| Tool | Version |
|------|---------|
| Nx | **23.0.1** |
| pnpm | **11.0.3** |

The repository SHALL follow the following structure:

- All Angular applications SHALL be located under `apps/`.
- All reusable libraries SHALL be located under `libs/`.

This layout establishes a clear separation between deployable applications and reusable components, allowing the repository to scale as additional frontend applications are introduced.

## Workspace Bootstrap

The workspace was created using the following command:

```bash
pnpm dlx create-nx-workspace@latest web \
  --preset=apps \
  --workspaceType=integrated \
  --packageManager=pnpm \
  --nxCloud=skip \
  --interactive=false \
  --skipGit
```

## Angular Installation

Angular support was added to the workspace using:

```bash
pnpm add -D @nx/angular
```

## Initial Application

The first Angular application, `landing-selection`, was created using:

```bash
pnpm nx g @nx/angular:app \
  --name=landing-selection \
  --directory=apps/landing-selection \
  --style=scss \
  --routing \
  --bundler=esbuild \
  --e2eTestRunner=playwright \
  --unitTestRunner=vitest-angular \
  --ssr=false
```

## Initial Library

The first shared library, `design-system`, was created using:

```bash
pnpm nx g @nx/angular:lib \
  --name=design-system \
  --directory=libs/design-system \
  --style=scss \
  --unitTestRunner=jest
```

## Finalization

After generating the initial workspace contents, the following commands were executed:

```bash
pnpm approve-builds
pnpm install
```

## Consequences

### Positive

- Establishes a reproducible and documented workspace creation process.
- Provides a consistent directory structure for all frontend applications and shared libraries.
- Enables efficient code sharing through reusable libraries.
- Leverages Nx tooling for scalable monorepo management.
- Uses pnpm for fast, disk-efficient dependency management.

### Negative

- Future upgrades of Nx or pnpm may require updates to the documented bootstrap commands.
- Contributors are expected to be familiar with Nx workspace conventions.

## References

- Project ADR selecting Angular as the frontend technology.
- Nx Workspace documentation.
- pnpm documentation.
