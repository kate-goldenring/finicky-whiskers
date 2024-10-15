FROM --platform=${BUILDPLATFORM} ubuntu AS builder

COPY . /src

FROM scratch

COPY --from=builder /src/components /components
COPY --from=builder /src/site /site
COPY --from=builder /src/spin.toml /spin.toml
